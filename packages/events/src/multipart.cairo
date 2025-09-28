/// Multi-part database events for when data payloads exceed single event limits.
/// These events mirror the standard database events but include an `id` field for message tracking.
///
/// Multi-part message common fields
///
/// - `id` Message ID for grouping related parts.
/// - `part` Part number in sequence (0-based).
/// - `table` Table ID being operated on.
/// - `record`/`records` Record ID(s) being modified.
/// - `field`/`fields` Field selector(s).
/// - `schema` Schema ID for schema-based operations.
/// - `data` Serialized data payload.
#[derive(Drop, Serde, starknet::Event)]
pub enum MultiPartMessages {
    MultiPartMessage: MultiPartMessage,
    MultiPartMessageEnd: MultiPartMessageEnd,
    SetValue: MultiPartSetValue,
    SetRecordFields: MultiPartSetRecordFields,
    SetRecordsField: MultiPartSetRecordsField,
    SetRecordsFields: MultiPartSetRecordsFields,
    SetRecord: MultiPartSetRecord,
    SetRecords: MultiPartSetRecords,
    SetRecordDataFromSchema: MultiPartSetRecordDataFromSchema,
    SetRecordsDataFromSchema: MultiPartSetRecordsDataFromSchema,
}

/// Generic multi-part message for large data payloads.
#[derive(Drop, Serde, starknet::Event)]
pub struct MultiPartMessage {
    #[key]
    pub id: felt252,
    #[key]
    pub part: u32,
    pub data: Span<felt252>,
}

/// Final part of a multi-part message sequence.
#[derive(Drop, Serde, starknet::Event)]
pub struct MultiPartMessageEnd {
    #[key]
    pub id: felt252,
    #[key]
    pub part: u32,
    pub data: Span<felt252>,
}

/// Multi-part version of SetValue - sets a single field of a single record.
#[derive(Drop, Serde, starknet::Event)]
pub struct MultiPartSetValue {
    #[key]
    pub id: felt252,
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub field: felt252,
    pub data: Span<felt252>,
}

/// Multi-part version of SetRecordFields - sets multiple fields of a single record.
#[derive(Drop, Serde, starknet::Event)]
pub struct MultiPartSetRecordFields {
    #[key]
    pub id: felt252,
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub fields: Span<felt252>,
    pub data: Span<felt252>,
}

/// Multi-part version of SetRecordsField - sets a single field across multiple records.
#[derive(Drop, Serde, starknet::Event)]
pub struct MultiPartSetRecordsField {
    #[key]
    pub id: felt252,
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    #[key]
    pub field: felt252,
    pub data: Span<felt252>,
}

/// Multi-part version of SetRecordsFields - sets multiple fields across multiple records.
#[derive(Drop, Serde, starknet::Event)]
pub struct MultiPartSetRecordsFields {
    #[key]
    pub id: felt252,
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    pub fields: Span<felt252>,
    pub data: Span<felt252>,
}

/// Multi-part version of SetRecord - sets all fields of a single record.
#[derive(Drop, Serde, starknet::Event)]
pub struct MultiPartSetRecord {
    #[key]
    pub id: felt252,
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    pub data: Span<felt252>,
}

/// Multi-part version of SetRecords - sets all fields across multiple records.
#[derive(Drop, Serde, starknet::Event)]
pub struct MultiPartSetRecords {
    #[key]
    pub id: felt252,
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    pub data: Span<felt252>,
}

/// Multi-part version of SetRecordDataFromSchema - sets a record using schema definition.
#[derive(Drop, Serde, starknet::Event)]
pub struct MultiPartSetRecordDataFromSchema {
    #[key]
    pub id: felt252,
    #[key]
    pub table: felt252,
    #[key]
    pub record: felt252,
    #[key]
    pub schema: felt252,
    pub data: Span<felt252>,
}

/// Multi-part version of SetRecordsDataFromSchema - sets multiple records using schema.
#[derive(Drop, Serde, starknet::Event)]
pub struct MultiPartSetRecordsDataFromSchema {
    #[key]
    pub id: felt252,
    #[key]
    pub table: felt252,
    pub records: Span<felt252>,
    #[key]
    pub schema: felt252,
    pub data: Span<felt252>,
}

