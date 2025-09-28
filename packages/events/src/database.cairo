use introspect_types::Ty;
use introspect_types::ty::Field;


#[derive(Drop, Serde, starknet::Event)]
pub enum DatabaseEvents {
    DeclareTable: DeclareTable,
    DeclareTableWithFields: DeclareTableWithFields,
    DeclareTableWithSchema: DeclareTableWithSchema,
    UndeclareTable: UndeclareTable,
    DeclareTableField: DeclareTableField,
    DeclareTableFields: DeclareTableFields,
    UndeclareField: UndeclareField,
    UndeclareFields: UndeclareFields,
    SetValue: SetValue,
    SetRecord: SetRecord,
    SetRecords: SetRecords,
    SetRecordFields: SetRecordFields,
    SetRecordsField: SetRecordsField,
    SetRecordsFields: SetRecordsFields,
    SetRecordDataFromSchema: SetRecordDataFromSchema,
    SetRecordsDataFromSchema: SetRecordsDataFromSchema,
    DeleteValue: DeleteValue,
    DeleteRecord: DeleteRecord,
    DeleteRecords: DeleteRecords,
    DeleteRecordFields: DeleteRecordFields,
    DeleteRecordsField: DeleteRecordsField,
    DeleteRecordsFields: DeleteRecordsFields,
    DeleteRecordFromSchema: DeleteRecordFromSchema,
    DeleteRecordsFromSchema: DeleteRecordsFromSchema,
}


/// Table declaration common fields
///
/// - `id` Table ID.
/// - `name` in table events: Table name, in field events: Table ID the field name.
/// - `fields` in table events: Table fields.
/// - `schema` in table events: Schema ID.

///Emitted when a new table is declared without field definitions.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeclareTable {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
}

/// Emitted when a new table is declared with inline field definitions.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeclareTableWithFields {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub fields: Span<Field>,
}

/// Declares a table using a pre-defined schema.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeclareTableWithSchema {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub schema: felt252,
}

///Emitted when a table is undeclared.
#[derive(Drop, Serde, starknet::Event)]
pub struct UndeclareTable {
    #[key]
    pub id: felt252,
}

/// Table field management common fields
///
/// - `table` Table ID the field belongs to.
/// - `name` Field name.
/// - `attrs` Field attributes.
/// - `ty` Field type.

/// Emitted when a new field is declared for an existing table.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeclareTableField {
    #[key]
    pub table: felt252,
    pub id: felt252,
    pub name: felt252,
    pub attrs: Span<felt252>,
    pub ty: Ty,
}

/// Emitted when multiple new fields are declared for an existing table.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeclareTableFields {
    #[key]
    pub table: felt252,
    pub fields: Span<Field>,
}

/// Emitted when a field is undeclared from a table.
#[derive(Drop, Serde, starknet::Event)]
pub struct UndeclareField {
    #[key]
    pub table: felt252,
    pub field: felt252,
}

/// Emitted when multiple fields are undeclared from a table.
#[derive(Drop, Serde, starknet::Event)]
pub struct UndeclareFields {
    #[key]
    pub table: felt252,
    pub fields: Span<felt252>,
}


/// Database values common fields
///
/// - `table` Table ID.
/// - `record`/`records` Record ID.
/// - `field`/`fields` Field selector.
/// - `data` Serialised data being set.
///
#[derive(Drop, Serde, starknet::Event)]
pub struct SetValue {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub field: felt252,
    pub data: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct SetRecordFields {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub fields: Span<felt252>,
    pub data: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct SetRecordsField {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    #[key]
    pub field: felt252,
    pub data: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct SetRecordsFields {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    pub fields: Span<felt252>,
    pub data: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct SetRecord {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub data: Span<felt252>,
}


#[derive(Drop, Serde, starknet::Event)]
pub struct SetRecords {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    pub data: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct SetRecordDataFromSchema {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub schema: felt252,
    pub data: Span<felt252>,
}


#[derive(Drop, Serde, starknet::Event)]
pub struct SetRecordsDataFromSchema {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    #[key]
    pub schema: felt252,
    pub data: Span<felt252>,
}


#[derive(Drop, Serde, starknet::Event)]
pub struct DeleteValue {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub field: felt252,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DeleteRecord {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DeleteRecords {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DeleteRecordFields {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub fields: Span<felt252>,
}


#[derive(Drop, Serde, starknet::Event)]
pub struct DeleteRecordsField {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    #[key]
    pub field: felt252,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DeleteRecordsFields {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    pub fields: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DeleteRecordFromSchema {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub schema: felt252,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct DeleteRecordsFromSchema {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    #[key]
    pub schema: felt252,
}

