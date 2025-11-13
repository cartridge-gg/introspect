# Introspection

---

snip: $SNIP_ID
title: Introspection
description: Introspection of onchain types and data
author: Bengineer <@bengineer42>, Glihm <@glihm>
discussions-to: The url pointing to the official discussion thread
status: Draft
type: Standards Track
category: SRC
created: 2025-10-28

---

<!-- Refer to: <https://github.com/starknet-io/SNIPs/blob/main/SNIPS/snip-1.md#snip-header-preamble> -->

## Simple Summary

This SNIP proposes a standardized collection of events and data structures to describe on-chain data structures, enabling the creation of generic indexers and tooling to interact with on-chain data.

## Motivation

One of the challenges with building on starknet is making chain data easily accessible and queryable to client side applications. Most apps will have to make there own systems to fetch, decode and make avaliable the data stored on chain. This leads to a lot of duplicated effort, high barrier to entry for new developers in both time and skill.

Some standards such as ERC20 and ERC721 have helped with this by providing a common interface for certain data structures, but there is no general purpose standard for describing arbitrary data structures stored on chain.

## Specification

The standard consists of two main parts the events and data structures used to describe the data.

### TypeEvents

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

### Database Events

These consist of events for table, column and record manipulation.

#### Table Management Events:

- `CreateTable`: Create a new table with a given name.
- `CreateTableWithColumns`: Create or update a table with a given name and columns.
- `CreateTableWithSchema`: Create or update a table with a given name and schema.
- `RenameTable`: Rename an existing table.
- `DropTable`: Drop an existing table.

```rust
/// id: felt252 - Unique identifier for the table.
/// name: ByteArray - Name of the table.
/// primary: FieldDef - Definition of the primary key field.
/// columns: Span<ColumnDef> - Definitions of the columns in the table.
/// schema: felt252 - Identifier of the schema used by the table.

struct CreateTable {
    #[key]
    id: felt252,
    name: ByteArray,
    primary: FieldDef,
}

struct CreateTableWithColumns {
    #[key]
    id: felt252,
    name: ByteArray,
    columns: Span<ColumnDef>,
    primary: FieldDef,
}

struct CreateTableWithSchema {
    #[key]
    id: felt252,
    name: ByteArray,
    primary: FieldDef,
    schema: felt252,
}

struct RenameTable {
    #[key]
    id: felt252,
    new_name: ByteArray,
}

struct DropTable {
    #[key]
    id: felt252,
}
```

#### Primary Key Management Events:

- `RenamePrimary`: Rename the primary key of a table.
- `RetypePrimary`: Change the type of the primary key of a table.

```rust
/// table: felt252 - Unique identifier for the table.
/// name: ByteArray - Name of the primary key.
/// attributes: Span<Attribute> - Attributes of the column.
/// type_def: TypeDef - Type definition of the primary key.

struct RenamePrimary {
    #[key]
    table: felt252,
    name: ByteArray,
}

struct RetypePrimary {
    #[key]
    table: felt252,
    attributes: Span<Attribute>,
    type_def: TypeDef,
}

```

#### Column Management Events:

-
- `AddColumn`: Add a new column to a table.
- `AddColumns`: Add multiple new columns to a table.
- `RenameColumn`: Rename an existing column in a table.
- `RenameColumns`: Rename multiple existing columns in a table.
- `RetypeColumn`: Change the type of an existing column in a table.
- `RetypeColumns`: Change the types of multiple existing columns in a table.
- `DropColumn`: Drop an existing column from a table.
- `DropColumns`: Drop multiple existing columns from a table.

```rust
/// table: felt252 - Unique identifier for the table.
/// column: felt252 - Unique identifier for the column.
/// name: ByteArray - Name of the column.
/// attributes: Span<Attribute> - Attributes of the column.
/// type_def: TypeDef - Type definition of the column.

struct AddColumn {
    #[key]
    table: felt252,
    /// id: Unique identifier for the column.
    #[key]
    id: felt252,
    name: ByteArray,
    attributes: Span<Attribute>,
    type_def: TypeDef,
}

struct AddColumns {
    #[key]
    table: felt252,
    /// columns: Definitions of the columns being added.
    columns: Span<ColumnDef>,
}

struct RenameColumn {
    #[key]
    table: felt252,
    #[key]
    column: felt252,
    name: ByteArray,
}

struct RenameColumns {
    #[key]
    table: felt252,
    /// columns: Pairs of column ids and their new names.
    columns: Span<(felt252, ByteArray)>,
}

struct RetypeColumn {
    #[key]
    table: felt252,
    #[key]
    column: felt252,
    attributes: Span<Attribute>,
    type_def: TypeDef,
}

struct RetypeColumns {
    #[key]
    table: felt252,
    /// columns: Pairs of column ids and their new type definitions with attributes.
    columns: Span<(felt252, TypeWithAttributes)>,
}

struct DropColumn {
    #[key]
    table: felt252,
    #[key]
    column: felt252,
}

struct DropColumns {
    #[key]
    table: felt252,
    /// columns: column ids to drop
    columns: Span<felt252>,
}
```

#### Record Manipulation Events:

- `InsertRecord`: Insert or update a record in a table.
- `InsertRecords`: Insert or update multiple records in a table.
- `InsertField`: Insert or update a field in a record.
- `InsertFields`: Insert or update multiple fields in a record.
- `InsertsField`: Insert or update a field in multiple records.
- `InsertsFields`: Insert or update multiple fields in multiple records.
- `InsertSchema`: Insert or update a record in a table using a schema.
- `InsertsSchema`: Insert or update multiple records in a table using a schema.
- `DeleteRecord`: Drop an existing record from a table.
- `DeleteRecords`: Drop multiple existing records from a table.
- `DeleteField`: Drop an existing field from a record.
- `DeleteFields`: Drop multiple existing fields from a record.
- `DeletesField`: Drop an existing field from multiple records.
- `DeletesFields`: Drop multiple existing fields from multiple records.
- `DeleteSchema`: Drop an existing record from a schema.
- `DeletesSchema`: Drop multiple existing records from a schema.

```rust
/// Database values common fields
///
/// - table - Table ID.
/// - record/records - Record ID.
/// - column/columns - Column ID.
/// - data - Serialised data being set.
/// - records_data - Pairs of Record IDs and their serialised data being set.
/// - schema - Schema ID.


struct InsertRecord {
    #[key]
    table: felt252,
    #[key]
    record: felt252,
    data: Span<felt252>,
}

struct InsertRecords {
    #[key]
    table: felt252,
    records_data: Span<(felt252, Span<felt252>)>,
}

struct InsertField {
    #[key]
    table: felt252,
    #[key]
    record: felt252,
    #[key]
    column: felt252,
    data: Span<felt252>,
}


struct InsertFields {
    #[key]
    table: felt252,
    #[key]
    record: felt252,
    columns: Span<felt252>,
    data: Span<felt252>,
}


struct InsertRecordsField {
    #[key]
    table: felt252,
    #[key]
    column: felt252,
    records_data: Span<(felt252, Span<felt252>)>,
}


struct InsertRecordsFields {
    #[key]
    table: felt252,
    columns: Span<felt252>,
    records_data: Span<(felt252, Span<felt252>)>,
}

struct InsertSchema {
    #[key]
    table: felt252,
    #[key]
    record: felt252,
    #[key]
    schema: felt252,
    data: Span<felt252>,
}

struct InsertRecordsSchema {
    #[key]
    table: felt252,
    #[key]
    schema: felt252,
    records_data: Span<(felt252, Span<felt252>)>,
}



struct DeleteRecord {
    #[key]
    table: felt252,
    #[key]
    record: felt252,
}


struct DeleteRecords {
    #[key]
    table: felt252,
    records: Span<felt252>,
}

struct DeleteField {
    #[key]
    table: felt252,
    #[key]
    record: felt252,
    #[key]
    column: felt252,
}


struct DeleteFields {
    #[key]
    table: felt252,
    #[key]
    row: felt252,
    columns: Span<felt252>,
}

struct DeletesField {
    #[key]
    table: felt252,
    #[key]
    column: felt252,
    records: Span<felt252>,

}

struct DeletesFields {
    #[key]
    table: felt252,
    records: Span<felt252>,
    columns: Span<felt252>,
}

struct DeleteSchema {
    #[key]
    table: felt252,
    #[key]
    record: felt252,
    #[key]
    schema: felt252,
}


struct DeletesSchema {
    #[key]
    table: felt252,
    #[key]
    schema: felt252,
    records: Span<felt252>,
}
```

### Variable Events

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

### Type Definitions

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

#### Attributes

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

#### Structs

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

#### Enums

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

#### Fixed Arrays

`FixedArrayDef`: Defines a fixed-size array type with the fields:

- `type_def`: The type definition of the elements in the array.
- `size`: The size of the array.

```rust
struct FixedArrayDef {
    type_def: TypeDef,
    size: u32,
}
```

#### Results

`ResultDef`: Defines the structure of a result type with the fields:

- `ok`: The type definition of the Ok variant.
- `err`: The type definition of the Err variant.

```rust
struct ResultDef {
    ok: TypeDef,
    err: TypeDef,
}
```

### Schema

`ColumnDef`: Defines a column in a schema with the fields:

- `id`: A unique identifier for the column.
- `name`: The name of the column.
- `attributes`: A span of attributes associated with the column.
- `type_def`: The type definition of the column.

`PrimaryDef`: Defines a field in a record with the fields:

- `name`: The name of the field.
- `attributes`: A span of attributes associated with the field.
- `type_def`: The type definition of the field.

```rust
struct ColumnDef {
    id: felt252,
    name: ByteArray,
    attributes: Span<Attribute>,
    type_def: TypeDef,
}

struct FieldDef {
    name: ByteArray,
    attributes: Span<Attribute>,
    type_def: TypeDef,
}
```

### Traits

The Introspect trait is used to describe types so they can be reconstructed by another system.

```rust
trait Introspect<T> {
    fn type_def() -> TypeDef;
    fn child_defs() -> Array<(felt252, TypeDef)> {
        Default::default()
    }
    fn hash() -> felt252 {
        let mut serialized: Array<felt252> = Default::default();
        Serde::<TypeDef>::serialize(@Self::type_def(), ref serialized);
        poseidon_hash_span(serialized.span())
    }
}
```
