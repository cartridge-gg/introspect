use introspect_types::{ColumnDef, TypeDef};


#[derive(Drop, Serde, starknet::Event)]
pub enum DatabaseEvents {
    CreateTable: CreateTable,
    CreateTableWithColumns: CreateTableWithColumns,
    CreateTableWithSchema: CreateTableWithSchema,
    RenameTable: RenameTable,
    DropTable: DropTable,
    AddColumn: AddTableColumn,
    AddColumns: AddTableColumns,
    RenameColumn: RenameColumn,
    RenameColumns: RenameColumns,
    RetypeColumn: RetypeColumn,
    RetypeColumns: RetypeColumns,
    DropColumn: DropColumn,
    DropColumns: DropColumns,
    InsertRecord: InsertRecord,
    InsertRecords: InsertRecords,
    InsertField: InsertField,
    InsertFields: InsertFields,
    InsertRecordsField: InsertRecordsField,
    InsertRecordsFields: InsertRecordsFields,
    InsertSchema: InsertSchema,
    InsertRecordsSchema: InsertRecordsSchema,
    DropRecord: DropRecord,
    DropRecords: DropRecords,
    DropField: DropField,
    DropFields: DropFields,
    DropRecordsField: DropRecordsField,
    DropRecordsFields: DropRecordsFields,
    DropSchema: DropSchema,
    DropRecordsSchema: DropRecordsSchema,
}


/// Table management events
/// - id: felt252 - Unique identifier for the table.
/// - name: ByteArray - Name of the table.
/// - columns: Span<ColumnDef> - Definitions of the columns in the table.
/// - schema: felt252 - Identifier of the schema used by the table.

/// Emitted when a new table is created.
#[derive(Drop, Serde, starknet::Event)]
pub struct CreateTable {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
}

/// Emitted when a new table is created with specified columns.
#[derive(Drop, Serde, starknet::Event)]
pub struct CreateTableWithColumns {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub columns: Span<ColumnDef>,
}


/// Declares a table using a pre-defined schema.
#[derive(Drop, Serde, starknet::Event)]
pub struct CreateTableWithSchema {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub schema: felt252,
}


///Emitted when a table is renamed.
#[derive(Drop, Serde, starknet::Event)]
pub struct RenameTable {
    #[key]
    pub id: felt252,
    pub new_name: ByteArray,
}

///Emitted when a table is dropped.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropTable {
    #[key]
    pub id: felt252,
}

/// Column management events
/// - table: felt252 - Unique identifier for the table.
/// - column: felt252 - Unique identifier for the column.
/// - name: ByteArray - Name of the column.
/// - attrs: Span<felt252> - Attributes of the column.
/// - type_def: TypeDef - Type definition of the column.

#[derive(Drop, Serde, starknet::Event)]
pub struct AddColumn {
    #[key]
    pub table: felt252,
    pub id: felt252,
    pub name: ByteArray,
    pub attrs: Span<felt252>,
    pub type_def: TypeDef,
}

/// Emitted when multiple new columns are declared for an existing table.
/// - columns: Definitions of the columns being added.
#[derive(Drop, Serde, starknet::Event)]
pub struct AddColumns {
    #[key]
    pub table: felt252,
    pub columns: Span<ColumnDef>,
}

// Emitted when a column is renamed in a table.
#[derive(Drop, Serde, starknet::Event)]
pub struct RenameColumn {
    #[key]
    pub table: felt252,
    pub column: felt252,
    pub name: ByteArray,
}


// Emitted when a columns is renamed in a table.
/// - columns: List of (column ID, new name) pairs.
#[derive(Drop, Serde, starknet::Event)]
pub struct RenameColumns {
    #[key]
    pub table: felt252,
    pub columns: Span<(felt252, ByteArray)>,
}


/// Remove a single record from a table.
/// Emitted when a column is retyped in a table.
#[derive(Drop, Serde, starknet::Event)]
pub struct RetypeColumn {
    #[key]
    pub table: felt252,
    pub column: felt252,
    pub type_def: TypeDef,
}

/// Emitted when multiple columns are retyped in a table.
/// - columns: List of (column ID, new TypeDef) pairs.
#[derive(Drop, Serde, starknet::Event)]
pub struct RetypeColumns {
    #[key]
    pub table: felt252,
    pub columns: Span<(felt252, TypeDef)>,
}

/// Emitted when a column is undeclared from a table.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropColumn {
    #[key]
    pub table: felt252,
    pub column: felt252,
}

/// Emitted when multiple columns are undeclared from a table.
/// - columns: List of column IDs being dropped.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropColumns {
    #[key]
    pub table: felt252,
    pub columns: Span<felt252>,
}


/// Record management events
/// - table - Table ID.
/// - record/records - Record ID.
/// - column/columns - Column ID.
/// - data - Serialised data being set.
/// - records_data - Pairs of Record IDs and their serialised data being set.
/// - schema - Schema ID.

#[derive(Drop, Serde, starknet::Event)]
pub struct InsertRecord {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub data: Span<felt252>,
}

/// Remove a record from a table using a schema.
#[derive(Drop, Serde, starknet::Event)]
pub struct InsertRecords {
    #[key]
    pub table: felt252,
    pub records_data: Span<(felt252, Span<felt252>)>,
}


//// Insert a single field into a record.
#[derive(Drop, Serde, starknet::Event)]
pub struct InsertField {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub column: felt252,
    pub data: Span<felt252>,
}

/// Insert multiple fields into a record.
#[derive(Drop, Serde, starknet::Event)]
pub struct InsertFields {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub columns: Span<felt252>,
    pub data: Span<felt252>,
}

/// Insert a single field into multiple records.
#[derive(Drop, Serde, starknet::Event)]
pub struct InsertRecordsField {
    #[key]
    pub table: felt252,
    #[key]
    pub column: felt252,
    pub records_data: Span<(felt252, Span<felt252>)>,
}

/// Insert multiple fields into multiple records.
#[derive(Drop, Serde, starknet::Event)]
pub struct InsertRecordsFields {
    #[key]
    pub table: felt252,
    pub columns: Span<felt252>,
    pub records_data: Span<(felt252, Span<felt252>)>,
}

/// Insert a schema into a record.
#[derive(Drop, Serde, starknet::Event)]
pub struct InsertSchema {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub schema: felt252,
    pub data: Span<felt252>,
}

/// Insert multiple records into a table using a schema.
#[derive(Drop, Serde, starknet::Event)]
pub struct InsertRecordsSchema {
    #[key]
    pub table: felt252,
    #[key]
    pub schema: felt252,
    pub records_data: Span<(felt252, Span<felt252>)>,
}

/// Remove a single record from a table.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropRecord {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
}

/// Remove multiple records from a table.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropRecords {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
}


/// Remove a single field from a record.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropField {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub column: felt252,
}


/// Remove multiple fields from a record.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropFields {
    #[key]
    pub table: felt252,
    #[key]
    pub row: felt252,
    pub columns: Span<felt252>,
}


/// Remove a single field from multiple records.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropRecordsField {
    #[key]
    pub table: felt252,
    #[key]
    pub column: felt252,
    pub records: Span<felt252>,
}

/// Remove multiple fields from multiple records.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropRecordsFields {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    pub columns: Span<felt252>,
}


/// Remove a schema from a record.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropSchema {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub schema: felt252,
}

/// Remove multiple fields from multiple records.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropRecordsSchema {
    #[key]
    pub table: felt252,
    #[key]
    pub schema: felt252,
    pub records: Span<felt252>,
}

