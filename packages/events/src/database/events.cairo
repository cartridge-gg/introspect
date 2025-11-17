use introspect_types::{Attribute, ColumnDef, IdData, PrimaryDef, PrimaryTypeDef, TypeDef};

pub mod selectors {
    pub const CreateFieldGroup: felt252 = selector!("CreateFieldGroup");
    pub const CreateTable: felt252 = selector!("CreateTable");
    pub const CreateTableWithColumns: felt252 = selector!("CreateTableWithColumns");
    pub const CreateTableFromClassHash: felt252 = selector!("CreateTableFromClassHash");
    pub const RenameTable: felt252 = selector!("RenameTable");
    pub const DropTable: felt252 = selector!("DropTable");
    pub const RenamePrimary: felt252 = selector!("RenamePrimary");
    pub const RetypePrimary: felt252 = selector!("RetypePrimary");
    pub const AddColumn: felt252 = selector!("AddColumn");
    pub const AddColumns: felt252 = selector!("AddColumns");
    pub const RenameColumn: felt252 = selector!("RenameColumn");
    pub const RenameColumns: felt252 = selector!("RenameColumns");
    pub const RetypeColumn: felt252 = selector!("RetypeColumn");
    pub const RetypeColumns: felt252 = selector!("RetypeColumns");
    pub const DropColumn: felt252 = selector!("DropColumn");
    pub const DropColumns: felt252 = selector!("DropColumns");
    pub const InsertRecord: felt252 = selector!("InsertRecord");
    pub const InsertRecords: felt252 = selector!("InsertRecords");
    pub const InsertField: felt252 = selector!("InsertField");
    pub const InsertFields: felt252 = selector!("InsertFields");
    pub const InsertsField: felt252 = selector!("InsertsField");
    pub const InsertsFields: felt252 = selector!("InsertsFields");
    pub const InsertFieldGroup: felt252 = selector!("InsertFieldGroup");
    pub const InsertFieldGroups: felt252 = selector!("InsertFieldGroups");
    pub const InsertsFieldGroup: felt252 = selector!("InsertsFieldGroup");
    pub const InsertsFieldGroups: felt252 = selector!("InsertsFieldGroups");
    pub const DeleteRecord: felt252 = selector!("DeleteRecord");
    pub const DeleteRecords: felt252 = selector!("DeleteRecords");
    pub const DeleteField: felt252 = selector!("DeleteField");
    pub const DeleteFields: felt252 = selector!("DeleteFields");
    pub const DeletesField: felt252 = selector!("DeletesField");
    pub const DeletesFields: felt252 = selector!("DeletesFields");
    pub const DeleteSchema: felt252 = selector!("DeleteSchema");
    pub const DeletesSchema: felt252 = selector!("DeletesSchema");
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


/// Emitted when a new field group (schema) is created.
/// - id: felt252 - Unique identifier for the field group.
/// - columns: Span<felt252> - List of column IDs included in the field group
#[derive(Drop, Serde, starknet::Event)]
pub struct CreateFieldGroup {
    #[key]
    pub id: felt252,
    pub columns: Span<felt252>,
}


/// Table management events
/// - id: felt252 - Unique identifier for the table.
/// - name: ByteArray - Name of the table.
/// - attributes: Span<Attribute> - Attributes of the table.
/// - columns: Span<ColumnDef> - Definitions of the columns in the table.
/// - class_hash: ClassHash - Class hash to derive schema from.

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

#[derive(Drop, Serde, starknet::Event)]
pub struct CreateTableFromClassHash {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub class_hash: felt252,
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
    pub id: felt252,
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
    pub id: felt252,
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
    pub ids: Span<felt252>,
}


/// Record management events
/// - table - Table ID.
/// - record/records - Record ID.
/// - column/columns - Column ID.
/// - data - Serialised data being set.
/// - records_data - Pairs of Record IDs and their serialised data being set.
/// - group - Field group (schema) ID.

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
pub struct InsertFieldGroup {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub group: felt252,
    pub data: Span<felt252>,
}


/// Insert multiple schemas into a record.
#[derive(Drop, Serde, starknet::Event)]
pub struct InsertFieldGroups {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub groups: Span<felt252>,
    pub data: Span<felt252>,
}

/// Insert multiple records into a table using a schema.
#[derive(Drop, Serde, starknet::Event)]
pub struct InsertsFieldGroup {
    #[key]
    pub table: felt252,
    #[key]
    pub group: felt252,
    pub records_data: Span<IdData>,
}


/// Insert multiple schemas into a record.
#[derive(Drop, Serde, starknet::Event)]
pub struct InsertsFieldGroups {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub groups: Span<felt252>,
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
pub struct DeleteFieldGroup {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub group: felt252,
}
/// Remove multiple fields from a record.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeleteFieldGroups {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub groups: Span<felt252>,
}

/// Remove multiple fields from multiple records.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeletesFieldGroup {
    #[key]
    pub table: felt252,
    #[key]
    pub group: felt252,
    pub records: Span<felt252>,
}

/// Remove multiple fields from multiple records.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeletesFieldGroups {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    pub groups: Span<felt252>,
}
