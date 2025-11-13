use introspect_types::{ColumnDef, TypeDef};
use starknet::syscalls::emit_event_syscall;
use starknet::{ClassHash, SyscallResultTrait};

pub fn emit_declare_schema(id: felt252, columns: Span<ColumnDef>) {
    let mut data: Array<felt252> = Default::default();
    columns.serialize(ref data);
    emit_event_syscall([id].span(), data.span()).unwrap_syscall();
}

pub fn emit_declare_type(id: felt252, type_def: TypeDef) {
    let mut data: Array<felt252> = Default::default();
    type_def.serialize(ref data);
    emit_event_syscall([id].span(), data.span()).unwrap_syscall();
}

pub fn emit_declare_type_from_class(class_hash: ClassHash) {
    emit_event_syscall([class_hash.into()].span(), [].span()).unwrap_syscall();
}

pub fn emit_declare_schema_from_class(class_hash: ClassHash) {
    emit_event_syscall([class_hash.into()].span(), [].span()).unwrap_syscall();
}
