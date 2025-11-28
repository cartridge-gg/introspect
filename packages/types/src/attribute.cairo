use core::num::traits::Pow;
use crate::ISerde;
use crate::serde::{ISerdeByteArray, SHIFT_31B};


pub const B31_4: felt252 = 0b00000100 * SHIFT_31B;
pub const B31_4_U256: u256 = (0b00000100 * 256_u256.pow(31));

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct Attribute {
    pub name: ByteArray,
    pub data: Option<ByteArray>,
}


pub impl AttributeISerde of ISerde<Attribute> {
    fn iserialize(self: @Attribute, ref output: Array<felt252>) {
        match self.data {
            Option::Some(data) => {
                output.append(self.name.iserialize_and_last(ref output) + B31_4);
                data.iserialize(ref output);
            },
            Option::None => { self.name.iserialize(ref output); },
        }
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<Attribute> {
        let (name_data, last) = ISerdeByteArray::ideserialize_and_last(ref serialized)?;
        let last_u256: u256 = last.into();
        let (data, last) = if last_u256 >= B31_4_U256 {
            (Some(ISerde::ideserialize(ref serialized)?), last - B31_4)
        } else {
            (None, last)
        };
        Some(Attribute { name: ISerdeByteArray::ideserialize_from_parts(name_data, last)?, data })
    }
}
