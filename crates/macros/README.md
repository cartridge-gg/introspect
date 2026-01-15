# Introspect Macros Derive

Procedural macros for the introspect project.

## Macros

### `#[set_once]`

Attribute macro that generates `set_*` methods for all `Option<T>` fields that check for duplicate assignments.

```rust
use introspect_macros_derive::set_once;

#[set_once]
#[derive(Default)]
pub struct ColumnAttributes {
    name: Option<String>,
    id: Option<IdVariant>,
}

// Automatically generates:
// impl ColumnAttributes {
//     pub fn set_name(&mut self, value: String) -> Result<()> { ... }
//     pub fn set_id(&mut self, value: IdVariant) -> Result<()> { ... }
// }
```

Each setter will:

- Check if the field is already set
- Return `Err(TableError::DuplicateColumnAttribute(field_name))` if already set
- Otherwise, set the field to `Some(value)` and return `Ok(())`
