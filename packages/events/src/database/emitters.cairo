use introspect_types::{Attribute, ColumnDef, PrimaryDef, Schema};
use starknet::SyscallResultTrait;
use starknet::syscalls::emit_event_syscall;

pub fn emit_create_table(id: felt252, name: ByteArray, primary: PrimaryDef) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
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
