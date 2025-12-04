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

One of the challenges with building on starknet is making chain data easily accessible and queryable to client side applications. Most apps will have to make there own systems to fetch, decode and make available the data stored on chain. This leads to a lot of duplicated effort, high barrier to entry for new developers in both time and skill.

Some standards such as ERC20 and ERC721 have helped with this by providing a common interface for certain data structures, but there is no general purpose standard for describing arbitrary data structures stored on chain.

Currently the only source of type data for contracts is the ABI, which only describes function inputs and outputs and events declared by the contract. This is not verified and when deploying could be set to anything.

This SNIP splits the solution into two parts: type declarations and data serialisation. Both these also need standardised event.

This specification also considers gas optimisation by packing some data types and removing redundant length prefixes.

This SNIP does not cover higher level frameworks only the type and event spec.

## Specification

The standard consists of two main parts the events and data structures used to describe the data.

### Key words

- Modifying Tables
  - `Create` Used to make tables
  - `Add` Used to add columns to tables
  - `Drop` Used to remove tables and columns
  - `Rename`/`Retype` Used to change the name or type of tables columns and primaries
- Modifying Records
  - `Insert` Used to add or update records and fields
  - `Delete` Used to remove records and fields

### Primary Keys

To simplify the spec primary keys are always a single column that can be represented by single felt.

### TypeEvents

- `DeclareType`: Declare a new type with a given name and structure.

```rust
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

- `CreateFieldGroup`: Create a field group (schema) with specified columns.
- `CreateTable`: Create a new table with a given name.
- `CreateTableWithColumns`: Create or update a table with a given name and columns.
- `CreateTableFromClassHash`: Create or update a table from a class hash.
- `RenameTable`: Rename an existing table.
- `DropTable`: Drop an existing table.

```rust
/// Field group management:
/// - id: felt252 - Unique identifier for the field group.
/// - columns: Span<felt252> - List of column IDs included in the field group.

struct CreateFieldGroup {
    #[key]
    id: felt252,
    columns: Span<felt252>,
}

/// Table management:
/// - id: felt252 - Unique identifier for the table.
/// - name: ByteArray - Name of the table.
/// - primary: PrimaryDef - Definition of the primary key field.
/// - columns: Span<ColumnDef> - Definitions of the columns in the table.
/// - class_hash: felt252 - Class hash to derive schema from.

struct CreateTable {
    #[key]
    id: felt252,
    name: ByteArray,
    attributes: Span<Attribute>,
    primary: PrimaryDef,
}

struct CreateTableWithColumns {
    #[key]
    id: felt252,
    name: ByteArray,
    attributes: Span<Attribute>,
    primary: PrimaryDef,
    columns: Span<ColumnDef>,
}

struct CreateTableFromClassHash {
    #[key]
    id: felt252,
    name: ByteArray,
    class_hash: felt252,
}

struct RenameTable {
    #[key]
    id: felt252,
    name: ByteArray,
}

struct DropTable {
    #[key]
    id: felt252,
}
```

#### Index Management Events:

- `CreateIndex`: Create a new index on a table.
- `DropIndex`: Drop an existing index from a table.

```rust
/// table: felt252 - Unique identifier for the table.
/// id: felt252 - Unique identifier for the index.
/// name: ByteArray - Name of the index.
/// columns: Span<felt252> - Column IDs included in the index.

struct CreateIndex {
    #[key]
    table: felt252,
    #[key]
    id: felt252,
    name: ByteArray,
    columns: Span<felt252>,
}

struct DropIndex {
    #[key]
    table: felt252,
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
    type_def: PrimaryTypeDef,
    attributes: Span<Attribute>,
}

```

#### Column Management Events:

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
/// id: felt252 - Unique identifier for the column.
/// name: ByteArray - Name of the column.
/// attributes: Span<Attribute> - Attributes of the column.
/// type_def: TypeDef - Type definition of the column.

struct AddColumn {
    #[key]
    table: felt252,
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
    id: felt252,
    name: ByteArray,
}

struct RenameColumns {
    #[key]
    table: felt252,
    /// columns: Pairs of column ids and their new names.
    columns: Span<IdName>,
}

struct RetypeColumn {
    #[key]
    table: felt252,
    #[key]
    id: felt252,
    attributes: Span<Attribute>,
    type_def: TypeDef,
}

struct RetypeColumns {
    #[key]
    table: felt252,
    /// columns: column ids to retype with their new type defs and attributes
    columns: Span<IdTypeAttributes>,
}

struct IdTypeAttributes {
    id: felt252,
    type_def: TypeDef,
    attributes: Span<Attribute>,
}

struct DropColumn {
    #[key]
    table: felt252,
    #[key]
    id: felt252,
}

struct DropColumns {
    #[key]
    table: felt252,
    /// ids: column ids to drop
    ids: Span<felt252>,
}
```

#### Record Manipulation Events:

- `InsertRecord`: Insert or update a record in a table.
- `InsertRecords`: Insert or update multiple records in a table.
- `InsertField`: Insert or update a field in a record.
- `InsertFields`: Insert or update multiple fields in a record.
- `InsertsField`: Insert or update a field in multiple records.
- `InsertsFields`: Insert or update multiple fields in multiple records.
- `InsertFieldGroup`: Insert or update a group of fields in a record.
- `InsertFieldGroups`: Insert or update multiple groups of fields in a record.
- `InsertsFieldGroup`: Insert or update a group of fields in multiple records.
- `InsertsFieldGroups`: Insert or update multiple groups of fields in multiple records.
- `DeleteRecord`: Drop an existing record from a table.
- `DeleteRecords`: Drop multiple existing records from a table.
- `DeleteField`: Drop an existing field from a record.
- `DeleteFields`: Drop multiple existing fields from a record.
- `DeletesField`: Drop an existing field from multiple records.
- `DeletesFields`: Drop multiple existing fields from multiple records.
- `DeleteFieldGroup`: Drop a group of fields from a record.
- `DeleteFieldGroups`: Drop multiple groups of fields from a record.
- `DeletesFieldGroup`: Drop a group of fields from multiple records.
- `DeletesFieldGroups`: Drop multiple groups of fields from multiple records.

```rust
/// Database values common fields
///
/// - table - Table ID.
/// - record/records - Record ID.
/// - column/columns - Column ID.
/// - data - Serialised data being set.
/// - records_data - Pairs of Record IDs and their serialised data being set.
/// - group/groups - Field group ID.

struct IdData {
    id: felt252,
    data: Span<felt252>,
}

struct IdName {
    id: felt252,
    name: ByteArray,
}


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
    records_data: Span<IdData>,
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


struct InsertsField {
    #[key]
    table: felt252,
    #[key]
    column: felt252,
    records_data: Span<IdData>,
}


struct InsertsFields {
    #[key]
    table: felt252,
    columns: Span<felt252>,
    records_data: Span<IdData>,
}

struct InsertFieldGroup {
    #[key]
    table: felt252,
    #[key]
    record: felt252,
    #[key]
    group: felt252,
    data: Span<felt252>,
}


struct InsertFieldGroups {
    #[key]
    table: felt252,
    #[key]
    record: felt252,
    groups: Span<felt252>,
    data: Span<felt252>,
}

struct InsertsFieldGroup {
    #[key]
    table: felt252,
    #[key]
    group: felt252,
    records_data: Span<IdData>,
}


struct InsertsFieldGroups {
    #[key]
    table: felt252,
    groups: Span<felt252>,
    records_data: Span<IdData>,
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
    record: felt252,
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

struct DeleteFieldGroup {
    #[key]
    table: felt252,
    #[key]
    record: felt252,
    #[key]
    group: felt252,
}
struct DeleteFieldGroups {
    #[key]
    table: felt252,
    #[key]
    record: felt252,
    groups: Span<felt252>,
}

struct DeletesFieldGroup {
    #[key]
    table: felt252,
    #[key]
    group: felt252,
    records: Span<felt252>,
}

struct DeletesFieldGroups {
    #[key]
    table: felt252,
    records: Span<felt252>,
    groups: Span<felt252>,
}
```

### Variable Events

These events are for values that don't fit into the table/record model, such as global variables or configuration settings.

- `RegisterVariable`: Register a new variable with value.
- `DeclareVariable`: Register a new variable with a given name and type.
- `SetVariable`: Set the value of an existing variable.
- `RenameVariable`: Rename an existing variable.
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
}

struct DeclareVariable {
    #[key]
    id: felt252,
    name: ByteArray,
    type_def: TypeDef,
    data: Span<felt252>,
}

struct SetVariable {
    #[key]
    id: felt252,
    data: Span<felt252>,
}

struct RenameVariable {
    #[key]
    id: felt252,
    name: ByteArray,
}

struct DeleteVariable {
    #[key]
    id: felt252,
}

```

### Type Definitions

The following data structures are used to describe the types and schemas of cairo structures.

`TypeDef`: An enum to represent what the value should be decoded as. The values are:
| Variant | Description | Selector|
|----------|-------------|-----------|
|`None`| None type e.g. `()` or an empty enum.| 0
|`Felt252`| Base [field element](https://docs.starknet.io/build/corelib/core-felt252) in cairo | 'felt252'
|`Bytes31`| 31 bytes packed into a felt252.| 'bytes31'
|`Bytes31E`| 31 bytes packed into a felt252 with encoding.| 'bytes31e'
|`ShortUtf8`| A 31 byte UTF-8 string.| 'ShortUtf8'
|`Bool`| A boolean value (true or false).| 'bool'
|`U8`| An unsigned 8-bit integer.| 'u8'
|`U16`| An unsigned 16-bit integer.| 'u16'
|`U32`| An unsigned 32-bit integer.| 'u32'
|`U64`| An unsigned 64-bit integer.| 'u64'
|`U128`| An unsigned 128-bit integer.| 'u128'
|`U256`| An unsigned 256-bit integer.| 'u256'
|`U512`| An unsigned 512-bit integer.| 'u512'
|`I8`| A signed 8-bit integer.| 'i8'
|`I16`| A signed 16-bit integer.| 'i16'
|`I32`| A signed 32-bit integer.| 'i32'
|`I64`| A signed 64-bit integer.| 'i64'
|`I128`| A signed 128-bit integer.| 'i128'
|`ClassHash`| An identifier for a contract class.| 'ClassHash'
|`ContractAddress`| An address of a contract on StarkNet.| 'ContractAddress'
|`EthAddress`| An Ethereum address.| 'EthAddress'
|`StorageAddress`| An address of a storage location of a contract.| 'StorageAddress'
|`StorageBaseAddress`| An address of a storage base location of a contract.| 'StorageBaseAddress'
|`ByteArray`| A byte array (An array of bytes31 with a pending word).| 'ByteArray'
|`Utf8String`| A UTF-8 string.| 'Utf8String'
|`ByteArrayE`| A byte array with encoding (An array of bytes31 with a pending word).| 'ByteArrayE'
|`Tuple`| A tuple | 'Tuple'
|`Array`| Variable sized array | 'Array'
|`FixedArray`| Compile-time sized array| 'FixedArray'
|`Felt252Dict`| A dictionary (A mapping from felt252 keys to values).| 'Felt252Dict'
|`Struct`| Custom struct| 'struct'
|`Enum`| Rust style enum | 'enum'
|`Option`| Option type - Either `Some` with a value or `None`| 'Option'
|`Result`| Result type - Either `Ok` or `Err` each with there own type| 'Result'
|`Nullable`| Nullable type - Either a value or `Null`| 'Nullable'
|`Ref`| A reference type (A type that refers to another value).| 'ref'
|`Custom`| A custom type (A user-defined data structure and decoding).| 'custom'

```rust
enum TypeDef {
    #[default]
    None,
    Felt252,
    ShortUtf8,
    Bytes31,
    Bytes31E: ByteArray,
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
    Utf8String,
    ByteArrayE: ByteArray,
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
    Custom: ByteArray,
}
```

> [!IMPORTANT] > `Box<T>` are used to wrap types that also contain TypeDefs as the compiler can't take recursive types.

For primary keys the `PrimaryTypeDef` which contains a subset of `TypeDef` that can be used as primary keys.

```rust
enum PrimaryTypeDef {
    #[default]
    Felt252,
    ShortUtf8,
    Bytes31,
    Bytes31E: ByteArray,
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
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
}
```

These are the types that can be used as primary keys as they can be represented by a single felt252 value.

#### Attributes

An attribute is a key-value pair that can be attached to various type definitions to provide additional metadata or information about the type such as encoding.

`Attribute`: Defines an attribute with the fields:

- `name`: The name/identifier of the attribute as a ByteArray.
- `data`: Optional data associated with the attribute as a ByteArray.

```rust
struct Attribute {
    name: ByteArray,
    data: Option<ByteArray>,
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
- `type_def`: The type definition of the field.
- `attributes`: A span of attributes associated with the field.

```rust
struct ColumnDef {
    id: felt252,
    name: ByteArray,
    attributes: Span<Attribute>,
    type_def: TypeDef,
}

struct PrimaryDef {
    name: ByteArray,
    attributes: Span<Attribute>,
    type_def: PrimaryTypeDef,
}
```

### Traits

Although to use this spec directly you can just emit the events with the correct data structures, to make it easier to work with there are some traits provided to generate TypeDefs and serialize data. These are implemented for all core cairo and starknet types and there are macros provided to generate them for custom structs and enums.

### Introspect Trait

The Introspect trait is used to describe types so they can be reconstructed by another system.

Methods:

- `type_def() -> TypeDef`: Returns the type definition of the implementing type.
- `child_defs() -> Array<(felt252, TypeDef)>`: Returns any types and their ids that are referenced (including recursively) by this type.
- `hash() -> felt252`: Returns a unique hash of the type definition.

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

### ISerde Trait

The ISerde trait is used to serialize data to a span of felt252 values which can then be decoded using the corresponding `TypeDef` by another system. Its`iserialize` mirrors `serialize` from the Serde trait for the most part but is implemented separately to allow for optimizations specific to either ISerde or Serde without causing conflicts. The only diffrence is the parsing of ByteArrays.

#### ISerde ByteArray Serialization

To reduce event size ByteArrays are serialized like so:

- No length prefix
- The 0 bit in the 31st byte is used to indicate if it is the last felt252 in the ByteArray; 0 = more to follow, 1 = is last.
- The 1 bit in the 31st byte is use to show if the felt is a partial word (less than 31 bytes). 0 = full 31 bytes, 1 = 30 bytes or less. If the felt is a partial word the 30th byte contains the length of the valid bytes in that felt.

This means a ByteArray with 31 Bytes or less will only take up a single felt252 with no felts used for total felts or pending word size.

As all data emitted with ISerde is expected to be decoded using the corresponding TypeDef by another system and wont be represented in the ABI, this shouldn't cause any issues with compatibility.

```rust
pub trait ISerde<T> {
    fn iserialize(self: @T, ref output: Array<felt252>);
    fn iserialize_inline(
        self: @T,
    ) -> Span<
        felt252,
    > {
        let mut data: Array<felt252> = Default::default();
        Self::iserialize(self, ref data);
        data.span()
    }
    fn ideserialize(ref serialized: Span<felt252>) -> Option<T>;
}
```

## Implementation

This SNIP provides a spec for encoding types, corresponding data and events to describe what to do with that data. It is expected that most of these will be hidden from most developers behind higher level frameworks and libraries both on the contract, indexer and client side. Its not expected that all these tool will use all the events but each library/framework should clearly document which events they use and how they interpret them. Optimization can be made by limiting certain features e.g. not allowing upgrades of database to allow for more efficient storage and querying.

## Security Considerations

As this SNIP primarily defines events and data structures for describing on-chain data, it does not introduce new security risks. However, implementers should ensure that the calls to allow emitting these events are properly authorized to prevent unauthorized modifications to the described data structures.

## History

## Copyright

Copyright and related rights waived via [MIT](../LICENSE).

```

```
