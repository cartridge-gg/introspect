use introspect_events::EmitEvent;
use introspect_events::database::{
    CreateTableWithColumns, DeleteField, DeleteFields, DeleteRecord, DeleteRecords, DeletesField,
    InsertField, InsertsField,
};
use introspect_types::Attribute;
use crate::field::RecordsField;
use crate::record::RecordIds;
use crate::recordable_events::{RecordableEvent, RecordablesEvent};
use crate::{MemberTrait, RecordId, Snapable, TableStructure};


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
    fn insert<Entry, impl RE: RecordableEvent<Self::ID, Self::Struct, Entry>, +Drop<Entry>>(
        record: Entry,
    ) {
        RE::emit_recordable(@record);
    }
    fn inserts<Entries, impl RE: RecordablesEvent<Self::ID, Self::Struct, Entries>, +Drop<Entries>>(
        records: Entries,
    ) {
        RE::emit_recordables(records);
    }
    fn insert_field<
        const ID: felt252,
        ToId,
        F,
        impl RId: RecordId<Self::Struct, ToId>,
        impl Member: MemberTrait<Self::Struct, ID>,
        impl SF: Snapable<F, Member::Type>,
        +Drop<ToId>,
        +Drop<F>,
    >(
        id: ToId, field: F,
    ) {
        InsertField {
            table: Self::ID,
            record: RId::record_id(@id),
            column: ID,
            data: Member::serialize_member_inline(SF::snapshot(field)),
        }
            .emit_event();
    }
    fn inserts_field<
        const ID: felt252,
        impl Member: MemberTrait<Self::Struct, ID>,
        impl RF: RecordsField<ID, Self::Struct, Member, Entries>,
        Entries,
    >(
        id_fields: Entries,
    ) {
        let records_data = RF::serialise_to_id_data_span(id_fields);
        InsertsField { table: Self::ID, column: ID, records_data }.emit_event();
    }
    // fn insert_fields<R, impl RE: RecordFieldsEvent<R, Self::Struct, Self::ID>, +Drop<R>>(
    //     record: R,
    // ) {
    //     RE::emit_record_fields(@record);
    // }

    // fn inserts_fields<RS, impl RE: RecordsFieldsEvent<RS, Self::Struct, Self::ID>, +Drop<RS>>(
    //     records: RS,
    // ) {
    //     RE::emit_records_fields(records);
    // }
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
        impl Member: MemberTrait<Self::Struct, COLUMN_ID>,
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
        impl Member: MemberTrait<Self::Struct, COLUMN_ID>,
    >(
        ids: ToIds,
    ) {
        DeletesField { table: Self::ID, records: TID::record_ids(ids), column: COLUMN_ID }
            .emit_event();
    }
    // fn delete_fields<const SIZE: usize, K, impl RID: RecordId<K, Self::Struct>, +Drop<K>>(
//     id: K, columns: ColumnIds,
// ) {
//     DeleteFields {
//         table: Self::ID, record: RID::record_id(@id), columns: CID::columns_ids(columns),
//     }
//         .emit_event();
// }
// fn deletes_fields<KS, CS, impl RID: RecordIds<KS, Self::Struct>, +Drop<KS>, +Drop<CS>>(
//     ids: KS, columns: CS,
// ) {
//     DeletesFields {
//         table: Self::ID, records: RID::record_ids(ids), columns: CID::columns_ids(columns),
//     }
//         .emit_event();
// }
}
