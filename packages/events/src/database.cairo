use introspect_types::Ty;
use introspect_types::ty::Field;

pub enum DatabaseEvents {
    DeclareTable: DeclareTable,
    DeclareTableWithFields: DeclareTableWithFields,
    DeclareTableWithSchema: DeclareTableWithSchema,
    UndeclareTable: UndeclareTable,
    DeclareTableField: DeclareTableField,
    DeclareTableFields: DeclareTableFields,
    UndeclareField: UndeclareField,
    UndeclareFields: UndeclareFields,
    SetTableValue: SetValue,
    SetRecordFields: SetRecordFields,
    SetFieldRecords: SetFieldRecords,
    SetRecordsFields: SetRecordsFields,
    SetRecord: SetRecord,
    SetRecords: SetRecords,
    SetRecordFromSchema: SetRecordFromSchema,
    SetRecordsFromSchema: SetRecordsFromSchema,
    DeleteValue: DeleteValue,
    DeleteRecordFields: DeleteRecordFields,
    DeleteFieldRecords: DeleteFieldRecords,
    DeleteRecordsFields: DeleteRecordsFields,
    DeleteRecord: DeleteRecord,
    DeleteRecords: DeleteRecords,
    DeleteRecordFromSchema: DeleteRecordFromSchema,
    DeleteRecordsFromSchema: DeleteRecordsFromSchema,
}


/// Table declaration common fields
///
/// - `id` Table ID.
/// - `name` in table events: Table name, in field events: Table ID the field name.
/// - `fields` in table events: Table fields.
/// - `schema` in table events: Schema ID.

/// Emitted when a new table is declared with inline field definitions.
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

/// Emitted when a table is undeclared.
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

pub struct SetValue {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub field: felt252,
    pub data: Span<felt252>,
}

pub struct SetRecordFields {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub fields: Span<felt252>,
    pub data: Span<felt252>,
}

pub struct SetRecordsField {
    #[key]
    pub table: felt252,
    #[key]
    pub field: felt252,
    pub records: Span<felt252>,
    pub data: Span<felt252>,
}

pub struct SetRecordsFields {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    pub fields: Span<felt252>,
    pub data: Span<Span<felt252>>,
}


/// Sets a record using a predefined schema layout.
///
/// Fields:
/// - `table`: Table ID.
/// - `record`: Record ID.
/// - `schema`: Schema ID (defines field order).
/// - `data`: Field values matching the schema.

pub struct SetRecordFromSchema {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub schema: felt252,
    pub data: Span<felt252>,
}

/// Sets multiple records using a predefined schema layout.
///
/// Fields:
/// - `table`: Table ID.
/// - `schema`: Schema ID.
/// - `records`: List of record IDs.
/// - `data`: Flattened record values following the schema order.

pub struct SetRecordsFromSchema {
    #[key]
    pub table: felt252,
    #[key]
    pub schema: felt252,
    pub records: Span<felt252>,
    pub data: Span<Span<felt252>>,
}
