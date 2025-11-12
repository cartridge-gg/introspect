use introspect_types::{ColumnDef, TypeDef};
use starknet::ClassHash;

pub enum TypeEvents {
    DeclareSchema: DeclareSchema,
    DeclareType: DeclareType,
}

/// Declares a reusable schema layout.
///
/// Fields:
/// - `id`: Deterministic schema ID (e.g., hash of fields).
/// - `fields`: Field members (selector + layout + attributes)

#[derive(Drop, Serde, starknet::Event)]
pub struct DeclareSchema {
    #[key]
    pub id: felt252,
    pub columns: Span<ColumnDef>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DeclareType {
    #[key]
    pub id: felt252,
    pub type_def: TypeDef,
}


#[derive(Drop, Serde, starknet::Event)]
pub struct DeclareTypeFromClass {
    #[key]
    pub id: felt252,
    pub class_hash: ClassHash,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DeclareSchemaFromClass {
    #[key]
    pub id: felt252,
    pub version: felt252,
    pub class_hash: ClassHash,
}
