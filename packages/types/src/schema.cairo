use core::poseidon;
use poseidon::poseidon_hash_span;
use crate::{Attribute, TypeDef};

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct ColumnDef {
    pub id: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub type_def: TypeDef,
}


pub trait Schema<T> {
    fn columns() -> Span<ColumnDef>;
    fn child_defs() -> Array<(felt252, TypeDef)>;
    fn hash() -> felt252 {
        let mut data: Array<felt252> = Default::default();
        let columns = Self::columns();
        columns.serialize(ref data);
        poseidon_hash_span(data.span())
    }
}
