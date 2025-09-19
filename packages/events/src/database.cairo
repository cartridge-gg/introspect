use introspect_types::Ty;
use introspect_types::ty::Member;

pub enum DatabaseEvents {
    DeclareTable: DeclareTable,
    DeclareTableWithFields: DeclareTableWithFields,
    DeclareTableWithSchema: DeclareTableWithSchema,
    DeclareTableField: DeclareTableField,
    DeclareTableFields: DeclareTableFields,
    UndeclareTable: UndeclareTable,
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


/// TABLE MANAGEMENT EVENTS
/// ===========================
/// Table management common fields
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
    pub fields: Span<Member>,
}

/// Declares a table using a pre-defined schema.
#[derive(Drop, Serde, starknet::Event)]
pub struct DeclareTableWithSchema {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub schema: felt252,
}

/// Table field common fields
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
    pub fields: Span<Member>,
}

/// Emitted when a table is undeclared.
///
/// Fields:
/// - `id`: Unique identifier of the table (e.g., hash of the name or a custom ID).

pub struct UndeclareTable {
    #[key]
    pub id: felt252,
}


///
/// Fields:
/// - `id`: Unique identifier for the table.
/// - `name`: Human-readable name of the table.
/// - `schema`: The schema declared via `DeclareSchema`.

/// Declares multiple fields for an existing table.
///
/// Fields:
/// - `table`: Table ID the fields belong to.
/// - `fields`: Array of field definitions (name + layout).

/// Sets a single field for a single record.
///
/// Fields:
/// - `table`: Table ID.
/// - `record`: Record/entity ID.
/// - `field`: Field selector.
/// - `value`: Value to set.

pub struct SetValue {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub field: felt252,
    pub value: Span<felt252>,
}


/// Sets multiple fields for a single record.
///
/// Fields:
/// - `table`: Table ID.
/// - `record`: Record ID.
/// - `fields`: List of field selectors.
/// - `values`: Concatenated field values (decoded per layout).

pub struct SetRecordFields {
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub fields: Span<felt252>,
    pub values: Span<felt252>,
}


/// Sets a single field across multiple records.
///
/// Fields:
/// - `table`: Table ID.
/// - `field`: Field selector.
/// - `records`: List of record IDs.
/// - `values`: Corresponding values for each record.

pub struct SetFieldRecords {
    #[key]
    pub table: felt252,
    #[key]
    pub field: felt252,
    pub records: Span<felt252>,
    pub values: Span<felt252>,
}

/// Sets multiple fields across multiple records (row-major order).
///
/// Fields:
/// - `table`: Table ID.
/// - `records`: List of record IDs (rows).
/// - `fields`: List of field selectors (columns).
/// - `data`: Flattened data buffer [record][field] values.

pub struct SetRecordsFields {
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    pub fields: Span<felt252>,
    pub data: Span<felt252>,
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
    pub data: Span<felt252>,
}
