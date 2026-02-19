use std::cell::RefCell;

use starknet_types_core::felt::Felt;

use crate::transcode::{
    CairoSerializer, CairoWrite, Transcode, TranscodeError, TranscodeResult,
    TranscodeSerializeResult, TranscodeWriter,
};
use crate::{
    ArrayDef, CairoDeserializer, DecodeError, DecodeResult, EnumDef, FixedArrayDef, NullableDef,
    OptionDef, ResultDef, StructDef, TupleDef, TypeDef,
};

struct SchemaSerialize<'a, D> {
    ty: &'a TypeDef,
    de: &'a mut D,
}

impl<In, Out> Transcode<In, Out> for TypeDef
where
    In: CairoDeserializer,
    Out: CairoSerializer,
{
    type DeserializeError = DecodeError;
    type SerializeError = Out::Error;
    type Ok = Out::Ok;
    fn transcode(
        &self,
        input: &mut In,
        output: Out,
    ) -> Result<Self::Ok, TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        match self {
            TypeDef::None => output.serialize_unit().map_se(),
            TypeDef::Felt252
            | TypeDef::ClassHash
            | TypeDef::ContractAddress
            | TypeDef::StorageAddress
            | TypeDef::StorageBaseAddress => input
                .next_felt_bytes()
                .and_then_tc(|v| output.serialize_felt(v)),
            TypeDef::ShortUtf8 => input
                .next_short_string()
                .and_then_tc(|v| output.serialize_str(&v)),
            TypeDef::Bytes31 | TypeDef::Bytes31Encoded(_) => input
                .next_bytes::<31>()
                .and_then_tc(|v| output.serialize_byte_string(&v)),
            TypeDef::Bool => input.next_bool().and_then_tc(|v| output.serialize_bool(v)),
            TypeDef::U8 => input.next_u8().and_then_tc(|v| output.serialize_u8(v)),
            TypeDef::U16 => input.next_u16().and_then_tc(|v| output.serialize_u16(v)),
            TypeDef::U32 => input.next_u32().and_then_tc(|v| output.serialize_u32(v)),
            TypeDef::U64 => input.next_u64().and_then_tc(|v| output.serialize_u64(v)),
            TypeDef::U128 => input.next_u128().and_then_tc(|v| output.serialize_u128(v)),
            TypeDef::U256 => input.next_u256().and_then_tc(|v| output.serialize_u256(v)),
            TypeDef::U512 => input.next_u512().and_then_tc(|v| output.serialize_u512(v)),
            TypeDef::I8 => input.next_i8().and_then_tc(|v| output.serialize_i8(v)),
            TypeDef::I16 => input.next_i16().and_then_tc(|v| output.serialize_i16(v)),
            TypeDef::I32 => input.next_i32().and_then_tc(|v| output.serialize_i32(v)),
            TypeDef::I64 => input.next_i64().and_then_tc(|v| output.serialize_i64(v)),
            TypeDef::I128 => input.next_i128().and_then_tc(|v| output.serialize_i128(v)),
            TypeDef::EthAddress => input
                .next_eth_address()
                .and_then_tc(|v| output.serialize_eth_address(v)),
            TypeDef::ByteArray | TypeDef::ByteArrayEncoded(_) | TypeDef::Custom(_) => input
                .next_byte_array_bytes()
                .and_then_tc(|v| output.serialize_byte_string(&v)),
            TypeDef::Utf8String => input
                .next_string()
                .and_then_tc(|v| output.serialize_str(&v)),
            TypeDef::Tuple(tuple) => tuple.transcode(input, output),
            TypeDef::Array(a) => a.transcode(input, output),
            TypeDef::FixedArray(fa) => fa.transcode(input, output),
            TypeDef::Felt252Dict(_ty) => unimplemented!(), // TODO: implement Felt252Dict parsing
            TypeDef::Struct(s) => s.transcode(input, output),
            TypeDef::Enum(e) => e.transcode(input, output),
            TypeDef::Ref(_) => Err(DecodeError::message(
                "TypeDef Ref needs to be expanded before trascoding",
            )),
            TypeDef::Option(option) => option.transcode(input, output),
            TypeDef::Result(result) => result.transcode(input, output),
            TypeDef::Nullable(nullable) => nullable.transcode(input, output),
        }
    }
}

impl<In, Out> Transcode<In, Out> for TupleDef
where
    In: CairoDeserializer,
    Out: CairoSerializer,
{
    type DeserializeError = DecodeError;
    type SerializeError = Out::Error;
    type Ok = Out::Ok;
    fn transcode(
        &self,
        input: &mut In,
        output: Out,
    ) -> Result<Self::Ok, TranscodeError<Self::DeserializeError, Self::SerializeError>> {
        output.serialize_tuple(len)
    }
}

impl<In, Out> Transcode<In, Out> for ArrayDef
where
    In: CairoDeserializer,
    Out: TranscodeWriter<In> + CairoWrite,
{
    fn transcode(&self, input: &mut In, output: &mut Out) -> DecodeResult<()> {
        let len = input.next_u32()?;
        output.write_bytes(&len.to_be_bytes())?;
        for _ in 0..len {
            self.type_def.transcode(input, output)?;
        }
        Ok(())
    }
}

impl<In, Out> Transcode<In, Out> for FixedArrayDef
where
    In: CairoDeserializer,
    Out: TranscodeWriter<In> + CairoWrite,
{
    fn transcode(&self, input: &mut In, output: &mut Out) -> DecodeResult<()> {
        for _ in 0..self.size {
            self.type_def.transcode(input, output)?;
        }
        Ok(())
    }
}

impl<In, Out> Transcode<In, Out> for StructDef
where
    In: CairoDeserializer,
    Out: TranscodeWriter<In> + CairoWrite,
{
    fn transcode(&self, input: &mut In, output: &mut Out) -> DecodeResult<()> {
        for member in &self.members {
            member.type_def.transcode(input, output)?;
        }
        Ok(())
    }
}

impl<In, Out> Transcode<In, Out> for EnumDef
where
    In: CairoDeserializer,
    Out: TranscodeWriter<In> + CairoWrite,
{
    fn transcode(&self, input: &mut In, output: &mut Out) -> DecodeResult<()> {
        let selector = input.next_enum_variant()?;
        output.write_felt(selector)?;
        self.get_variant(&selector)?
            .type_def
            .transcode(input, output)
    }
}

impl<In, Out> Transcode<In, Out> for OptionDef
where
    In: CairoDeserializer,
    Out: TranscodeWriter<In> + CairoWrite,
{
    fn transcode(&self, input: &mut In, output: &mut Out) -> DecodeResult<()> {
        let is_some = input.next_option_is_some()?;
        output.write_byte(is_some as u8)?;
        match is_some {
            true => self.type_def.transcode(input, output),
            false => Ok(()),
        }
    }
}

impl<In, Out> Transcode<In, Out> for ResultDef
where
    In: CairoDeserializer,
    Out: TranscodeWriter<In> + CairoWrite,
{
    fn transcode(&self, input: &mut In, output: &mut Out) -> DecodeResult<()> {
        let is_ok = input.next_result_is_ok()?;
        output.write_byte(is_ok as u8)?;
        match is_ok {
            true => self.ok.transcode(input, output),
            false => self.err.transcode(input, output),
        }
    }
}

impl<In, Out> Transcode<In, Out> for NullableDef
where
    In: CairoDeserializer,
    Out: TranscodeWriter<In> + CairoWrite,
{
    fn transcode(&self, input: &mut In, output: &mut Out) -> DecodeResult<()> {
        let is_null = input.next_nullable_is_null()?;
        output.write_byte(is_null as u8)?;
        match is_null {
            true => Ok(()),
            false => self.type_def.transcode(input, output),
        }
    }
}
