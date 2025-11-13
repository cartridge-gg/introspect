# Type Events

- `DeclareSchema`: Declare a new schema with a given name and structure.
- `DeclareType`: Declare a new type with a given name and structure.

```rust
struct DeclareSchema {
  /// A unique identifier for the schema.
  #[key]
  id: felt252,
  /// A list of column definitions that make up the schema.
  columns: Span<ColumnDef>,
}

struct DeclareType {
    /// A unique identifier for the type.
    #[key]
    id: felt252,
    /// The type definition.
    type_def: TypeDef,
}
```

> [!IMPORTANT]
> The ids for these types should be generated using a consistent hashing function to ensure uniqueness across different contracts.
