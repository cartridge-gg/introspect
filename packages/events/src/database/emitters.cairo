use introspect_types::{Attribute, ColumnDef, PrimaryDef, PrimaryTypeDef, TypeDef};
use starknet::SyscallResultTrait;
use starknet::syscalls::emit_event_syscall;
use super::events::{IdName, IdTypeAttributes, RecordData};

pub fn emit_create_table(
    id: felt252, name: ByteArray, attributes: Span<Attribute>, primary: PrimaryDef,
) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
    attributes.serialize(ref data);
    primary.serialize(ref data);
    emit_event_syscall([id].span(), data.span()).unwrap_syscall();
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
    emit_event_syscall([id].span(), data.span()).unwrap_syscall();
}

pub fn emit_create_table_with_schema(
    id: felt252, name: ByteArray, attributes: Span<Attribute>, primary: PrimaryDef, schema: felt252,
) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
    attributes.serialize(ref data);
    primary.serialize(ref data);
    schema.serialize(ref data);
    emit_event_syscall([id].span(), data.span()).unwrap_syscall();
}


pub fn emit_rename_table(id: felt252, name: ByteArray) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
    emit_event_syscall([id].span(), data.span()).unwrap_syscall();
}


pub fn emit_drop_table(id: felt252) {
    emit_event_syscall([id].span(), [].span()).unwrap_syscall();
}


pub fn emit_rename_primary(table: felt252, name: ByteArray) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
    emit_event_syscall([table].span(), data.span()).unwrap_syscall();
}


pub fn emit_retype_primary(table: felt252, attributes: Span<Attribute>, type_def: PrimaryTypeDef) {
    let mut data: Array<felt252> = Default::default();
    attributes.serialize(ref data);
    type_def.serialize(ref data);
    emit_event_syscall([table].span(), data.span()).unwrap_syscall();
}


pub fn emit_add_column(
    table: felt252, id: felt252, name: ByteArray, attributes: Span<Attribute>, type_def: TypeDef,
) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
    attributes.serialize(ref data);
    type_def.serialize(ref data);
    emit_event_syscall([table, id].span(), data.span()).unwrap_syscall();
}

pub fn emit_add_columns(table: felt252, columns: Span<ColumnDef>) {
    let mut data: Array<felt252> = Default::default();
    columns.serialize(ref data);
    emit_event_syscall([table].span(), data.span()).unwrap_syscall();
}

pub fn emit_rename_column(table: felt252, column: felt252, name: ByteArray) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
    emit_event_syscall([table, column].span(), data.span()).unwrap_syscall();
}


pub fn emit_rename_columns(table: felt252, columns: Span<IdName>) {
    let mut data: Array<felt252> = Default::default();
    columns.serialize(ref data);
    emit_event_syscall([table].span(), data.span()).unwrap_syscall();
}

pub fn emit_retype_column(
    table: felt252, column: felt252, attributes: Span<Attribute>, type_def: TypeDef,
) {
    let mut data: Array<felt252> = Default::default();
    attributes.serialize(ref data);
    type_def.serialize(ref data);
    emit_event_syscall([table, column].span(), data.span()).unwrap_syscall();
}

pub fn emit_retype_columns(table: felt252, columns: Span<IdTypeAttributes>) {
    let mut data: Array<felt252> = Default::default();
    columns.serialize(ref data);
    emit_event_syscall([table].span(), data.span()).unwrap_syscall();
}

pub fn emit_drop_column(table: felt252, column: felt252) {
    emit_event_syscall([table, column].span(), [].span()).unwrap_syscall();
}

pub fn emit_drop_columns(table: felt252, columns: Span<felt252>) {
    let mut data: Array<felt252> = Default::default();
    columns.serialize(ref data);
    emit_event_syscall([table].span(), data.span()).unwrap_syscall();
}

pub fn emit_insert_record(table: felt252, record: felt252, data: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    data.serialize(ref event_data);
    emit_event_syscall([table, record].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_insert_records(table: felt252, records: Span<RecordData>) {
    let mut event_data: Array<felt252> = Default::default();
    records.serialize(ref event_data);
    emit_event_syscall([table].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_insert_field(table: felt252, record: felt252, column: felt252, data: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    data.serialize(ref event_data);
    emit_event_syscall([table, record, column].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_insert_fields(
    table: felt252, record: felt252, columns: Span<felt252>, data: Span<felt252>,
) {
    let mut event_data: Array<felt252> = Default::default();
    columns.serialize(ref event_data);
    data.serialize(ref event_data);
    emit_event_syscall([table, record].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_inserts_field(table: felt252, column: felt252, records_data: Span<RecordData>) {
    let mut event_data: Array<felt252> = Default::default();
    records_data.serialize(ref event_data);
    emit_event_syscall([table, column].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_inserts_fields(table: felt252, columns: Span<felt252>, records_data: Span<RecordData>) {
    let mut event_data: Array<felt252> = Default::default();
    columns.serialize(ref event_data);
    records_data.serialize(ref event_data);
    emit_event_syscall([table].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_insert_schema(table: felt252, record: felt252, schema: felt252, data: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    data.serialize(ref event_data);
    emit_event_syscall([table, record, schema].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_inserts_schema(table: felt252, schema: felt252, records_data: Span<RecordData>) {
    let mut event_data: Array<felt252> = Default::default();
    records_data.serialize(ref event_data);
    emit_event_syscall([table, schema].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_delete_record(table: felt252, record: felt252) {
    emit_event_syscall([table, record].span(), [].span()).unwrap_syscall();
}

pub fn emit_delete_records(table: felt252, records: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    records.serialize(ref event_data);
    emit_event_syscall([table].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_delete_field(table: felt252, record: felt252, column: felt252) {
    emit_event_syscall([table, record, column].span(), [].span()).unwrap_syscall();
}

pub fn emit_delete_fields(table: felt252, record: felt252, columns: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    columns.serialize(ref event_data);
    emit_event_syscall([table, record].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_deletes_field(table: felt252, column: felt252, records: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    records.serialize(ref event_data);
    emit_event_syscall([table, column].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_deletes_fields(table: felt252, columns: Span<felt252>, records: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    columns.serialize(ref event_data);
    records.serialize(ref event_data);
    emit_event_syscall([table].span(), event_data.span()).unwrap_syscall();
}

pub fn emit_delete_schema(table: felt252, record: felt252, schema: felt252) {
    emit_event_syscall([table, record, schema].span(), [].span()).unwrap_syscall();
}

pub fn emit_deletes_schema(table: felt252, schema: felt252, records: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    records.serialize(ref event_data);
    emit_event_syscall([table, schema].span(), event_data.span()).unwrap_syscall();
}
