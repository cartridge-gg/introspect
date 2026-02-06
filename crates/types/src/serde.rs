use std::vec::IntoIter;

use crate::FeltIterator;
use crate::deserialize::{CairoDeserialize, CairoDeserializer};
use starknet_types_core::felt::Felt;

pub struct CairoSerde<I: FeltIterator>(pub I);

impl From<Vec<Felt>> for CairoSerde<IntoIter<Felt>> {
    fn from(felts: Vec<Felt>) -> Self {
        CairoSerde(felts.into_iter())
    }
}

impl<I: FeltIterator> CairoDeserializer for CairoSerde<I> {
    fn next_felt(&mut self) -> Option<Felt> {
        self.0.next()
    }
    fn next_array<T: CairoDeserialize<Self>>(&mut self) -> Option<Vec<T>> {
        let len = self.next_usize()?;
        (0..len).map(|_| T::deserialize(self)).collect()
    }
}
