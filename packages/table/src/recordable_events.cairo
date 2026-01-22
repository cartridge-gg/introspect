use introspect_events::EmitEvent;
use introspect_events::database::{
    InsertFieldGroup, InsertFields, InsertRecord, InsertRecords, InsertsFieldGroup, InsertsFields,
};
use crate::record::{RecordTrait, RecordsTrait};
use crate::set::ColumnSet;
use crate::{Spannable, TableStructure};

pub trait Emittable<const TABLE_ID: felt252, impl Struct: TableStructure, Entry> {
    fn emit(entry: @Entry);
}

pub trait EmittableBatch<const TABLE_ID: felt252, impl Struct: TableStructure, Entries> {
    fn emit_batch(entries: Entries);
}

pub trait EmittableFields<const TABLE_ID: felt252, impl Struct: TableStructure, Entry> {
    fn emit_fields(entry: @Entry);
}

pub trait EmittableFieldsBatch<const TABLE_ID: felt252, impl Struct: TableStructure, Entries> {
    fn emit_fields_batch(entries: Entries);
}

pub impl EmittableRecord<
    const TABLE_ID: felt252,
    impl Struct: TableStructure,
    Entry,
    impl Record: RecordTrait<Struct, Entry>,
    +Drop<Entry>,
> of Emittable<TABLE_ID, Struct, Entry> {
    fn emit(entry: @Entry) {
        let (record, data) = Record::record_tuple(entry);
        InsertRecord { table: TABLE_ID, record, data }.emit_event();
    }
}


pub impl EmittableRecordBatch<
    const TABLE_ID: felt252,
    impl Struct: TableStructure,
    Entries,
    impl Records: RecordsTrait<Struct, Entries>,
> of EmittableBatch<TABLE_ID, Struct, Entries> {
    fn emit_batch(entries: Entries) {
        let entries = Records::serialize_records(entries);
        InsertRecords { table: TABLE_ID, records_data: entries }.emit_event();
    }
}

pub impl EmittableColumnSet<
    const TABLE_ID: felt252,
    impl Struct: TableStructure,
    const SIZE: usize,
    Entry,
    impl Set: ColumnSet<Struct, Entry, SIZE>,
> of Emittable<TABLE_ID, Struct, Entry> {
    fn emit(entry: @Entry) {
        let (row, data) = Set::set_tuple(entry);
        InsertFieldGroup { table: TABLE_ID, group: Set::GROUP_ID, record: row, data }.emit_event();
    }
}

pub impl EmittableColumnSetBatch<
    const TABLE_ID: felt252,
    impl Struct: TableStructure,
    const SIZE: usize,
    Entries,
    Entry,
    impl Set: ColumnSet<Struct, Entry, SIZE>,
    +Spannable<Entries, Entry>,
> of EmittableBatch<TABLE_ID, Struct, Entries> {
    fn emit_batch(entries: Entries) {
        let entries = Set::serialise_rows_set(entries);
        InsertsFieldGroup { table: TABLE_ID, group: Set::GROUP_ID, records_data: entries }
            .emit_event();
    }
}

pub impl EmittableFieldsImpl<
    const TABLE_ID: felt252,
    impl Struct: TableStructure,
    const SIZE: usize,
    Entry,
    impl Set: ColumnSet<Struct, Entry, SIZE>,
    impl ToSpan: ToSpanTrait<[felt252; SIZE], felt252>,
> of EmittableFields<TABLE_ID, Struct, Entry> {
    fn emit_fields(entry: @Entry) {
        let (row, data) = Set::set_tuple(entry);
        InsertFields { table: TABLE_ID, columns: ToSpan::span(@Set::COLUMN_IDS), record: row, data }
            .emit_event();
    }
}

pub impl EmittableFieldsBatchImpl<
    const TABLE_ID: felt252,
    impl Struct: TableStructure,
    const SIZE: usize,
    Entries,
    Entry,
    impl Set: ColumnSet<Struct, Entry, SIZE>,
    +Spannable<Entries, Entry>,
    impl ToSpan: ToSpanTrait<[felt252; SIZE], felt252>,
> of EmittableFieldsBatch<TABLE_ID, Struct, Entries> {
    fn emit_fields_batch(entries: Entries) {
        let entries = Set::serialise_rows_set(entries);
        InsertsFields {
            table: TABLE_ID, columns: ToSpan::span(@Set::COLUMN_IDS), records_data: entries,
        }
            .emit_event();
    }
}

