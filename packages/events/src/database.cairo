use introspect_types::{ColumnDef, TypeDef};


#[derive(Drop, Serde, starknet::Event)]
pub enum DatabaseEvents {
    CreateTable: CreateTable,
    CreateTableWithFields: CreateTableWithFields,
    CreateTableWithSchema: CreateTableWithSchema,
    RenameTable: RenameTable,
    DropTable: DropTable,
    AddTableColumn: AddTableColumn,
    AddTableColumns: AddTableColumns,
    DropColumn: DropColumn,
    RenameColumn: RenameColumn,
    RenameColumns: RenameColumns,
    DropColumns: DropColumns,
    InsertRecordField: InsertRecordField,
    InsertRecord: InsertRecord,
    InsertRecords: InsertRecords,
    InsertRecordFields: InsertRecordFields,
    InsertRecordsField: InsertRecordsField,
    InsertRecordsFields: InsertRecordsFields,
    InsertRecordDataFromSchema: InsertRecordDataFromSchema,
    InsertRecordsDataFromSchema: InsertRecordsDataFromSchema,
    DropValue: DropValue,
    DropRecord: DropRecord,
    DropRecords: DropRecords,
    DropRecordFields: DropRecordFields,
    DropRecordsField: DropRecordsField,
    DropRecordsFields: DropRecordsFields,
    DropRecordFromSchema: DropRecordFromSchema,
    DropRecordsFromSchema: DropRecordsFromSchema,
}


/// Table createion common fields
///
/// - `id` Table ID.
/// - `name` in table events: Table name, in field events: Table ID the field name.
/// - `fields` in table events: Table fields.
/// - `schema` in table events: Schema ID.

///Emitted when a new table is declared without field definitions.
#[derive(Drop, Serde, starknet::Event)]
pub struct CreateTable {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
}

/// Emitted when a new table is declared with inline field definitions.
#[derive(Drop, Serde, starknet::Event)]
pub struct CreateTableWithFields {
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

///Emitted when a table is undeclared.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropTable {
    #[key]
    pub id: felt252,
}

/// Table column management common fields
///
/// - `table` Table ID the column belongs to.
/// - `name` Column name.
/// - `attrs` Column attributes.
/// - `type_def` Column type.

/// Emitted when a new column is declared for an existing table.
#[derive(Drop, Serde, starknet::Event)]
pub struct AddTableColumn {
    #[key]
    pub table: felt252,
    pub id: felt252,
    pub name: ByteArray,
    pub attrs: Span<felt252>,
    pub type_def: TypeDef,
}

/// Emitted when multiple new columns are declared for an existing table.
#[derive(Drop, Serde, starknet::Event)]
pub struct AddTableColumns {
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
#[derive(Drop, Serde, starknet::Event)]
pub struct RenameColumns {
    #[key]
    pub table: felt252,
    pub columns_names: Span<(felt252, ByteArray)>,
}

/// Emitted when a column is undeclared from a table.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropColumn {
    #[key]
    pub table: felt252,
    pub column: felt252,
}

/// Emitted when multiple columns are undeclared from a table.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropColumns {
    #[key]
    pub table: felt252,
    pub columns: Span<felt252>,
}


/// Database values common fields
///
/// - `table` Table ID.
/// - `record`/`records` Record ID.
/// - `field`/`fields` Field selector.
/// - `data` Serialised data being set.
///
#[derive(Drop, Serde, starknet::Event)]
pub struct InsertRecordField {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub field: felt252,
    pub data: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct InsertRecordFields {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub fields: Span<felt252>,
    pub data: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct InsertRecordsField {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    #[key]
    pub field: felt252,
    pub data: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct InsertRecordsFields {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    pub fields: Span<felt252>,
    pub data: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct InsertRecord {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub data: Span<felt252>,
}


#[derive(Drop, Serde, starknet::Event)]
pub struct InsertRecords {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    pub data: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct InsertRecordDataFromSchema {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub schema: felt252,
    pub data: Span<felt252>,
}


#[derive(Drop, Serde, starknet::Event)]
pub struct InsertRecordsDataFromSchema {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    #[key]
    pub schema: felt252,
    pub data: Span<felt252>,
}


#[derive(Drop, Serde, starknet::Event)]
pub struct DropValue {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub field: felt252,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DropRecord {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DropRecords {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DropRecordFields {
    #[key]
    pub table: felt252,
    #[key]
    pub row: felt252,
    pub fields: Span<felt252>,
}


#[derive(Drop, Serde, starknet::Event)]
pub struct DropRecordsField {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    #[key]
    pub column: felt252,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DropRecordsFields {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    pub fields: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DropRecordFromSchema {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub schema: felt252,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DropRecordsFromSchema {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    #[key]
    pub schema: felt252,
}

