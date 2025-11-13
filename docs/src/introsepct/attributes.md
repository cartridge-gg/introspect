# Attributes

An attribute is a key-value pair that can be attached to various type definitions to provide additional metadata or information about the type such as encoding.

`Attribute`: Defines an attribute with the fields:

- `id`: The name of the attribute.
- `data`: A span of data associated with the attribute.

```rust
struct Attribute {
    id: felt252,
    data: Span<felt252>,
}
```
