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

## Implementation

The standard consists of two main parts the events and data structures used to describe the data.

### TypeEvents

- `DeclareSchema`: Declare a new schema with a given name and structure.
- `DeclareType`: Declare a new type with a given name and structure.

### Database Events

A set of events that can be emitted by contracts to describe the structure of data stored on chain. These events include:

- `CreateTable`: Create a new table with a given name.
- `CreateTableWithColumns`: Create or update a table with a given name and columns.
- `CreateTableWithSchema`: Create or update a table with a given name and schema.
- `RenameTable`: Rename an existing table.
- `DropTable`: Drop an existing table.
- `AddColumn`: Add a new column to a table.
- `AddColumns`: Add multiple new columns to a table.
- `DropColumn`: Drop an existing column from a table.
- `RenameColumn`: Rename an existing column in a table.
- `DropColumns`: Drop multiple existing columns from a table.
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

### Variable Events

These events are for values that don't fit into the table/record model, such as global variables or configuration settings.

- `RegisterVariable`: Register a new variable with a given name and type.
- `SetVariable`: Set the value of an existing variable.
- `DeclareVariable`: Register a new variable with value.
- `DeleteVariable`: Delete an existing variable.

## Specification

## Security Considerations

As this SNIP primarily defines events and data structures for describing on-chain data, it does not introduce new security risks. However, implementers should ensure that the calls to allow emitting these events are properly authorized to prevent unauthorized modifications to the described data structures.

## History

## Copyright

Copyright and related rights waived via [MIT](../LICENSE).
