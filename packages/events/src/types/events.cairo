use introspect_types::TypeDef;
use starknet::ClassHash;

pub mod selectors {
    pub const DeclareSchema: felt252 = selector!("DeclareSchema");
    pub const DeclareType: felt252 = selector!("DeclareType");
    pub const DeclareTypeFromClass: felt252 = selector!("DeclareTypeFromClass");
    pub const DeclareSchemaFromClass: felt252 = selector!("DeclareSchemaFromClass");
}

/// Declares a reusable schema layout.
///
/// Fields:
/// - `id`: Deterministic schema ID (e.g., hash of fields).
/// - `fields`: Field members (selector + layout + attributes)

#[derive(Drop, Serde, starknet::Event)]
pub struct DeclareType {
    #[key]
    pub id: felt252,
    pub type_def: TypeDef,
}

