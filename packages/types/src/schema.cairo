use crate::{Attribute, TypeDef};

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct ColumnDef {
    pub id: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct PrimaryDef {
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub type_def: PrimaryTypeDef,
}

pub trait Schema<T> {
    fn columns() -> Span<ColumnDef>;
    fn child_defs() -> Array<(felt252, TypeDef)>;
}


#[derive(Drop, Serde, PartialEq, Default, Debug)]
pub enum PrimaryTypeDef {
    #[default]
    Felt252,
    Bytes31,
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    ClassHash,
    ContractAddress,
    EthAddress,
    StorageAddress,
    StorageBaseAddress,
}
