# Generic Events

This package contains generic cairo events that allows indexers to easily read and parse data from the chain without the need for custom events and indexer logic.

The two main types of events are Database events and Variable events.

## Database Events

Database events allow for declaring tables, fields and schemas and emitting data to these tables. This gives an easy way for indexed data to be structured and queried off chain.

A contract does not need to use all these events but any indexer using this standard should be able to read any contract using any of these events.

### Events

- DeclareTable: Declares a new table from the fields, name and ID.
- DeclareTableWithSchema: Declares a new table from a predefined schema.
- DeclareField: Declares an additional field in an already declared table.
- DeclareFields: Declares additional fields in an already declared table.
- DeclareSchema: Declares a new schema.
- UpdateValue: Updates a single value (single row and column) in a table.
- UpdateRecordFields: Updates multiple fields for a single record.
- UpdateFieldRecords: Updates a single field across multiple records.
- UpdateRecordsFields: Updates multiple fields across multiple records (row-major order).
- UpdateRecordFromSchema: Updates a record using a predefined schema layout.
- UpdateRecordsFromSchema: Updates multiple records using a predefined schema layout.

## Variable Events

Variable events allow for emitting data that is not structured in a table format such values that only occur once like totals or configuration values.

### Events

- DeclareVariable: Declares a new variable.
- SetVariable: Sets the value of an existing variable.
- DeleteVariable: Deletes a variable.
