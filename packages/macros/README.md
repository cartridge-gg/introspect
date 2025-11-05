# Introspect Macros

This package contains cairo macros for implementing the introspection of types and data.

Derive macros:

- `Introspect`: Automatically implement the `Introspect` trait for a struct or enum.
- `Schema`: Automatically implement the `Schema` trait for a struct or enum.

Inline macros:

- `Introspect`: Implement the `Introspect` trait for a struct or enum with custom attributes.
- `Schema`: Implement the `Schema` trait for a struct or enum with custom attributes.

## Derive

### Introspect

The derive macro can be used to automatically implement the `Introspect` trait for a struct or enum without added attributes.

```rust
#[derive(Introspect)]
struct MyStruct {
    #[encoding: "utf-8" ]]
    field1: felt252,
    field2: Option<felt252>,
}

#[derive(Introspect)]
enum MyEnum {
    #[id: "custom_selector"]
    Variant1,
    #[selector: "custom_selector_2"]
    Variant2: felt252,
}

```

### Schema

Used for defining schemas

```rust
#[derive(Schema)]
struct MySchema{
    #[id: "custom_id"]
    #[name: "custom-name"]

    field1: MyStruct,
    #[selector: "custom_selector"] // made with selector!("custom_selector")
    #[name: "another_name"]
    field2: MyEnum,
    field3: Array<felt252>,
    #[encoding: "utf-8"]
    something: ByteArray,
}
```

set_member('field1', model_id, my_struct);
set_member(selector!("custom_selector"), model_id, my_struct);

```rust
#[introspect: ]
struct MyStruct {
    #[encoding: "utf-8" ]]
    field1: felt252,
    field2: Option<felt252>,
}

```
