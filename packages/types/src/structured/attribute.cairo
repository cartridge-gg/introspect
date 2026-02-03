use core::num::traits::Pow;
use crate::ISerde;
use crate::serde::{ISerdeByteArray, SHIFT_31B, full_terminator, partial_terminator};


pub const B31_4: felt252 = 0b00000100 * SHIFT_31B;
pub const B31_4_U256: u256 = (0b00000100 * 256_u256.pow(31));

pub const fn partial_terminator_with_data(word: felt252, size: felt252) -> felt252 {
    B31_4 + partial_terminator(word, size)
}

pub const fn full_terminator_with_data(word: felt252) -> felt252 {
    B31_4 + full_terminator(word)
}

#[derive(Drop, Serde, PartialEq, Debug, Default)]
pub struct Attribute {
    pub name: ByteArray,
    pub data: Option<ByteArray>,
}

pub fn attribute(name: ByteArray, data: Option<ByteArray>) -> Attribute {
    Attribute { name, data }
}

pub fn attribute_empty(name: ByteArray) -> Attribute {
    Attribute { name, data: Option::None }
}

pub fn attribute_data(name: ByteArray, data: ByteArray) -> Attribute {
    Attribute { name, data: Some(data) }
}


pub impl ISerdeAttribute of ISerde<Attribute> {
    const SIZE_HINT: Option<u32> = None;
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
    fn iserialized_size(self: @Attribute) -> u32 {
        self.name.iserialized_size()
            + match self.data {
                Option::Some(data) => data.iserialized_size(),
                Option::None => 0,
            }
    }
}
