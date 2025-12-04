use introspect_types::{ISerde, TypeDef};
use starknet::Event;
use crate::utils::{DrainSpanTrait, VerifyEventDeserializeTrait};

// Emitted when a variable is declared.
/// Fields:
/// - `id`: Unique identifier of the variable (e.g., hash of the name or a custom ID).
/// - `name`: Human-readable name of the variable.
/// - `type_def`: The type description of the variable.
#[derive(Drop, Serde, PartialEq, Debug)]
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
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DeclareVariable {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub type_def: TypeDef,
    pub data: Span<felt252>,
}

// Emitted when a variable's value is set.
/// Fields:
/// - `id`: Unique identifier of the variable (e.g., hash of the name or a custom ID).
/// - `value`: The new value of the variable.
#[derive(Drop, Serde, PartialEq, Debug)]
pub struct SetVariable {
    #[key]
    pub id: felt252,
    pub data: Span<felt252>,
}


#[derive(Drop, Serde, PartialEq, Debug)]
pub struct RenameVariable {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
}

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct DeleteVariable {
    #[key]
    pub id: felt252,
}


impl RegisterVariableEvent of Event<RegisterVariable> {
    fn append_keys_and_data(
        self: @RegisterVariable, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.id);
        self.name.iserialize(ref data);
        self.type_def.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RegisterVariable> {
        RegisterVariable {
            id: *keys.pop_front()?,
            name: ISerde::ideserialize(ref data)?,
            type_def: ISerde::ideserialize(ref data)?,
        }
            .verify(ref keys, ref data)
    }
}

impl DeclareVariableEvent of Event<DeclareVariable> {
    fn append_keys_and_data(
        self: @DeclareVariable, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.id);
        self.name.iserialize(ref data);
        self.type_def.iserialize(ref data);
        data.append_span(*self.data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeclareVariable> {
        DeclareVariable {
            id: *keys.pop_front()?,
            name: ISerde::ideserialize(ref data)?,
            type_def: ISerde::ideserialize(ref data)?,
            data: data.drain(),
        }
            .verify_keys(ref keys)
    }
}

impl SetVariableEvent of Event<SetVariable> {
    fn append_keys_and_data(
        self: @SetVariable, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.id);
        data.append_span(*self.data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<SetVariable> {
        SetVariable { id: *keys.pop_front()?, data: data.drain() }.verify_keys(ref keys)
    }
}

impl RenameVariableEvent of Event<RenameVariable> {
    fn append_keys_and_data(
        self: @RenameVariable, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.id);
        self.name.iserialize(ref data);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<RenameVariable> {
        RenameVariable { id: *keys.pop_front()?, name: ISerde::ideserialize(ref data)? }
            .verify(ref keys, ref data)
    }
}

impl DeleteVariableEvent of Event<DeleteVariable> {
    fn append_keys_and_data(
        self: @DeleteVariable, ref keys: Array<felt252>, ref data: Array<felt252>,
    ) {
        keys.append(*self.id);
    }

    fn deserialize(ref keys: Span<felt252>, ref data: Span<felt252>) -> Option<DeleteVariable> {
        DeleteVariable { id: *keys.pop_front()? }.verify(ref keys, ref data)
    }
}
