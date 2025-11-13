# Defining Types

This library provides a set of macros to make the type definitions for custom structs and enums. These can be used in a default way with the derive macro as well as with options with the attribute macro.

## Derive Macros

The `Introspect`, `IntrospectRef` and `Schema` derive macro can be used on structs and enums to automatically generate the type definitions.

The `Introspect` macro generates the type definition and serialiser for

```rust
#[derive(Introspect)]
struct MyStruct {
    field1: u32,
    field2: bool,
}

#[derive(Introspect)]
enum MyEnum {
    Variant1,
    Variant2: u32,
}

#[derive(IntrospectRef)]
struct ARefStruct {
    field1: u32,
    field2: MyStruct,
    filed3: MyEnum,
}


```
