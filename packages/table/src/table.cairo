use core_ext::{ToSnapshot, ToSpan};
use introspect_events::EmitEvent;
use introspect_events::database::{
    CreateTableWithColumns, DeleteField, DeleteFields, DeleteRecord, DeleteRecords, DeletesField,
    DeletesFields, InsertField, InsertsField,
};
use introspect_types::Attribute;
use crate::field::RecordsField;
use crate::record::RecordIds;
use crate::recordable_events::{Emittable, EmittableBatch, EmittableFields, EmittableFieldsBatch};
use crate::{Member, RecordId, TableStructure};


pub trait ITable {
    impl Table: TableStructure;
    const ID: felt252;
    fn name() -> ByteArray;
    fn append_table_attributes(ref attributes: Array<Attribute>) {}
    fn register_table() {
        let mut attributes = Self::Table::attributes();
        Self::append_table_attributes(ref attributes);
        CreateTableWithColumns {
            id: Self::ID,
            name: Self::name(),
            attributes: attributes.span(),
            primary: Self::Table::primary(),
            columns: Self::Table::columns(),
        }
            .emit_event();
    }
    fn insert<Item, impl RE: Emittable<Self::ID, Self::Table, Item>, +Drop<Item>>(
        record: Item,
    ) {
        RE::emit_item(@record);
    }
    fn inserts<Items, impl RE: EmittableBatch<Self::ID, Self::Table, Items>, +Drop<Items>>(
        records: Items,
    ) {
        RE::emit_batch(records);
    }
    fn insert_field<
        const ID: felt252,
        ToId,
        ToField,
        impl RId: RecordId<Self::Table, ToId>,
        impl Member: Member<Self::Table, ID, Self::Table::Record>,
        impl SF: ToSnapshot<ToField, Member::Type>,
        +Drop<ToId>,
        +Drop<ToField>,
    >(
        id: ToId, field: ToField,
    ) {
        InsertField {
            table: Self::ID,
            row: RId::record_id(@id),
            column: ID,
            data: Member::serialize_member_inline(SF::to_snapshot(field)),
        }
            .emit_event();
    }
    fn inserts_field<
        const ID: felt252,
        impl Member: Member<Self::Table, ID, Self::Table::Record>,
        Items,
        impl Field: RecordsField<ID, Self::Table, Member, Items>,
    >(
        items: Items,
    ) {
        let entries = Field::serialise_to_entries(items);
        InsertsField { table: Self::ID, column: ID, entries }.emit_event();
    }
    fn insert_fields<Item, impl RE: EmittableFields<Self::ID, Self::Table, Item>, +Drop<Item>>(
        record: Item,
    ) {
        RE::emit_fields(@record);
    }

    fn inserts_fields<
        Items, impl RE: EmittableFieldsBatch<Self::ID, Self::Table, Items>, +Drop<Items>,
    >(
        records: Items,
    ) {
        RE::emit_fields_batch(records);
    }
    fn delete_record<ToId, impl RID: RecordId<Self::Table, ToId>, +Drop<ToId>>(
        id: ToId,
    ) {
        DeleteRecord { table: Self::ID, row: RID::record_id(@id) }.emit_event();
    }
    fn delete_records<ToIds, impl Ids: RecordIds<Self::Table, ToIds>, +Drop<ToIds>>(
        ids: ToIds,
    ) {
        DeleteRecords { table: Self::ID, rows: Ids::record_ids(ids) }.emit_event();
    }
    fn delete_field<
        const COLUMN_ID: felt252,
        ToId,
        impl RID: RecordId<Self::Table, ToId>,
        impl Member: Member<Self::Table, COLUMN_ID, Self::Table::Record>,
        +Drop<ToId>,
    >(
        id: ToId,
    ) {
        DeleteField { table: Self::ID, row: RID::record_id(@id), column: COLUMN_ID }.emit_event();
    }
    fn deletes_field<
        const COLUMN_ID: felt252,
        ToIds,
        impl TID: RecordIds<Self::Table, ToIds>,
        impl Member: Member<Self::Table, COLUMN_ID, Self::Table::Record>,
    >(
        ids: ToIds,
    ) {
        DeletesField { table: Self::ID, rows: TID::record_ids(ids), column: COLUMN_ID }
            .emit_event();
    }
    fn delete_fields<
        ToId,
        ColumnIds,
        impl Id: RecordId<Self::Table, ToId>,
        +ToSpan<ColumnIds, felt252>,
        +Drop<ToId>,
        +Drop<ColumnIds>,
    >(
        id: ToId, columns: ColumnIds,
    ) {
        DeleteFields { table: Self::ID, row: Id::record_id(@id), columns: columns.to_span() }
            .emit_event();
    }
    fn deletes_fields<
        ToIds,
        ColumnIds,
        impl Ids: RecordIds<Self::Table, ToIds>,
        +ToSpan<ColumnIds, felt252>,
        +Drop<ColumnIds>,
    >(
        ids: ToIds, columns: ColumnIds,
    ) {
        DeletesFields { table: Self::ID, rows: Ids::record_ids(ids), columns: columns.to_span() }
            .emit_event();
    }
}
