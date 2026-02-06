use serde::{Deserialize, Serialize};

use crate::FeltIterator;
use crate::deserialize::{ByteArray, CairoDeserialize, CairoDeserializer};
use crate::iserde::CairoISerde;
use crate::serde::CairoSerde;
use crate::utils::ideserialize_byte_array_with_last;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub data: Option<Vec<u8>>,
}

impl Attribute {
    pub fn new_empty(name: String) -> Attribute {
        Attribute { name, data: None }
    }
}

impl<I: FeltIterator> CairoDeserialize<CairoSerde<I>> for Attribute {
    fn deserialize(deserializer: &mut CairoSerde<I>) -> Option<Self> {
        let name = deserializer.next_string()?;
        let data = deserializer.next_option::<ByteArray>()?.map(Into::into);
        Some(Attribute { name, data })
    }
}

impl<I: FeltIterator> CairoDeserialize<CairoISerde<I>> for Attribute {
    fn deserialize(deserializer: &mut CairoISerde<I>) -> Option<Self> {
        let (name_bytes, info) = ideserialize_byte_array_with_last(&mut deserializer.0)?;
        let name = String::from_utf8(name_bytes).ok()?;
        let data = if info & 0b10000000 != 0 {
            Some(deserializer.next_byte_array()?.into())
        } else {
            None
        };
        Some(Attribute { name, data })
    }
}
