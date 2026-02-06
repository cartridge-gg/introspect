use core::fmt::Debug;
use introspect_events::database::{
    AddColumn, AddColumns, CreateColumnSet, CreateIndex, CreateTable, CreateTableFromClass,
    CreateTableFromContract, DatabaseEvents, DeleteField, DeleteFieldSet, DeleteFieldSets,
    DeleteFields, DeleteRecord, DeleteRecords, DeletesField, DeletesFieldSet, DeletesFieldSets,
    DeletesFields, DropColumn, DropColumns, DropIndex, DropTable, InsertField, InsertFieldSet,
    InsertFieldSets, InsertFields, InsertRecord, InsertRecords, InsertsField, InsertsFieldSet,
    InsertsFieldSets, InsertsFields, RenameColumn, RenameColumns, RenamePrimary, RenameTable,
    RetypeColumn, RetypeColumns, RetypePrimary,
};
use introspect_events::testing::database;
use introspect_events::testing::database::{
    AddColumnFuzzable, AddColumnsFuzzable, CreateFieldSetFuzzable, CreateIndexFuzzable,
    CreateTableFromClassFuzzable, CreateTableFromContractFuzzable, DeleteFieldFuzzable,
    DeleteFieldSetFuzzable, DeleteFieldSetsFuzzable, DeleteFieldsFuzzable, DeleteRecordFuzzable,
    DeleteRecordsFuzzable, DeletesFieldFuzzable, DeletesFieldSetFuzzable, DeletesFieldSetsFuzzable,
    DeletesFieldsFuzzable, DropColumnFuzzable, DropColumnsFuzzable, DropIndexFuzzable,
    DropTableFuzzable, IdNameFuzzable, InsertFieldFuzzable, InsertFieldSetFuzzable,
    InsertFieldSetsFuzzable, InsertFieldsFuzzable, InsertRecordFuzzable, InsertRecordsFuzzable,
    InsertsFieldFuzzable, InsertsFieldSetFuzzable, InsertsFieldSetsFuzzable, InsertsFieldsFuzzable,
    RenameColumnFuzzable, RenameColumnsFuzzable, RenamePrimaryFuzzable, RenameTableFuzzable,
    RetypeColumnFuzzable, RetypePrimaryFuzzable,
};
use starknet::Event;

fn verify_event<E, +Event<E>, +PartialEq<E>, +Drop<E>, +Debug<E>>(event: @E) {
    let mut keys: Array<felt252> = ArrayTrait::new();
    let mut data: Array<felt252> = ArrayTrait::new();
    event.append_keys_and_data(ref keys, ref data);
    let mut keys = keys.span();
    let mut data = data.span();
    let deserialized = Event::<E>::deserialize(ref keys, ref data).unwrap();
    assert_eq!(@deserialized, event);
}

#[test]
#[fuzzer]
fn test_create_field_group_event(event: CreateColumnSet) {
    verify_event(@event);
    verify_event(@DatabaseEvents::CreateFieldSet(event));
}

impl CreateTableFuzzableImpl = database::CreateTableFuzzable<8, 8>;


#[test]
#[fuzzer]
fn test_create_table_event(event: CreateTable) {
    verify_event(@event);
    verify_event(@DatabaseEvents::CreateTable(event));
}


#[test]
#[fuzzer]
fn test_create_table_from_contract_event(event: CreateTableFromContract) {
    verify_event(@event);
    verify_event(@DatabaseEvents::CreateTableFromContract(event));
}

#[test]
#[fuzzer]
fn test_create_table_from_class_event(event: CreateTableFromClass) {
    verify_event(@event);
    verify_event(@DatabaseEvents::CreateTableFromClass(event));
}

#[test]
#[fuzzer]
fn test_rename_table_event(event: RenameTable) {
    verify_event(@event);
    verify_event(@DatabaseEvents::RenameTable(event));
}


#[test]
#[fuzzer]
fn test_drop_table_event(event: DropTable) {
    verify_event(@event);
    verify_event(@DatabaseEvents::DropTable(event));
}

#[test]
#[fuzzer]
fn test_rename_primary_event(event: RenamePrimary) {
    verify_event(@event);
    verify_event(@DatabaseEvents::RenamePrimary(event));
}

#[test]
#[fuzzer]
fn test_retype_primary_event(event: RetypePrimary) {
    verify_event(@event);
    verify_event(@DatabaseEvents::RetypePrimary(event));
}

#[test]
#[fuzzer]
fn test_add_column_event(event: AddColumn) {
    verify_event(@event);
    verify_event(@DatabaseEvents::AddColumn(event));
}

#[test]
#[fuzzer]
fn test_add_columns_event(event: AddColumns) {
    verify_event(@event);
    verify_event(@DatabaseEvents::AddColumns(event));
}

#[test]
#[fuzzer]
fn test_rename_column_event(event: RenameColumn) {
    verify_event(@event);
    verify_event(@DatabaseEvents::RenameColumn(event));
}

#[test]
#[fuzzer]
fn test_rename_columns_event(event: RenameColumns) {
    verify_event(@event);
    verify_event(@DatabaseEvents::RenameColumns(event));
}

#[test]
#[fuzzer]
fn test_retype_column_event(event: RetypeColumn) {
    verify_event(@event);
    verify_event(@DatabaseEvents::RetypeColumn(event));
}


impl RetypeColumnsFuzzable = database::RetypeColumnsFuzzable<10, 10>;
#[test]
#[fuzzer]
fn test_retype_columns_event(event: RetypeColumns) {
    verify_event(@event);
    verify_event(@DatabaseEvents::RetypeColumns(event));
}

#[test]
#[fuzzer]
fn test_drop_column_event(event: DropColumn) {
    verify_event(@event);
    verify_event(@DatabaseEvents::DropColumn(event));
}

#[test]
#[fuzzer]
fn test_drop_columns_event(event: DropColumns) {
    verify_event(@event);
    verify_event(@DatabaseEvents::DropColumns(event));
}

#[test]
#[fuzzer]
fn test_create_index_event(event: CreateIndex) {
    verify_event(@event);
    verify_event(@DatabaseEvents::CreateIndex(event));
}

#[test]
#[fuzzer]
fn test_drop_index_event(event: DropIndex) {
    verify_event(@event);
    verify_event(@DatabaseEvents::DropIndex(event));
}

#[test]
#[fuzzer]
fn test_insert_record_event(event: InsertRecord) {
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertRecord(event));
}

#[test]
#[fuzzer]
fn test_insert_records_event(event: InsertRecords) {
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertRecords(event));
}

#[test]
#[fuzzer]
fn test_insert_field_event(event: InsertField) {
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertField(event));
}

#[test]
#[fuzzer]
fn test_insert_fields_event(event: InsertFields) {
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertFields(event));
}

#[test]
#[fuzzer]
fn test_inserts_field_event(event: InsertsField) {
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertsField(event));
}

#[test]
#[fuzzer]
fn test_inserts_fields_event(event: InsertsFields) {
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertsFields(event));
}

#[test]
#[fuzzer]
fn test_insert_field_group_event(event: InsertFieldSet) {
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertFieldSet(event));
}

#[test]
#[fuzzer]
fn test_insert_field_groups_event(event: InsertFieldSets) {
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertFieldSets(event));
}

#[test]
#[fuzzer]
fn test_inserts_field_group_event(event: InsertsFieldSet) {
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertsFieldSet(event));
}

#[test]
#[fuzzer]
fn test_inserts_field_groups_event(event: InsertsFieldSets) {
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertsFieldSets(event));
}

#[test]
#[fuzzer]
fn test_delete_record_event(event: DeleteRecord) {
    verify_event(@event);
    verify_event(@DatabaseEvents::DeleteRecord(event));
}

#[test]
#[fuzzer]
fn test_delete_records_event(event: DeleteRecords) {
    verify_event(@event);
    verify_event(@DatabaseEvents::DeleteRecords(event));
}

#[test]
#[fuzzer]
fn test_delete_field_event(event: DeleteField) {
    verify_event(@event);
    verify_event(@DatabaseEvents::DeleteField(event));
}

#[test]
#[fuzzer]
fn test_delete_fields_event(event: DeleteFields) {
    verify_event(@event);
    verify_event(@DatabaseEvents::DeleteFields(event));
}

#[test]
#[fuzzer]
fn test_deletes_field_event(event: DeletesField) {
    verify_event(@event);
    verify_event(@DatabaseEvents::DeletesField(event));
}

#[test]
#[fuzzer]
fn test_deletes_fields_event(event: DeletesFields) {
    verify_event(@event);
    verify_event(@DatabaseEvents::DeletesFields(event));
}

#[test]
#[fuzzer]
fn test_delete_field_group_event(event: DeleteFieldSet) {
    verify_event(@event);
    verify_event(@DatabaseEvents::DeleteFieldSet(event));
}

#[test]
#[fuzzer]
fn test_delete_field_groups_event(event: DeleteFieldSets) {
    verify_event(@event);
    verify_event(@DatabaseEvents::DeleteFieldSets(event));
}

#[test]
#[fuzzer]
fn test_deletes_field_group_event(event: DeletesFieldSet) {
    verify_event(@event);
    verify_event(@DatabaseEvents::DeletesFieldSet(event));
}

#[test]
#[fuzzer]
fn test_deletes_field_groups_event(event: DeletesFieldSets) {
    verify_event(@event);
    verify_event(@DatabaseEvents::DeletesFieldSets(event));
}

