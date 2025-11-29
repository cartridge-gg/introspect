use core::fmt::Debug;
use introspect_events::database::events::{IdName, IdTypeAttributes};
use introspect_events::database::{
    AddColumn, AddColumns, CreateFieldGroup, CreateIndex, CreateTable, CreateTableFromClassHash,
    CreateTableWithColumns, DatabaseEvents, DeleteField, DeleteFieldGroup, DeleteFieldGroups,
    DeleteFields, DeleteRecord, DeleteRecords, DeletesField, DeletesFieldGroup, DeletesFieldGroups,
    DeletesFields, DropColumn, DropColumns, DropIndex, DropTable, InsertField, InsertFieldGroup,
    InsertFieldGroups, InsertFields, InsertRecord, InsertRecords, InsertsField, InsertsFieldGroup,
    InsertsFieldGroups, InsertsFields, RenameColumn, RenameColumns, RenamePrimary, RenameTable,
    RetypeColumn, RetypeColumns, RetypePrimary,
};
use introspect_types::{Attribute, ColumnDef, IdData, PrimaryDef, PrimaryTypeDef, TypeDef};
use starknet::Event;

fn verify_event<E, +Event<E>, +PartialEq<E>, +Drop<E>, +Debug<E>>(event: @E) {
    let mut keys: Array<felt252> = Default::default();
    let mut data: Array<felt252> = Default::default();
    event.append_keys_and_data(ref keys, ref data);
    let mut keys = keys.span();
    let mut data = data.span();
    let deserialized = Event::<E>::deserialize(ref keys, ref data).unwrap();
    assert_eq!(@deserialized, event);
}

fn key_attribute() -> Attribute {
    Attribute { name: "key", data: None }
}

fn felt252_column() -> ColumnDef {
    ColumnDef {
        id: 1, name: "felt_col", attributes: [key_attribute()].span(), type_def: TypeDef::Felt252,
    }
}

fn byte_array_column() -> ColumnDef {
    ColumnDef {
        id: 2,
        name: "byte_array_col",
        attributes: [key_attribute()].span(),
        type_def: TypeDef::Utf8String,
    }
}

#[test]
fn test_create_field_group_event() {
    let event = CreateFieldGroup { id: 5, columns: [1, 2, 34, 5, 6].span() };
    verify_event(@event);
    verify_event(@DatabaseEvents::CreateFieldGroup(event));
}

#[test]
fn test_create_table_event() {
    let event = CreateTable {
        id: 12,
        name: "TestTable",
        attributes: [Attribute { name: "key", data: None }].span(),
        primary: PrimaryDef {
            name: "id", attributes: [].span(), type_def: PrimaryTypeDef::Felt252,
        },
    };
    verify_event(@event);
    verify_event(@DatabaseEvents::CreateTable(event));
}

#[test]
fn test_create_table_with_columns_event() {
    let event = CreateTableWithColumns {
        id: 13,
        name: "TableWithCols",
        attributes: [key_attribute()].span(),
        primary: PrimaryDef {
            name: "id", attributes: [].span(), type_def: PrimaryTypeDef::Felt252,
        },
        columns: [felt252_column(), byte_array_column()].span(),
    };
    verify_event(@event);
    verify_event(@DatabaseEvents::CreateTableWithColumns(event));
}

#[test]
fn test_create_table_from_class_hash_event() {
    let event = CreateTableFromClassHash {
        id: 14, name: "ClassHashTable", class_hash: 0x123456789abcdef,
    };
    verify_event(@event);
    verify_event(@DatabaseEvents::CreateTableFromClassHash(event));
}

#[test]
fn test_rename_table_event() {
    let event = RenameTable { id: 15, name: "RenamedTable" };
    verify_event(@event);
    verify_event(@DatabaseEvents::RenameTable(event));
}

#[test]
fn test_drop_table_event() {
    let event = DropTable { id: 16 };
    verify_event(@event);
    verify_event(@DatabaseEvents::DropTable(event));
}

#[test]
fn test_rename_primary_event() {
    let event = RenamePrimary { table: 17, name: "new_primary_name" };
    verify_event(@event);
    verify_event(@DatabaseEvents::RenamePrimary(event));
}

#[test]
fn test_retype_primary_event() {
    let event = RetypePrimary {
        table: 18, attributes: [key_attribute()].span(), type_def: PrimaryTypeDef::ClassHash,
    };
    verify_event(@event);
    verify_event(@DatabaseEvents::RetypePrimary(event));
}

#[test]
fn test_add_column_event() {
    let event = AddColumn {
        table: 19,
        id: 1,
        name: "new_column",
        attributes: [key_attribute()].span(),
        type_def: TypeDef::U128,
    };
    verify_event(@event);
    verify_event(@DatabaseEvents::AddColumn(event));
}

#[test]
fn test_add_columns_event() {
    let event = AddColumns { table: 20, columns: [felt252_column(), byte_array_column()].span() };
    verify_event(@event);
    verify_event(@DatabaseEvents::AddColumns(event));
}

#[test]
fn test_rename_column_event() {
    let event = RenameColumn { table: 21, id: 1, name: "renamed_column" };
    verify_event(@event);
    verify_event(@DatabaseEvents::RenameColumn(event));
}

#[test]
fn test_rename_columns_event() {
    let event = RenameColumns {
        table: 22, columns: [IdName { id: 1, name: "col1" }, IdName { id: 2, name: "col2" }].span(),
    };
    verify_event(@event);
    verify_event(@DatabaseEvents::RenameColumns(event));
}

#[test]
fn test_retype_column_event() {
    let event = RetypeColumn {
        table: 23, id: 1, attributes: [key_attribute()].span(), type_def: TypeDef::Bool,
    };
    verify_event(@event);
    verify_event(@DatabaseEvents::RetypeColumn(event));
}

#[test]
fn test_retype_columns_event() {
    let event = RetypeColumns {
        table: 24,
        columns: [
            IdTypeAttributes {
                id: 1, attributes: [key_attribute()].span(), type_def: TypeDef::U64,
            },
            IdTypeAttributes { id: 2, attributes: [].span(), type_def: TypeDef::U32 },
        ]
            .span(),
    };
    verify_event(@event);
    verify_event(@DatabaseEvents::RetypeColumns(event));
}

#[test]
fn test_drop_column_event() {
    let event = DropColumn { table: 25, column: 1 };
    verify_event(@event);
    verify_event(@DatabaseEvents::DropColumn(event));
}

#[test]
fn test_drop_columns_event() {
    let event = DropColumns { table: 26, columns: [1, 2, 3].span() };
    verify_event(@event);
    verify_event(@DatabaseEvents::DropColumns(event));
}

#[test]
fn test_create_index_event() {
    let event = CreateIndex { table: 27, id: 200, name: "test_index", columns: [1, 2].span() };
    verify_event(@event);
    verify_event(@DatabaseEvents::CreateIndex(event));
}

#[test]
fn test_drop_index_event() {
    let event = DropIndex { table: 28, id: 201 };
    verify_event(@event);
    verify_event(@DatabaseEvents::DropIndex(event));
}

#[test]
fn test_insert_record_event() {
    let event = InsertRecord { table: 29, record: 100, data: [1, 2, 3].span() };
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertRecord(event));
}

#[test]
fn test_insert_records_event() {
    let event = InsertRecords {
        table: 30,
        records_data: [
            IdData { id: 100, data: [1, 2].span() }, IdData { id: 101, data: [3, 4].span() },
        ]
            .span(),
    };
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertRecords(event));
}

#[test]
fn test_insert_field_event() {
    let event = InsertField { table: 31, column: 1, record: 100, data: [42].span() };
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertField(event));
}

#[test]
fn test_insert_fields_event() {
    let event = InsertFields {
        table: 32, record: 100, columns: [1, 2].span(), data: [10, 20].span(),
    };
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertFields(event));
}

#[test]
fn test_inserts_field_event() {
    let event = InsertsField {
        table: 33,
        column: 1,
        records_data: [IdData { id: 100, data: [1].span() }, IdData { id: 101, data: [2].span() }]
            .span(),
    };
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertsField(event));
}

#[test]
fn test_inserts_fields_event() {
    let event = InsertsFields {
        table: 34,
        columns: [1, 2].span(),
        records_data: [IdData { id: 12, data: [1, 23, 4].span() }].span(),
    };
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertsFields(event));
}

#[test]
fn test_insert_field_group_event() {
    let event = InsertFieldGroup { table: 35, record: 100, group: 5, data: [1, 2].span() };
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertFieldGroup(event));
}

#[test]
fn test_insert_field_groups_event() {
    let event = InsertFieldGroups {
        table: 36, record: 100, groups: [5, 6].span(), data: [1, 2, 3, 4].span(),
    };
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertFieldGroups(event));
}

#[test]
fn test_inserts_field_group_event() {
    let event = InsertsFieldGroup {
        table: 37,
        group: 5,
        records_data: [IdData { id: 100, data: [1].span() }, IdData { id: 101, data: [2].span() }]
            .span(),
    };
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertsFieldGroup(event));
}

#[test]
fn test_inserts_field_groups_event() {
    let event = InsertsFieldGroups {
        table: 38,
        record: 100,
        groups: [5, 6].span(),
        records_data: [IdData { id: 100, data: [1, 2].span() }].span(),
    };
    verify_event(@event);
    verify_event(@DatabaseEvents::InsertsFieldGroups(event));
}

#[test]
fn test_delete_record_event() {
    let event = DeleteRecord { table: 39, record: 100 };
    verify_event(@event);
    verify_event(@DatabaseEvents::DeleteRecord(event));
}

#[test]
fn test_delete_records_event() {
    let event = DeleteRecords { table: 40, records: [100, 101, 102].span() };
    verify_event(@event);
    verify_event(@DatabaseEvents::DeleteRecords(event));
}

#[test]
fn test_delete_field_event() {
    let event = DeleteField { table: 41, record: 100, column: 1 };
    verify_event(@event);
    verify_event(@DatabaseEvents::DeleteField(event));
}

#[test]
fn test_delete_fields_event() {
    let event = DeleteFields { table: 42, record: 100, columns: [1, 2].span() };
    verify_event(@event);
    verify_event(@DatabaseEvents::DeleteFields(event));
}

#[test]
fn test_deletes_field_event() {
    let event = DeletesField { table: 43, column: 1, records: [100, 101].span() };
    verify_event(@event);
    verify_event(@DatabaseEvents::DeletesField(event));
}

#[test]
fn test_deletes_fields_event() {
    let event = DeletesFields { table: 44, records: [100, 101].span(), columns: [1, 2].span() };
    verify_event(@event);
    verify_event(@DatabaseEvents::DeletesFields(event));
}

#[test]
fn test_delete_field_group_event() {
    let event = DeleteFieldGroup { table: 45, record: 100, group: 5 };
    verify_event(@event);
    verify_event(@DatabaseEvents::DeleteFieldGroup(event));
}

#[test]
fn test_delete_field_groups_event() {
    let event = DeleteFieldGroups { table: 46, record: 100, groups: [5, 6].span() };
    verify_event(@event);
    verify_event(@DatabaseEvents::DeleteFieldGroups(event));
}

#[test]
fn test_deletes_field_group_event() {
    let event = DeletesFieldGroup { table: 47, group: 5, records: [100, 101].span() };
    verify_event(@event);
    verify_event(@DatabaseEvents::DeletesFieldGroup(event));
}

#[test]
fn test_deletes_field_groups_event() {
    let event = DeletesFieldGroups { table: 48, records: [100, 101].span(), groups: [5, 6].span() };
    verify_event(@event);
    verify_event(@DatabaseEvents::DeletesFieldGroups(event));
}
