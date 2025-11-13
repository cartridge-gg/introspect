## Type Definitions

The following data structures are used to describe the types and schemas of cairo structures.

`TypeDef`: An enum to represent what the value should be decoded as. The values are:

- `None` A None type such as a `()` or an empty enum.
- `Felt252` A `felt252` field element (The base type of starknet).
- `Bytes31` 31 bytes packed into a felt252.
- `Bool` A boolean value (true or false).
- `U8` An unsigned 8-bit integer.
- `U16` An unsigned 16-bit integer.
- `U32` An unsigned 32-bit integer.
- `U64` An unsigned 64-bit integer.
- `U128` An unsigned 128-bit integer.
- `U256` An unsigned 256-bit integer.
- `U512` An unsigned 512-bit integer.
- `I8` A signed 8-bit integer.
- `I16` A signed 16-bit integer.
- `I32` A signed 32-bit integer.
- `I64` A signed 64-bit integer.
- `I128` A signed 128-bit integer.
- `ClassHash` A class hash (The hash of a contract class).
- `ContractAddress` A contract address (The address of a deployed contract).
- `EthAddress` An Ethereum address.
- `StorageAddress` A storage address (The address of a storage location).
- `StorageBaseAddress` A storage base address (The base address of a storage location).
- `ByteArray` A byte array (An array of bytes31 with a pending word).
- `ShortString` A short string (A string of limited length).
- `Tuple` A tuple (A fixed-size collection of values).
- `Array` An array (A dynamically-sized collection of values).
- `FixedArray` A fixed-size array (An array with a known size at compile time).
- `Felt252Dict` A dictionary (A mapping from felt252 keys to values).
- `Struct` A struct (A custom data structure with named fields).
- `Enum` An enum (A custom data structure with named and typed variants).
- `Option` An option type (A type that can be either Some or None).
- `Result` A result type (A type that can be either Ok or Err).
- `Nullable` A nullable type (A type that can be null).
- `Ref` A reference type (A type that refers to another value).
- `Custom` A custom type (A user-defined data structure and decoding).

```rust
enum TypeDef {
    #[default]
    None,
    Felt252,
    Bytes31,
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    U512,
    I8,
    I16,
    I32,
    I64,
    I128,
    ClassHash,
    ContractAddress,
    EthAddress,
    StorageAddress,
    StorageBaseAddress,
    ByteArray,
    ShortString,
    Tuple: Span<TypeDef>,
    Array: Box<TypeDef>,
    FixedArray: Box<FixedArrayDef>,
    Felt252Dict: Box<TypeDef>,
    Struct: StructDef,
    Enum: EnumDef,
    Option: Box<TypeDef>,
    Result: Box<ResultDef>,
    Nullable: Box<TypeDef>,
    Ref: felt252,
    Custom: felt252,
}
```

> [!IMPORTANT] > `Box<T>` are used to wrap types that also contain TypeDefs as the compiler can't take recursive types.

## Attributes

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

## Structs

`StructDef`: Defines the structure of a struct type with the fields:

- `name`: The name of the struct.
- `attributes`: A span of attributes associated with the struct.
- `members`: A span of `MemberDef` representing the members of the struct.

`MemberDef`: Defines a member of a struct with the fields:

- `name`: The name of the member.
- `attributes`: A span of attributes associated with the member.
- `type_def`: The type definition of the member.

```rust
struct StructDef {
    name: ByteArray,
    attributes: Span<Attribute>,
    members: Span<MemberDef>,
}

struct MemberDef {
    name: ByteArray,
    attributes: Span<Attribute>,
    type_def: TypeDef,
}
```

## Enums

`EnumDef`: Defines the structure of an enum type with the fields:

- `name`: The name of the enum.
- `attributes`: A span of attributes associated with the enum.
- `variants`: A span of `VariantDef` representing the variants of the enum.

`VariantDef`: Defines a variant of an enum with the fields:

- `selector`: The selector of the variant.
- `name`: The name of the variant.
- `attributes`: A span of attributes associated with the variant.
- `type_def`: An optional type definition of the variant.

```rust
struct EnumDef {
    name: ByteArray,
    attributes: Span<Attribute>,
    variants: Span<VariantDef>,
}

struct VariantDef {
    selector: felt252,
    name: ByteArray,
    attributes: Span<Attribute>,
    type_def: TypeDef,
}
```

## Fixed Arrays

`FixedArrayDef`: Defines a fixed-size array type with the fields:

- `type_def`: The type definition of the elements in the array.
- `size`: The size of the array.

```rust
struct FixedArrayDef {
    type_def: TypeDef,
    size: u32,
}
```

## Results

`ResultDef`: Defines the structure of a result type with the fields:

- `ok`: The type definition of the Ok variant.
- `err`: The type definition of the Err variant.

```rust
struct ResultDef {
    ok: TypeDef,
    err: TypeDef,
}
```
