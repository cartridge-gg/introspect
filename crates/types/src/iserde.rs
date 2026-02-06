use crate::FeltIterator;
use crate::deserialize::{ByteArray, CairoDeserialize, CairoDeserializer};
use starknet_types_core::felt::Felt;

pub struct CairoISerde<I: FeltIterator>(pub I);

impl<I: FeltIterator> CairoDeserializer for CairoISerde<I> {
    fn next_felt(&mut self) -> Option<Felt> {
        self.0.next()
    }

    fn next_option<T: CairoDeserialize<Self>>(&mut self) -> Option<Option<T>> {
        if self.next_bool()? {
            T::deserialize(self).map(Some)
        } else {
            Some(None)
        }
    }

    fn next_byte_array(&mut self) -> Option<ByteArray> {
        self.next_byte_array_with_info_byte()
            .map(|(bytes, _)| bytes)
            .map(Into::into)
    }
}

impl<I: FeltIterator> CairoISerde<I> {
    pub fn next_byte_array_with_info_byte(&mut self) -> Option<(Vec<u8>, u8)> {
        let mut bytes = Vec::new();
        loop {
            let felt_bytes = self.next_felt_bytes()?;
            let info = felt_bytes[0];
            bytes.extend_from_slice(match info & 2 {
                0 => &felt_bytes[1..32],
                _ => &felt_bytes[(32 - felt_bytes[1] as usize)..32],
            });

            if info & 1 == 1 {
                return Some((bytes, info));
            }
        }
    }

    pub fn deserialize_end<T: CairoDeserialize<Self>>(&mut self) -> Vec<T> {
        let mut values = Vec::new();
        while let Some(value) = T::deserialize(self) {
            values.push(value);
        }
        values
    }
}
