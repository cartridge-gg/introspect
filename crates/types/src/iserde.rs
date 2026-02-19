use crate::decode_error::DecodeResultTrait;
use crate::deserialize::CairoDeserializer;
use crate::felt::IntoFeltSource;
use crate::{ByteArray, DecodeResult, FeltSource};
use starknet_types_core::felt::Felt;

pub struct CairoISerde<I: FeltSource>(pub I);

impl<I: FeltSource> CairoDeserializer for CairoISerde<I> {
    fn next_felt(&mut self) -> DecodeResult<Felt> {
        self.0.next()
    }

    fn next_option_is_some(&mut self) -> DecodeResult<bool> {
        self.next_bool_tag("option")
    }

    fn next_byte_array(&mut self) -> DecodeResult<ByteArray> {
        self.next_byte_array_with_info_byte()
            .map(|(bytes, _)| bytes)
            .map(Into::into)
    }
}

impl<'a, S: FeltSource + ?Sized> CairoISerde<&'a mut S> {
    #[inline]
    pub fn from_mut(source: &'a mut S) -> Self {
        Self(source)
    }
}

impl<T: IntoFeltSource> From<T> for CairoISerde<T::Source> {
    fn from(source: T) -> Self {
        CairoISerde(source.into_source())
    }
}

impl<I: FeltSource> CairoISerde<I> {
    pub fn next_byte_array_with_info_byte(&mut self) -> DecodeResult<(Vec<u8>, u8)> {
        fn extent_bytes(
            deserializer: &mut CairoISerde<impl FeltSource>,
            bytes: &mut Vec<u8>,
        ) -> DecodeResult<u8> {
            let [info, felt_bytes @ ..] = deserializer.next_felt_bytes()?;
            bytes.extend_from_slice(match info & 2 {
                0 => &felt_bytes,
                _ => &felt_bytes[(31 - felt_bytes[1] as usize)..31],
            });
            Ok(info)
        }

        let mut bytes = Vec::new();
        let info = extent_bytes(self, &mut bytes)?;
        if info & 1 == 1 {
            Ok((bytes, info))
        } else {
            loop {
                let info = extent_bytes(self, &mut bytes).raise_eof()?;
                if info & 1 == 1 {
                    break Ok((bytes, info));
                }
            }
        }
    }
}

impl<I: FeltSource> FeltSource for CairoISerde<I> {
    fn next(&mut self) -> Result<Felt, crate::DecodeError> {
        self.0.next()
    }

    fn position(&self) -> usize {
        self.0.position()
    }
}
