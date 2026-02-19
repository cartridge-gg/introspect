use crate::deserialize::CairoDeserializer;
use crate::felt::IntoFeltSource;
use crate::{DecodeResult, FeltSource};
use starknet_types_core::felt::Felt;

pub struct CairoSerde<F: FeltSource>(pub F);

impl<I: FeltSource> CairoSerde<I> {
    pub fn new(iterator: I) -> Self {
        CairoSerde(iterator)
    }
}

impl<'a, S: FeltSource + ?Sized> CairoSerde<&'a mut S> {
    #[inline]
    pub fn from_mut(source: &'a mut S) -> Self {
        Self(source)
    }
}

impl<T: IntoFeltSource> From<T> for CairoSerde<T::Source> {
    fn from(source: T) -> Self {
        CairoSerde(source.into_source())
    }
}

impl<I: FeltSource> CairoDeserializer for CairoSerde<I> {
    fn next_felt(&mut self) -> DecodeResult<Felt> {
        self.0.next()
    }
}

impl<I: FeltSource> FeltSource for CairoSerde<I> {
    fn next(&mut self) -> Result<Felt, crate::DecodeError> {
        self.0.next()
    }

    fn position(&self) -> usize {
        self.0.position()
    }
}
