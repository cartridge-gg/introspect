use serde::{Deserialize, Serialize};

use crate::decode_error::DecodeResultTrait;
use crate::deserialize::{ByteArray, CairoDeserialize, CairoDeserializer};
use crate::iserde::CairoISerde;
use crate::serde::CairoSerde;
use crate::utils::ideserialize_byte_array_with_last;
use crate::{DecodeResult, FeltSource};

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

impl<I: FeltSource> CairoDeserialize<CairoSerde<I>> for Attribute {
    fn deserialize(deserializer: &mut CairoSerde<I>) -> DecodeResult<Self> {
        let name = deserializer.next_string()?;
        let data = deserializer
            .next_option::<ByteArray>()
            .raise_eof()?
            .map(Into::into);
        Ok(Attribute { name, data })
    }
}

impl<I: FeltSource> CairoDeserialize<CairoISerde<I>> for Attribute {
    fn deserialize(deserializer: &mut CairoISerde<I>) -> DecodeResult<Self> {
        let (name_bytes, info) = ideserialize_byte_array_with_last(&mut deserializer.0)?;
        let name = String::from_utf8_lossy(&name_bytes).into_owned();
        let data = if info & 0b100 != 0 {
            Some(deserializer.next_byte_array().raise_eof()?.into())
        } else {
            None
        };
        Ok(Attribute { name, data })
    }
}

pub trait Attributes {
    fn attributes(&self) -> &[Attribute];
    fn get_attribute(&self, name: &str) -> Option<Option<&[u8]>> {
        self.attributes()
            .iter()
            .find(|attr| attr.name == name)
            .map(|attr| attr.data.as_deref())
    }
    fn has_attribute(&self, name: &str) -> bool {
        self.attributes().iter().any(|attr| attr.name == name)
    }
}
