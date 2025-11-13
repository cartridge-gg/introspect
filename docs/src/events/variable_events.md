# Variable Events

These events are for values that don't fit into the table/record model, such as global variables or configuration settings.

- `RegisterVariable`: Register a new variable with value.
- `SetVariable`: Set the value of an existing variable.
- `DeclareVariable`: Register a new variable with a given name and type.
- `DeleteVariable`: Delete an existing variable.

```rust
/// id: felt252 - Unique identifier for the variable.
/// name: ByteArray - Name of the variable.
/// type_def: TypeDef - Type definition of the variable.
/// data: Span<felt252> - Serialised data being set.


struct RegisterVariable {
    #[key]
    id: felt252,
    name: ByteArray,
    type_def: TypeDef,
    data: Span<felt252>,
}

struct DeclareVariable {
    #[key]
    id: felt252,
    name: ByteArray,
    type_def: TypeDef,
}

struct SetVariable {
    #[key]
    id: felt252,
    data: Span<felt252>,
}


struct DeleteVariable {
    #[key]
    id: felt252,
}

```
