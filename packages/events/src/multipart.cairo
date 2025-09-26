/// Messages for when data is too large to fit in a single event
#[derive(Drop, Serde, starknet::Event)]
pub enum MultiPartMessages {
    MultiPartMessage: MultiPartMessage,
    MultiPartMessageEnd: MultiPartMessageEnd,
    SetTableValue: MultiPartSetValue,
    SetRecordFields: MultiPartSetRecordFields,
    SetFieldRecords: MultiPartSetFieldRecords,
    SetRecordsFields: MultiPartSetRecordsFields,
    SetRecord: MultiPartSetRecord,
    SetRecords: MultiPartSetRecords,
    SetRecordFromSchema: MultiPartSetRecordFromSchema,
    SetRecordsFromSchema: MultiPartSetRecordsFromSchema,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct MultiPartMessage {
    #[key]
    pub id: felt252,
    #[key]
    pub part: u32,
    pub data: Span<felt252>,
}

#[derive(Drop, Serde, starknet::Event)]
pub struct MultiPartMessageEnd {
    #[key]
    pub id: felt252,
    #[key]
    pub part: u32,
    pub data: Span<felt252>,
}
