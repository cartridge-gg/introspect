use core_ext::Spannable;
use introspect_events::EmitEvent;
use introspect_events::database::{
    InsertFieldSet, InsertFields, InsertRecord, InsertRecords, InsertsFieldSet, InsertsFields,
};
use crate::TableStructure;
use crate::record::{RecordTrait, RecordsTrait};
use crate::set::ColumnSet;

pub trait Emittable<const TABLE_ID: felt252, impl Struct: TableStructure, Item> {
    fn emit_item(item: @Item);
}

pub trait EmittableBatch<const TABLE_ID: felt252, impl Struct: TableStructure, Items> {
    fn emit_batch(items: Items);
}

pub trait EmittableFields<const TABLE_ID: felt252, impl Struct: TableStructure, Item> {
    fn emit_fields(item: @Item);
}

pub trait EmittableFieldsBatch<const TABLE_ID: felt252, impl Struct: TableStructure, Items> {
    fn emit_fields_batch(items: Items);
}

pub impl EmittableRecord<
    const TABLE_ID: felt252,
    impl Struct: TableStructure,
    Item,
    impl Record: RecordTrait<Struct, Item>,
    +Drop<Item>,
> of Emittable<TABLE_ID, Struct, Item> {
    fn emit_item(item: @Item) {
        let (row, data) = Record::record_tuple(item);
        InsertRecord { table: TABLE_ID, row, data }.emit_event();
    }
}


pub impl EmittableRecordBatch<
    const TABLE_ID: felt252,
    impl Struct: TableStructure,
    Items,
    impl Records: RecordsTrait<Struct, Items>,
> of EmittableBatch<TABLE_ID, Struct, Items> {
    fn emit_batch(items: Items) {
        let entries = Records::serialize_records(items);
        InsertRecords { table: TABLE_ID, entries }.emit_event();
    }
}

pub impl EmittableColumnSet<
    const TABLE_ID: felt252,
    impl Struct: TableStructure,
    const SIZE: usize,
    Item,
    impl Set: ColumnSet<Struct, Item, SIZE>,
> of Emittable<TABLE_ID, Struct, Item> {
    fn emit_item(item: @Item) {
        let (row, data) = Set::set_tuple(item);
        InsertFieldSet { table: TABLE_ID, set: Set::GROUP_ID, row, data }.emit_event();
    }
}

pub impl EmittableColumnSetBatch<
    const TABLE_ID: felt252,
    impl Struct: TableStructure,
    const SIZE: usize,
    Items,
    Item,
    impl Set: ColumnSet<Struct, Item, SIZE>,
    +Spannable<Items, Item>,
> of EmittableBatch<TABLE_ID, Struct, Items> {
    fn emit_batch(items: Items) {
        let entries = Set::serialise_rows_set(items);
        InsertsFieldSet { table: TABLE_ID, set: Set::GROUP_ID, entries }.emit_event();
    }
}

pub impl EmittableFieldsImpl<
    const TABLE_ID: felt252,
    impl Struct: TableStructure,
    const SIZE: usize,
    Item,
    impl Set: ColumnSet<Struct, Item, SIZE>,
    impl ToSpan: ToSpanTrait<[felt252; SIZE], felt252>,
> of EmittableFields<TABLE_ID, Struct, Item> {
    fn emit_fields(item: @Item) {
        let (row, data) = Set::set_tuple(item);
        InsertFields { table: TABLE_ID, columns: ToSpan::span(@Set::COLUMN_IDS), row, data }
            .emit_event();
    }
}

pub impl EmittableFieldsBatchImpl<
    const TABLE_ID: felt252,
    impl Struct: TableStructure,
    const SIZE: usize,
    Items,
    Item,
    impl Set: ColumnSet<Struct, Item, SIZE>,
    +Spannable<Items, Item>,
    impl ToSpan: ToSpanTrait<[felt252; SIZE], felt252>,
> of EmittableFieldsBatch<TABLE_ID, Struct, Items> {
    fn emit_fields_batch(items: Items) {
        let entries = Set::serialise_rows_set(items);
        InsertsFields { table: TABLE_ID, columns: ToSpan::span(@Set::COLUMN_IDS), entries }
            .emit_event();
    }
}

