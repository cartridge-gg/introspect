use cgg_utils::testing::{Fuzzy, random_snake_string};
use introspect_types::structured::{PrimaryDef, PrimaryTypeDef};
use snforge_std::fuzzable::{Fuzzable, generate_arg};
use super::attribute::FuzzableAttribute;

pub impl PrimaryTypeDefFuzzable of Fuzzable<PrimaryTypeDef> {
    fn blank() -> PrimaryTypeDef {
        Default::default()
    }

    fn generate() -> PrimaryTypeDef {
        match generate_arg(0, 19) {
            0 => PrimaryTypeDef::Felt252,
            1 => PrimaryTypeDef::ShortUtf8,
            2 => PrimaryTypeDef::Bytes31,
            3 => PrimaryTypeDef::Bytes31Encoded("ascii"),
            4 => PrimaryTypeDef::Bool,
            5 => PrimaryTypeDef::U8,
            6 => PrimaryTypeDef::U16,
            7 => PrimaryTypeDef::U32,
            8 => PrimaryTypeDef::U64,
            9 => PrimaryTypeDef::U128,
            10 => PrimaryTypeDef::I8,
            11 => PrimaryTypeDef::I16,
            12 => PrimaryTypeDef::I32,
            13 => PrimaryTypeDef::I64,
            14 => PrimaryTypeDef::I128,
            15 => PrimaryTypeDef::ClassHash,
            16 => PrimaryTypeDef::ContractAddress,
            17 => PrimaryTypeDef::EthAddress,
            18 => PrimaryTypeDef::StorageAddress,
            19 => PrimaryTypeDef::StorageBaseAddress,
            _ => panic!("Unreachable"),
        }
    }
}


pub impl PrimaryDefFuzzable of Fuzzable<PrimaryDef> {
    fn blank() -> PrimaryDef {
        Default::default()
    }

    fn generate() -> PrimaryDef {
        PrimaryDef {
            name: random_snake_string(64, 5),
            attributes: Fuzzy::generate_span_lt(10),
            type_def: Fuzzable::generate(),
        }
    }
}
