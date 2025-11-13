# Database Events

These consist of events for table, column and record manipulation.

## Table Management Events

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

## Primary Key Management Events

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

## Column Management Events

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

## Record Manipulation Events

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
