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
    MultiPartMessageStart: MultiPartMessageStart,
    MultiPartMessageEnd: MultiPartMessageEnd,
}


pub struct MultiPartMessage {
    #[key]
    pub selector: felt252,
    #[key]
    pub id: felt252,
    #[key]
    pub part: u32,
    pub data: Span<felt252>,
}

pub struct MultiPartMessageEnd {
    #[key]
    pub selector: felt252,
    #[key]
    pub id: felt252,
    #[key]
    pub part: u32,
    pub data: Span<felt252>,
}
// #[derive(Drop, Serde, starknet::Event)]
// pub enum MultiPartDatabaseEvents {
//     CreateTableWithColumnsMp: MultiPartCreateTableWithColumns,
//     AddColumnMp: MultiPartAddTableColumn,
//     AddColumnsMp: MultiPartAddTableColumns,
//     InsertRecordFieldMp: MultiPartInsertRecordField,
//     InsertRecordFieldsMp: MultiPartInsertRecordFields,
//     InsertRecordsFieldMp: MultiPartInsertRecordsField,
//     InsertRecordsFieldsMp: MultiPartInsertRecordsFields,
//     InsertRecordMp: MultiPartInsertRecord,
//     InsertRecordsMp: MultiPartInsertRecords,
//     InsertRecordDataFromSchemaMp: MultiPartInsertRecordDataFromSchema,
//     InsertRecordsDataFromSchemaMp: MultiPartInsertRecordsDataFromSchema,
// }

// #[derive(Drop, Serde, starknet::Event)]
// pub enum MultiPartVariableEvents {
//     MultiPartRegisterVariable: MultiPartRegisterVariable,
//     MultiPartDeclareVariable: MultiPartDeclareVariable,
//     MultiPartSetVariable: MultiPartSetVariable,
// }

/// Multi-part version of CreateTableWithColumns event.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartCreateTableWithColumns {
//     #[key]
//     pub mp_id: felt252,
//     pub id: felt252,
//     pub name: ByteArray,
//     pub data: Span<felt252>,
// }

// /// Multi-part version of AddTableColumn event.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartAddTableColumn {
//     #[key]
//     pub mp_id: felt252,
//     pub table: felt252,
//     pub id: felt252,
//     pub name: ByteArray,
//     pub attrs: Span<felt252>,
//     pub data: Span<felt252>,
// }

// /// Multi-part version of AddTableColumns event.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartAddTableColumns {
//     #[key]
//     pub table: felt252,
//     pub data: Span<felt252>,
// }

// /// Multi-part version of InsertRecordField event.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartInsertRecordField {
//     #[key]
//     pub table: felt252,
//     #[key]
//     pub record: felt252,
//     #[key]
//     pub field: felt252,
//     pub data: Span<felt252>,
// }

// /// Multi-part version of InsertRecordFields event.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartInsertRecordFields {
//     #[key]
//     pub table: felt252,
//     #[key]
//     pub record: felt252,
//     pub fields: Span<felt252>,
//     pub data: Span<felt252>,
// }

// /// Multi-part version of InsertRecordsField event.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartInsertRecordsField {
//     #[key]
//     pub table: felt252,
//     pub records: Span<felt252>,
//     #[key]
//     pub field: felt252,
//     pub data: Span<felt252>,
// }

// /// Multi-part version of InsertRecordsFields event.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartInsertRecordsFields {
//     #[key]
//     pub table: felt252,
//     pub records: Span<felt252>,
//     pub fields: Span<felt252>,
//     pub data: Span<felt252>,
// }

// /// Multi-part version of InsertRecord event.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartInsertRecord {
//     #[key]
//     pub table: felt252,
//     #[key]
//     pub record: felt252,
//     pub data: Span<felt252>,
// }

// /// Multi-part version of InsertRecords event.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartInsertRecords {
//     #[key]
//     pub table: felt252,
//     pub records: Span<felt252>,
//     pub data: Span<felt252>,
// }

// /// Multi-part version of InsertRecordDataFromSchema event.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartInsertRecordDataFromSchema {
//     #[key]
//     pub table: felt252,
//     #[key]
//     pub record: felt252,
//     #[key]
//     pub schema: felt252,
//     pub data: Span<felt252>,
// }

// /// Multi-part version of InsertRecordsDataFromSchema event.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartInsertRecordsDataFromSchema {
//     #[key]
//     pub table: felt252,
//     pub records: Span<felt252>,
//     #[key]
//     pub schema: felt252,
//     pub data: Span<felt252>,
// }

// /// Multi-part version of RegisterVariable event.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartRegisterVariable {
//     #[key]
//     pub id: felt252,
//     pub name: ByteArray,
//     pub type_def: felt252,
//     pub value: Span<felt252>,
// }

// /// Multi-part version of DeclareVariable event.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartDeclareVariable {
//     #[key]
//     pub id: felt252,
//     pub name: ByteArray,
//     pub type_def: felt252,
//     pub value: Span<felt252>,
// }

// /// Multi-part version of SetVariable event.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartSetVariable {
//     #[key]
//     pub id: felt252,
//     pub value: Span<felt252>,
// }

// /// Generic multi-part message for large data payloads.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartMessage {
//     #[key]
//     pub id: felt252,
//     #[key]
//     pub part: u32,
//     pub data: Span<felt252>,
// }

// /// Final part of a multi-part message sequence.
// #[derive(Drop, Serde, starknet::Event)]
// pub struct MultiPartMessageEnd {
//     #[key]
//     pub id: felt252,
//     #[key]
//     pub part: u32,
//     pub data: Span<felt252>,
// }


