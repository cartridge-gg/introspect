use introspect_events::EmitEvent;
use introspect_events::database::{InsertRecord, InsertRecords};
use crate::TableStructure;
use crate::record::{RecordTrait, RecordsTrait};

pub trait RecordableEvent<const TABLE_ID: felt252, impl Struct: TableStructure, Entry> {
    fn emit_recordable(entry: @Entry);
}

pub trait RecordablesEvent<const TABLE_ID: felt252, impl Struct: TableStructure, Entries> {
    fn emit_recordables(entries: Entries);
}

trait RecordFieldsEvent<R, impl Struct: TableStructure, const ID: felt252> {
    fn emit_record_fields(record_fields: @R);
}

trait RecordsFieldsEvent<RS, impl Struct: TableStructure, const ID: felt252> {
    fn emit_records_fields(records_fields: RS);
}


pub impl EmitRecordableRecordImpl<
    const TABLE_ID: felt252,
    Entry,
    impl Struct: TableStructure,
    impl Record: RecordTrait<Struct, Entry>,
    +Drop<Entry>,
> of RecordableEvent<TABLE_ID, Struct, Entry> {
    fn emit_recordable(entry: @Entry) {
        let (record, data) = Record::record_tuple(entry);
        InsertRecord { table: TABLE_ID, record, data }.emit_event();
    }
}


pub impl RecordablesEventImpl<
    const TABLE_ID: felt252,
    Entries,
    impl Struct: TableStructure,
    impl Records: RecordsTrait<Struct, Entries>,
> of RecordablesEvent<TABLE_ID, Struct, Entries> {
    fn emit_recordables(entries: Entries) {
        let entries = Records::serialize_records(entries);
        InsertRecords { table: TABLE_ID, records_data: entries }.emit_event();
    }
}
