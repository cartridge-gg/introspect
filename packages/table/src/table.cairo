use introspect_events::EmitEvent;
use introspect_events::database::{
    CreateTableWithColumns, DeleteField, DeleteFields, DeleteRecord, DeleteRecords, DeletesField,
    DeletesFields, InsertField, InsertsField,
};
use introspect_types::Attribute;
use crate::field::RecordsField;
use crate::record::RecordIds;
use crate::recordable_events::{Emittable, EmittableBatch, EmittableFields, EmittableFieldsBatch};
use crate::{Member, RecordId, Spannable, TableStructure, ToSnapshot};


pub trait ITable {
    impl Struct: TableStructure;
    const ID: felt252;
    fn name() -> ByteArray;
    fn append_table_attributes(ref attributes: Array<Attribute>) {}
    fn register_table() {
        let mut attributes = Self::Struct::attributes();
        Self::append_table_attributes(ref attributes);
        CreateTableWithColumns {
            id: Self::ID,
            name: Self::name(),
            attributes: attributes.span(),
            primary: Self::Struct::primary(),
            columns: Self::Struct::columns(),
        }
            .emit_event();
    }
    fn insert<Entry, impl RE: Emittable<Self::ID, Self::Struct, Entry>, +Drop<Entry>>(
        record: Entry,
    ) {
        RE::emit(@record);
    }
    fn inserts<Entries, impl RE: EmittableBatch<Self::ID, Self::Struct, Entries>, +Drop<Entries>>(
        records: Entries,
    ) {
        RE::emit_batch(records);
    }
    fn insert_field<
        const ID: felt252,
        ToId,
        F,
        impl RId: RecordId<Self::Struct, ToId>,
        impl Member: Member<Self::Struct, ID, Self::Struct::Record>,
        impl SF: ToSnapshot<F, Member::Type>,
        +Drop<ToId>,
        +Drop<F>,
    >(
        id: ToId, field: F,
    ) {
        InsertField {
            table: Self::ID,
            record: RId::record_id(@id),
            column: ID,
            data: Member::serialize_member_inline(SF::to_snapshot(field)),
        }
            .emit_event();
    }
    fn inserts_field<
        const ID: felt252,
        impl Member: Member<Self::Struct, ID, Self::Struct::Record>,
        Entries,
        impl Field: RecordsField<ID, Self::Struct, Member, Entries>,
    >(
        entries: Entries,
    ) {
        let records_data = Field::serialise_to_id_data_span(entries);
        InsertsField { table: Self::ID, column: ID, records_data }.emit_event();
    }
    fn insert_fields<Entry, impl RE: EmittableFields<Self::ID, Self::Struct, Entry>, +Drop<Entry>>(
        record: Entry,
    ) {
        RE::emit_fields(@record);
    }

    fn inserts_fields<
        Entries, impl RE: EmittableFieldsBatch<Self::ID, Self::Struct, Entries>, +Drop<Entries>,
    >(
        records: Entries,
    ) {
        RE::emit_fields_batch(records);
    }
    fn delete_record<ToId, impl RID: RecordId<Self::Struct, ToId>, +Drop<ToId>>(
        id: ToId,
    ) {
        DeleteRecord { table: Self::ID, record: RID::record_id(@id) }.emit_event();
    }
    fn delete_records<ToIds, impl Ids: RecordIds<Self::Struct, ToIds>, +Drop<ToIds>>(
        ids: ToIds,
    ) {
        DeleteRecords { table: Self::ID, records: Ids::record_ids(ids) }.emit_event();
    }
    fn delete_field<
        const COLUMN_ID: felt252,
        ToId,
        impl RID: RecordId<Self::Struct, ToId>,
        impl Member: Member<Self::Struct, COLUMN_ID, Self::Struct::Record>,
        +Drop<ToId>,
    >(
        id: ToId,
    ) {
        DeleteField { table: Self::ID, record: RID::record_id(@id), column: COLUMN_ID }
            .emit_event();
    }
    fn deletes_field<
        const COLUMN_ID: felt252,
        ToIds,
        impl TID: RecordIds<Self::Struct, ToIds>,
        impl Member: Member<Self::Struct, COLUMN_ID, Self::Struct::Record>,
    >(
        ids: ToIds,
    ) {
        DeletesField { table: Self::ID, records: TID::record_ids(ids), column: COLUMN_ID }
            .emit_event();
    }
    fn delete_fields<
        ToId,
        ColumnIds,
        impl Id: RecordId<Self::Struct, ToId>,
        +Spannable<ColumnIds, felt252>,
        +Drop<ToId>,
        +Drop<ColumnIds>,
    >(
        id: ToId, columns: ColumnIds,
    ) {
        DeleteFields { table: Self::ID, record: Id::record_id(@id), columns: columns.to_span() }
            .emit_event();
    }
    fn deletes_fields<
        ToIds,
        ColumnIds,
        impl Ids: RecordIds<Self::Struct, ToIds>,
        +Spannable<ColumnIds, felt252>,
        +Drop<ColumnIds>,
    >(
        ids: ToIds, columns: ColumnIds,
    ) {
        DeletesFields { table: Self::ID, records: Ids::record_ids(ids), columns: columns.to_span() }
            .emit_event();
    }
}
