use std::ops::Deref;

use crate::transcode::{
    CairoWrite, Transcode, TranscodeError, TranscodeResult, TranscodeSerializeResult,
    TranscodeWriter,
};
use crate::{
    ArrayDef, CairoDeserializer, ColumnDef, DecodeError, EnumDef, FixedArrayDef, NullableDef,
    OptionDef, ResultDef, StructDef, TupleDef, TypeDef,
};

impl<T: CairoWrite + ?Sized> CairoWrite for &mut T {
    type Error = T::Error;
    type Ok = T::Ok;
    fn write_byte(&mut self, byte: u8) -> Result<Self::Ok, Self::Error> {
        (**self).write_byte(byte)
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> Result<Self::Ok, Self::Error> {
        (**self).write_bytes(bytes)
    }

    fn write_felt(
        &mut self,
        felt: starknet_types_core::felt::Felt,
    ) -> Result<Self::Ok, Self::Error> {
        (**self).write_felt(felt)
    }
}

impl<T, S, In, Out> Transcode<In, Out> for T
where
    T: Deref<Target = [S]>,
    S: Transcode<In, Out>,
{
    type SerializeError = S::SerializeError;
    type DeserializeError = S::DeserializeError;
    fn transcode(
        &self,
        input: &mut In,
        output: &mut Out,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        for element in self.deref() {
            element.transcode(input, output)?;
        }
        Ok(())
    }
}

impl<T, In, Out> Transcode<In, Out> for [&T]
where
    T: Transcode<In, Out>,
{
    type SerializeError = T::SerializeError;
    type DeserializeError = T::DeserializeError;
    fn transcode(
        &self,
        input: &mut In,
        output: &mut Out,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        for element in self {
            element.transcode(input, output)?;
        }
        Ok(())
    }
}

impl<In, Out> Transcode<In, Out> for TypeDef
where
    In: CairoDeserializer,
    Out: CairoWrite,
{
    type SerializeError = <Out as TranscodeWriter<In>>::SerializeError;
    type DeserializeError = DecodeError;

    fn transcode(
        &self,
        input: &mut In,
        output: &mut Out,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        match self {
            TypeDef::None => Ok(()),
            TypeDef::Felt252
            | TypeDef::ClassHash
            | TypeDef::ContractAddress
            | TypeDef::StorageAddress
            | TypeDef::StorageBaseAddress => output.transcode_felt(input),
            TypeDef::ShortUtf8 | TypeDef::Bytes31 | TypeDef::Bytes31Encoded(_) => {
                output.transcode_felt(input)
            }
            TypeDef::Bool => output.transcode_bytes::<1>(input),
            TypeDef::U8 => output.transcode_bytes::<1>(input),
            TypeDef::U16 => output.transcode_bytes::<2>(input),
            TypeDef::U32 => output.transcode_bytes::<4>(input),
            TypeDef::U64 => output.transcode_bytes::<8>(input),
            TypeDef::U128 => output.transcode_bytes::<16>(input),
            TypeDef::U256 => input
                .next_u256()
                .and_then_tc(|v| output.write_bytes(&v.to_big_endian())),
            TypeDef::U512 => input
                .next_u512()
                .and_then_tc(|v| output.write_bytes(&v.to_big_endian())),
            TypeDef::I8 => output.transcode_bytes::<1>(input),
            TypeDef::I16 => output.transcode_bytes::<2>(input),
            TypeDef::I32 => output.transcode_bytes::<4>(input),
            TypeDef::I64 => output.transcode_bytes::<8>(input),
            TypeDef::I128 => output.transcode_bytes::<16>(input),
            TypeDef::EthAddress => output.transcode_bytes::<20>(input),
            TypeDef::ByteArray
            | TypeDef::Utf8String
            | TypeDef::ByteArrayEncoded(_)
            | TypeDef::Custom(_) => input
                .next_byte_array_bytes()
                .and_then_tc(|v| output.write_variable_bytes(&v)),
            TypeDef::Tuple(tuple) => tuple.transcode(input, output),
            TypeDef::Array(a) => a.transcode(input, output),
            TypeDef::FixedArray(fa) => fa.transcode(input, output),
            TypeDef::Felt252Dict(_ty) => unimplemented!(), // TODO: implement Felt252Dict parsing
            TypeDef::Struct(s) => s.transcode(input, output),
            TypeDef::Enum(e) => e.transcode(input, output),
            TypeDef::Ref(_) => Err(TranscodeError::de(DecodeError::message(
                "TypeDef Ref needs to be expanded before transoding",
            ))),
            TypeDef::Option(option) => option.transcode(input, output),
            TypeDef::Result(result) => result.transcode(input, output),
            TypeDef::Nullable(nullable) => nullable.transcode(input, output),
        }
    }
}

impl<In, Out> Transcode<In, Out> for TupleDef
where
    In: CairoDeserializer,
    Out: CairoWrite,
{
    type SerializeError = <Out as TranscodeWriter<In>>::SerializeError;
    type DeserializeError = DecodeError;

    fn transcode(
        &self,
        input: &mut In,
        output: &mut Out,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        for element in &self.elements {
            element.transcode(input, &mut *output)?;
        }
        Ok(())
    }
}

impl<In, Out> Transcode<In, Out> for ArrayDef
where
    In: CairoDeserializer,
    Out: CairoWrite,
{
    type SerializeError = <Out as TranscodeWriter<In>>::SerializeError;
    type DeserializeError = DecodeError;

    fn transcode(
        &self,
        input: &mut In,
        output: &mut Out,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        let len = input.next_u32().map_de()?;
        output.write_bytes(&len.to_be_bytes()).map_se()?;
        for _ in 0..len {
            self.type_def.transcode(input, &mut *output)?;
        }
        Ok(())
    }
}

impl<In, Out> Transcode<In, Out> for FixedArrayDef
where
    In: CairoDeserializer,
    Out: CairoWrite,
{
    type SerializeError = <Out as TranscodeWriter<In>>::SerializeError;
    type DeserializeError = DecodeError;

    fn transcode(
        &self,
        input: &mut In,
        output: &mut Out,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        for _ in 0..self.size {
            self.type_def.transcode(input, &mut *output)?;
        }
        Ok(())
    }
}

impl<In, Out> Transcode<In, Out> for StructDef
where
    In: CairoDeserializer,
    Out: TranscodeWriter<In> + CairoWrite,
{
    type SerializeError = <Out as TranscodeWriter<In>>::SerializeError;
    type DeserializeError = DecodeError;

    fn transcode(
        &self,
        input: &mut In,
        output: &mut Out,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        for member in &self.members {
            member.type_def.transcode(input, &mut *output)?;
        }
        Ok(())
    }
}

impl<In, Out> Transcode<In, Out> for EnumDef
where
    In: CairoDeserializer,
    Out: CairoWrite,
{
    type SerializeError = <Out as TranscodeWriter<In>>::SerializeError;
    type DeserializeError = DecodeError;

    fn transcode(
        &self,
        input: &mut In,
        output: &mut Out,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        let selector = input.next_enum_variant().map_de()?;
        output.write_felt(selector).map_se()?;
        self.get_variant(&selector)
            .map_de()?
            .type_def
            .transcode(input, output)
    }
}

impl<In, Out> Transcode<In, Out> for OptionDef
where
    In: CairoDeserializer,
    Out: CairoWrite,
{
    type SerializeError = <Out as TranscodeWriter<In>>::SerializeError;
    type DeserializeError = DecodeError;

    fn transcode(
        &self,
        input: &mut In,
        output: &mut Out,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        let is_some = input.next_option_is_some().map_de()?;
        output.write_byte(is_some as u8).map_se()?;
        match is_some {
            true => self.type_def.transcode(input, output),
            false => Ok(()),
        }
    }
}

impl<In, Out> Transcode<In, Out> for ResultDef
where
    In: CairoDeserializer,
    Out: CairoWrite,
{
    type SerializeError = <Out as TranscodeWriter<In>>::SerializeError;
    type DeserializeError = DecodeError;

    fn transcode(
        &self,
        input: &mut In,
        output: &mut Out,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        let is_ok = input.next_result_is_ok().map_de()?;
        output.write_byte(!is_ok as u8).map_se()?;
        match is_ok {
            true => self.ok.transcode(input, output),
            false => self.err.transcode(input, output),
        }
    }
}

impl<In, Out> Transcode<In, Out> for NullableDef
where
    In: CairoDeserializer,
    Out: CairoWrite,
{
    type SerializeError = <Out as TranscodeWriter<In>>::SerializeError;
    type DeserializeError = DecodeError;

    fn transcode(
        &self,
        input: &mut In,
        output: &mut Out,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        let is_null = input.next_nullable_is_null().map_de()?;
        output.write_byte(!is_null as u8).map_se()?;
        match is_null {
            true => Ok(()),
            false => self.type_def.transcode(input, output),
        }
    }
}

impl<In, Out> Transcode<In, Out> for ColumnDef
where
    In: CairoDeserializer,
    Out: CairoWrite,
{
    type SerializeError = <Out as TranscodeWriter<In>>::SerializeError;
    type DeserializeError = DecodeError;

    fn transcode(
        &self,
        input: &mut In,
        output: &mut Out,
    ) -> Result<(), TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        self.type_def.transcode(input, output)
    }
}
