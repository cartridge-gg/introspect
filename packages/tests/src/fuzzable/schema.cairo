use introspect_tests::{ByteArrayExt, random_pascal_string};
use introspect_types::{Attribute, ColumnDef, TypeDef};
use snforge_std::fuzzable::{Fuzzable, FuzzableBool};
use crate::Fuzzy;
use super::FuzzyImpl;
use super::type_def::TypeDefFuzzable;


pub fn generate_column_attributes() -> Span<Attribute> {
    match FuzzableBool::generate() {
        false => [].span(),
        true => [Attribute { name: "key", data: None }].span(),
    }
}

pub impl FuzzableColumnDef<const MAX_DEPTH: u32> of Fuzzable<ColumnDef> {
    fn blank() -> ColumnDef {
        ColumnDef { id: 0, name: "", attributes: [].span(), type_def: TypeDef::None }
    }
    fn generate() -> ColumnDef {
        let name = random_pascal_string(31, 4);
        let id = name.selector();
        ColumnDef {
            id,
            name,
            attributes: generate_column_attributes(),
            type_def: TypeDefFuzzable::generate(MAX_DEPTH),
        }
    }
}

pub impl FuzzableExtColumnDef<const MAX_DEPTH: u32> of Fuzzy<ColumnDef> {
    fn generate() -> ColumnDef {
        let name = random_pascal_string(31, 4);
        let id = name.selector();
        ColumnDef {
            id,
            name,
            attributes: generate_column_attributes(),
            type_def: TypeDefFuzzable::generate(MAX_DEPTH),
        }
    }
}

