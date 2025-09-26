use introspect_types::{Field, Ty};

pub enum TypeEvents {
    DeclareSchema: DeclareSchema,
    DeclareCustom: DeclareCustom,
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
    pub version: felt252,
    pub fields: Span<Field>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DeclareCustom {
    #[key]
    pub id: felt252,
    pub version: felt252,
    pub custom: Ty,
}

