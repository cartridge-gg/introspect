use crate::transcode::{CairoWrite, Transcode, TranscodeWriter};
use crate::{
    ArrayDef, CairoDeserializer, DecodeError, DecodeResult, EnumDef, FixedArrayDef, NullableDef,
    OptionDef, ResultDef, StructDef, TupleDef, TypeDef,
};

impl<In, Out> Transcode<In, Out> for TypeDef
where
    In: CairoDeserializer,
    Out: TranscodeWriter<In> + CairoWrite,
{
    fn transcode(&self, input: &mut In, output: &mut Out) -> DecodeResult<()> {
        match self {
            TypeDef::None => Ok(()),
            TypeDef::Felt252
            | TypeDef::ClassHash
            | TypeDef::ContractAddress
            | TypeDef::StorageAddress
            | TypeDef::StorageBaseAddress => input.next_felt().and_then(|v| output.write_felt(v)),
            TypeDef::ShortUtf8 | TypeDef::Bytes31 | TypeDef::Bytes31Encoded(_) => {
                input.next_bytes::<31>().map(|v| output.write_bytes(&v))?
            }
            TypeDef::Bool => output.transcode_bytes::<1>(input),
            TypeDef::U8 => output.transcode_bytes::<1>(input),
            TypeDef::U16 => output.transcode_bytes::<2>(input),
            TypeDef::U32 => output.transcode_bytes::<4>(input),
            TypeDef::U64 => output.transcode_bytes::<8>(input),
            TypeDef::U128 => output.transcode_bytes::<16>(input),
            TypeDef::U256 => input
                .next_u256()
                .and_then(|v| output.write_bytes(&v.to_big_endian())),
            TypeDef::U512 => input
                .next_u512()
                .and_then(|v| output.write_bytes(&v.to_big_endian())),
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
                .and_then(|v| output.write_variable_bytes(&v)),
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
    Out: TranscodeWriter<In> + CairoWrite,
{
    fn transcode(&self, input: &mut In, output: &mut Out) -> DecodeResult<()> {
        for element in &self.elements {
            element.transcode(input, output)?;
        }
        Ok(())
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
