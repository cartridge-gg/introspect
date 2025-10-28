use introspect_types::TypeDef;

#[derive(Drop, Serde, starknet::Event)]
pub enum VariableEvents {
    DeclareVariable: DeclareVariable,
    SetVariable: SetVariable,
    DeleteVariable: DeleteVariable,
}


// Emitted when a variable is declared.
/// Fields:
/// - `id`: Unique identifier of the variable (e.g., hash of the name or a custom ID).
/// - `name`: Human-readable name of the variable.
/// - `type_def`: The type description of the variable.
#[derive(Drop, Serde, starknet::Event)]
pub struct RegisterVariable {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub type_def: TypeDef,
}

// Emitted when a variable is declared.
/// Fields:
/// - `id`: Unique identifier of the variable (e.g., hash of the name or a custom ID).
/// - `name`: Human-readable name of the variable.
/// - `type_def`: The type description of the variable.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeclareVariable {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub type_def: TypeDef,
    pub value: Span<felt252>,
}

// Emitted when a variable's value is set.
/// Fields:
/// - `id`: Unique identifier of the variable (e.g., hash of the name or a custom ID).
/// - `value`: The new value of the variable.
#[derive(Drop, Serde, starknet::Event)]
pub struct SetVariable {
    #[key]
    pub id: felt252,
    pub value: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DeleteVariable {
    #[key]
    pub id: felt252,
}
