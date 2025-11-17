use introspect_types::{Attribute, ColumnDef, IdData, PrimaryDef, PrimaryTypeDef, TypeDef};
use starknet::SyscallResultTrait;
use starknet::syscalls::emit_event_syscall;
use super::events::{IdName, IdTypeAttributes};
use super::selectors::{
    AddColumn, AddColumns, CreateFieldGroup, CreateTable, CreateTableFromClassHash,
    CreateTableWithColumns, DeleteField, DeleteFields, DeleteRecord, DeleteRecords, DeleteSchema,
    DeletesField, DeletesFields, DeletesSchema, DropColumn, DropColumns, DropTable, InsertField,
    InsertFieldGroup, InsertFieldGroups, InsertFields, InsertRecord, InsertRecords, InsertsField,
    InsertsFieldGroup, InsertsFieldGroups, InsertsFields, RenameColumn, RenameColumns,
    RenamePrimary, RenameTable, RetypeColumn, RetypeColumns, RetypePrimary,
};

pub fn emit_create_field_group(id: felt252, columns: Span<felt252>) {
    let mut data: Array<felt252> = Default::default();
    columns.serialize(ref data);
    emit_event_syscall([CreateFieldGroup, id].span(), data.span()).unwrap_syscall();
}

pub fn emit_create_table(
    id: felt252, name: ByteArray, attributes: Span<Attribute>, primary: PrimaryDef,
) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
    attributes.serialize(ref data);
    primary.serialize(ref data);
    emit_event_syscall([CreateTable, id].span(), data.span()).unwrap_syscall();
}

pub fn emit_create_table_with_columns(
    id: felt252,
    name: ByteArray,
    attributes: Span<Attribute>,
    primary: PrimaryDef,
    columns: Span<ColumnDef>,
) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
    attributes.serialize(ref data);
    primary.serialize(ref data);
    columns.serialize(ref data);
    emit_event_syscall([CreateTableWithColumns, id].span(), data.span()).unwrap_syscall();
}

pub fn emit_create_table_from_class_hash(id: felt252, name: ByteArray, class_hash: felt252) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
    class_hash.serialize(ref data);
    emit_event_syscall([CreateTableFromClassHash, id].span(), data.span()).unwrap_syscall();
}


pub fn emit_rename_table(id: felt252, name: ByteArray) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
    emit_event_syscall([RenameTable, id].span(), data.span()).unwrap_syscall();
}


pub fn emit_drop_table(id: felt252) {
    emit_event_syscall([DropTable, id].span(), [].span()).unwrap_syscall();
}


pub fn emit_rename_primary(table: felt252, name: ByteArray) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
    emit_event_syscall([RenamePrimary, table].span(), data.span()).unwrap_syscall();
}


pub fn emit_retype_primary(table: felt252, attributes: Span<Attribute>, type_def: PrimaryTypeDef) {
    let mut data: Array<felt252> = Default::default();
    attributes.serialize(ref data);
    type_def.serialize(ref data);
    emit_event_syscall([RetypePrimary, table].span(), data.span()).unwrap_syscall();
}


pub fn emit_add_column(
    table: felt252, id: felt252, name: ByteArray, attributes: Span<Attribute>, type_def: TypeDef,
) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
    attributes.serialize(ref data);
    type_def.serialize(ref data);
    emit_event_syscall([AddColumn, table, id].span(), data.span()).unwrap_syscall();
}

pub fn emit_add_columns(table: felt252, columns: Span<ColumnDef>) {
    let mut data: Array<felt252> = Default::default();
    columns.serialize(ref data);
    emit_event_syscall([AddColumns, table].span(), data.span()).unwrap_syscall();
}

pub fn emit_rename_column(table: felt252, column: felt252, name: ByteArray) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
    emit_event_syscall([RenameColumn, table, column].span(), data.span()).unwrap_syscall();
}


pub fn emit_rename_columns(table: felt252, columns: Span<IdName>) {
    let mut data: Array<felt252> = Default::default();
    columns.serialize(ref data);
    emit_event_syscall([RenameColumns, table].span(), data.span()).unwrap_syscall();
}

pub fn emit_retype_column(
    table: felt252, column: felt252, attributes: Span<Attribute>, type_def: TypeDef,
) {
    let mut data: Array<felt252> = Default::default();
    attributes.serialize(ref data);
    type_def.serialize(ref data);
    emit_event_syscall([RetypeColumn, table, column].span(), data.span()).unwrap_syscall();
}

pub fn emit_retype_columns(table: felt252, columns: Span<IdTypeAttributes>) {
    let mut data: Array<felt252> = Default::default();
    columns.serialize(ref data);
    emit_event_syscall([RetypeColumns, table].span(), data.span()).unwrap_syscall();
}

pub fn emit_drop_column(table: felt252, column: felt252) {
    emit_event_syscall([DropColumn, table, column].span(), [].span()).unwrap_syscall();
}

pub fn emit_drop_columns(table: felt252, columns: Span<felt252>) {
    let mut data: Array<felt252> = Default::default();
    columns.serialize(ref data);
    emit_event_syscall([DropColumns, table].span(), data.span()).unwrap_syscall();
}

pub fn emit_insert_record(table: felt252, record: felt252, data: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    data.serialize(ref event_data);
    emit_event_syscall([InsertRecord, table, record].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_insert_records(table: felt252, records: Span<IdData>) {
    let mut event_data: Array<felt252> = Default::default();
    records.serialize(ref event_data);
    emit_event_syscall([InsertRecords, table].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_insert_field(table: felt252, record: felt252, column: felt252, data: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    data.serialize(ref event_data);
    emit_event_syscall([InsertField, table, record, column].span(), event_data.span())
        .unwrap_syscall();
}

pub fn emit_insert_fields(
    table: felt252, record: felt252, columns: Span<felt252>, data: Span<felt252>,
) {
    let mut event_data: Array<felt252> = Default::default();
    columns.serialize(ref event_data);
    data.serialize(ref event_data);
    emit_event_syscall([InsertFields, table, record].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_inserts_field(table: felt252, column: felt252, records_data: Span<IdData>) {
    let mut event_data: Array<felt252> = Default::default();
    records_data.serialize(ref event_data);
    emit_event_syscall([InsertsField, table, column].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_inserts_fields(table: felt252, columns: Span<felt252>, records_data: Span<IdData>) {
    let mut event_data: Array<felt252> = Default::default();
    columns.serialize(ref event_data);
    records_data.serialize(ref event_data);
    emit_event_syscall([InsertsFields, table].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_insert_field_group(
    table: felt252, record: felt252, group: felt252, data: Span<felt252>,
) {
    let mut event_data: Array<felt252> = Default::default();
    data.serialize(ref event_data);
    emit_event_syscall([InsertFieldGroup, table, record, group].span(), event_data.span())
        .unwrap_syscall();
}

pub fn emit_insert_field_groups(
    table: felt252, record: felt252, groups: Span<felt252>, data: Span<felt252>,
) {
    let mut event_data: Array<felt252> = Default::default();
    groups.serialize(ref event_data);
    data.serialize(ref event_data);
    emit_event_syscall([InsertFieldGroups, table, record].span(), event_data.span())
        .unwrap_syscall();
}

pub fn emit_inserts_field_group(table: felt252, group: felt252, records_data: Span<IdData>) {
    let mut event_data: Array<felt252> = Default::default();
    records_data.serialize(ref event_data);
    emit_event_syscall([InsertsFieldGroup, table, group].span(), event_data.span())
        .unwrap_syscall();
}

pub fn emit_inserts_field_groups(
    table: felt252, groups: Span<felt252>, records_data: Span<IdData>,
) {
    let mut event_data: Array<felt252> = Default::default();
    groups.serialize(ref event_data);
    records_data.serialize(ref event_data);
    emit_event_syscall([InsertsFieldGroups, table].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_delete_record(table: felt252, record: felt252) {
    emit_event_syscall([DeleteRecord, table, record].span(), [].span()).unwrap_syscall();
}

pub fn emit_delete_records(table: felt252, records: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    records.serialize(ref event_data);
    emit_event_syscall([DeleteRecords, table].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_delete_field(table: felt252, record: felt252, column: felt252) {
    emit_event_syscall([DeleteField, table, record, column].span(), [].span()).unwrap_syscall();
}

pub fn emit_delete_fields(table: felt252, record: felt252, columns: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    columns.serialize(ref event_data);
    emit_event_syscall([DeleteFields, table, record].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_deletes_field(table: felt252, column: felt252, records: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    records.serialize(ref event_data);
    emit_event_syscall([DeletesField, table, column].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_deletes_fields(table: felt252, columns: Span<felt252>, records: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    columns.serialize(ref event_data);
    records.serialize(ref event_data);
    emit_event_syscall([DeletesFields, table].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_delete_schema(table: felt252, record: felt252, schema: felt252) {
    emit_event_syscall([DeleteSchema, table, record, schema].span(), [].span()).unwrap_syscall();
}

pub fn emit_deletes_schema(table: felt252, schema: felt252, records: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    records.serialize(ref event_data);
    emit_event_syscall([DeletesSchema, table, schema].span(), event_data.span()).unwrap_syscall();
}
