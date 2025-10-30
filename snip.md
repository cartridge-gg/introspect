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
/// columns: Span<ColumnDef> - Definitions of the columns in the table.
/// schema: felt252 - Identifier of the schema used by the table.

struct CreateTable {
    #[key]
    id: felt252,
    name: ByteArray,
}

struct CreateTableWithColumns {
    #[key]
    id: felt252,
    name: ByteArray,
    columns: Span<ColumnDef>,
}

struct CreateTableWithSchema {
    #[key]
    id: felt252,
    name: ByteArray,
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
/// column: felt252 - Unique identifier for the column.
/// name: ByteArray - Name of the column.
/// attrs: Span<felt252> - Attributes of the column.
/// type_def: TypeDef - Type definition of the column.

struct AddTableColumn {
    #[key]
    table: felt252,
    /// id: Unique identifier for the column.
    #[key]
    id: felt252,
    name: ByteArray,
    attrs: Span<felt252>,
    type_def: TypeDef,
}

struct AddTableColumns {
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
    type_def: TypeDef,
}

struct RetypeColumns {
    #[key]
    table: felt252,
    /// columns: Pairs of column ids and their new type definitions.
    columns: Span<(felt252, TypeDef)>,
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

- `InsertRecordField`: Insert or update a field in a record.
- `InsertRecord`: Insert or update a record in a table.
- `InsertRecords`: Insert or update multiple records in a table.
- `InsertRecordFields`: Insert or update multiple fields in a record.
- `InsertRecordsField`: Insert or update a field in multiple records.
- `InsertRecordsFields`: Insert or update multiple fields in multiple records.
- `InsertRecordDataFromSchema`: Insert or update a record in a table using a schema.
- `InsertRecordsDataFromSchema`: Insert or update multiple records in a table using a schema.
- `DropValue`: Drop an existing value from a record.
- `DropRecord`: Drop an existing record from a table.
- `DropRecords`: Drop multiple existing records from a table.
- `DropRecordFields`: Drop multiple existing fields from a record.
- `DropRecordsField`: Drop an existing field from multiple records.
- `DropRecordsFields`: Drop multiple existing fields from multiple records.
- `DropRecordFromSchema`: Drop an existing record from a schema.
- `DropRecordsFromSchema`: Drop multiple existing records from a schema.

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



struct DropRecord {
    #[key]
    table: felt252,
    #[key]
    record: felt252,
}


struct DropRecords {
    #[key]
    table: felt252,
    records: Span<felt252>,
}

struct DropField {
    #[key]
    table: felt252,
    #[key]
    record: felt252,
    #[key]
    column: felt252,
}


struct DropFields {
    #[key]
    table: felt252,
    #[key]
    row: felt252,
    columns: Span<felt252>,
}

struct DropRecordsField {
    #[key]
    table: felt252,
    #[key]
    column: felt252,
    records: Span<felt252>,

}

struct DropRecordsFields {
    #[key]
    table: felt252,
    records: Span<felt252>,
    columns: Span<felt252>,
}

struct DropSchema {
    #[key]
    table: felt252,
    #[key]
    record: felt252,
    #[key]
    schema: felt252,
}


struct DropRecordsSchema {
    #[key]
    table: felt252,
    #[key]
    schema: felt252,
    records: Span<felt252>,
}
```

### Variable Events

These events are for values that don't fit into the table/record model, such as global variables or configuration settings.

- `RegisterVariable`: Register a new variable with a given name and type.
- `SetVariable`: Set the value of an existing variable.
- `DeclareVariable`: Register a new variable with value.
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


struct DeleteVariable {
    #[key]
    id: felt252,
}

```

### Type Definitions

The following data structures are used to describe the types and schemas of cairo structures.

- `TypeDef`: An enum to represent what the value should be decoded as. The values are:
  - `None`: Used to represent an absence or empty type. (e.g., and empty tuple or void type enum)
  - `Felt252`: A 252-bit field element.
  - `Bool`: A boolean value.
  - `U8`: An 8-bit unsigned integer.
  - `U16`: A 16-bit unsigned integer.
  - `U32`: A 32-bit unsigned integer.
  - `U64`: A 64-bit unsigned integer.
  - `U128`: A 128-bit unsigned integer.
  - `U256`: A 256-bit unsigned integer.
  - `I8`: An 8-bit signed integer.
  - `I16`: A 16-bit signed integer.
  - `I32`: A 32-bit signed integer.
  - `I64`: A 64-bit signed integer.
  - `I128`: A 128-bit signed integer.
  - `USize`: An unsigned integer of the size of a pointer.
  - `ShortString`: An UTF-8 encoded single felt.
  - `ClassHash`: A Starknet class hash.
  - `ContractAddress`: A Starknet contract address.
  - `EthAddress`: An Ethereum address.
  - `ByteArray`: A byte array.
  - `Tuple`: A fixed-size tuple.
  - `Array`: A dynamic array.
  - `FixedArray`: A fixed-size array.
  - `Felt252Dict`: A dictionary with Felt252 keys.
  - `Struct`: A user-defined struct.
  - `Enum`: A user-defined enum.
  - `Ref`: A reference to a predefined type.
  - `Encoded`: Data encoded in a specific format.
  - `Custom`: A user defined custom decoding.
  - `Option`: An Option type can can be a value or None.
  - `Result`: A Result type that can either have an Ok or Err value.
  - `Nullable`: A Nullable type that can be a value or Null.
  - `DynamicEncoding`: Data encoded with an included encoding.

## Security Considerations

As this SNIP primarily defines events and data structures for describing on-chain data, it does not introduce new security risks. However, implementers should ensure that the calls to allow emitting these events are properly authorized to prevent unauthorized modifications to the described data structures.

## History

## Copyright

Copyright and related rights waived via [MIT](../LICENSE).
