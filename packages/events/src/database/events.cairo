use introspect_types::{Attribute, ColumnDef, IdData, PrimaryDef, PrimaryTypeDef, TypeDef};


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
    pub attributes: Span<Attribute>,
    pub primary: PrimaryDef,
}

/// Emitted when a new table is created with specified columns.
#[derive(Drop, Serde, starknet::Event)]
pub struct CreateTableWithColumns {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub primary: PrimaryDef,
    pub columns: Span<ColumnDef>,
}


/// Declares a table using a pre-defined schema.
#[derive(Drop, Serde, starknet::Event)]
pub struct CreateTableWithSchema {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub primary: PrimaryDef,
    pub schema: felt252,
}


///Emitted when a table is renamed.
#[derive(Drop, Serde, starknet::Event)]
pub struct RenameTable {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
}

///Emitted when a table is dropped.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropTable {
    #[key]
    pub id: felt252,
}

/// Primary key management events
/// - table: felt252 - Unique identifier for the table.
/// - name: ByteArray - Name of the primary key field.
/// - attributes: Span<Attribute> - Attributes of the primary key field.
/// - type_def: TypeDef - Type definition of the primary key field.

/// Emitted when the primary key field is renamed.
#[derive(Drop, Serde, starknet::Event)]
pub struct RenamePrimary {
    #[key]
    pub table: felt252,
    pub name: ByteArray,
}

/// Emitted when the primary key field is retyped.
#[derive(Drop, Serde, starknet::Event)]
pub struct RetypePrimary {
    #[key]
    pub table: felt252,
    pub attributes: Span<Attribute>,
    pub type_def: PrimaryTypeDef,
}

/// Column management events
/// - table: felt252 - Unique identifier for the table.
/// - column: felt252 - Unique identifier for the column.
/// - name: ByteArray - Name of the column.
/// - attributes: Span<Attribute> - Attributes of the column.
/// - type_def: TypeDef - Type definition of the column.

#[derive(Drop, Serde, starknet::Event)]
pub struct AddColumn {
    #[key]
    pub table: felt252,
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
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
    #[key]
    pub column: felt252,
    pub name: ByteArray,
}


// Emitted when a columns is renamed in a table.
/// - columns: List of (column ID, new name) pairs.
#[derive(Drop, Serde, starknet::Event)]
pub struct RenameColumns {
    #[key]
    pub table: felt252,
    pub columns: Span<IdName>,
}

/// Remove a single record from a table.
/// Emitted when a column is retyped in a table.
#[derive(Drop, Serde, starknet::Event)]
pub struct RetypeColumn {
    #[key]
    pub table: felt252,
    #[key]
    pub column: felt252,
    pub attributes: Span<Attribute>,
    pub type_def: TypeDef,
}

/// Emitted when multiple columns are retyped in a table.
/// - columns: List of (column ID, new TypeDef) pairs.
#[derive(Drop, Serde, starknet::Event)]
pub struct RetypeColumns {
    #[key]
    pub table: felt252,
    pub columns: Span<IdTypeAttributes>,
}

/// Emitted when a column is undeclared from a table.
#[derive(Drop, Serde, starknet::Event)]
pub struct DropColumn {
    #[key]
    pub table: felt252,
    #[key]
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
    pub records_data: Span<IdData>,
}


//// Insert a single field into a record.
#[derive(Drop, Serde, starknet::Event)]
pub struct InsertField {
    #[key]
    pub table: felt252,
    #[key]
    pub column: felt252,
    #[key]
    pub record: felt252,
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
pub struct InsertsField {
    #[key]
    pub table: felt252,
    #[key]
    pub column: felt252,
    pub records_data: Span<IdData>,
}

/// Insert multiple fields into multiple records.
#[derive(Drop, Serde, starknet::Event)]
pub struct InsertsFields {
    #[key]
    pub table: felt252,
    pub columns: Span<felt252>,
    pub records_data: Span<IdData>,
}

/// Insert a schema into a record.
#[derive(Drop, Serde, starknet::Event)]
pub struct InsertColumnGroup {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub group: felt252,
    pub data: Span<felt252>,
}

/// Insert multiple records into a table using a schema.
#[derive(Drop, Serde, starknet::Event)]
pub struct InsertsColumnGroup {
    #[key]
    pub table: felt252,
    #[key]
    pub group: felt252,
    pub records_data: Span<IdData>,
}

/// Remove a single record from a table.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeleteRecord {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
}

/// Remove multiple records from a table.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeleteRecords {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
}


/// Remove a single field from a record.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeleteField {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub column: felt252,
}


/// Remove multiple fields from a record.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeleteFields {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub columns: Span<felt252>,
}


/// Remove a single field from multiple records.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeletesField {
    #[key]
    pub table: felt252,
    #[key]
    pub column: felt252,
    pub records: Span<felt252>,
}

/// Remove multiple fields from multiple records.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeletesFields {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    pub columns: Span<felt252>,
}


/// Remove a schema from a record.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeleteSchema {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub schema: felt252,
}

/// Remove multiple fields from multiple records.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeletesSchema {
    #[key]
    pub table: felt252,
    #[key]
    pub schema: felt252,
    pub records: Span<felt252>,
}


#[derive(Drop, Serde)]
pub struct IdName {
    pub id: felt252,
    pub name: ByteArray,
}


#[derive(Drop, Serde)]
pub struct IdTypeAttributes {
    pub id: felt252,
    pub attributes: Span<Attribute>,
    pub type_def: TypeDef,
}


#[derive(Drop, Serde, starknet::Event)]
pub struct CreateColumnGroup {
    pub id: felt252,
    pub columns: Span<felt252>,
}
