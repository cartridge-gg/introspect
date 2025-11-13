use introspect_types::TypeDef;
use starknet::SyscallResultTrait;
use starknet::syscalls::emit_event_syscall;

pub fn emit_register_variable(id: felt252, name: ByteArray, type_def: TypeDef) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
    type_def.serialize(ref data);
    emit_event_syscall([id].span(), data.span()).unwrap_syscall();
}


pub fn emit_declare_variable(id: felt252, name: ByteArray, type_def: TypeDef, data: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    name.serialize(ref event_data);
    type_def.serialize(ref event_data);
    data.serialize(ref event_data);
    emit_event_syscall([id].span(), event_data.span()).unwrap_syscall();
}


pub fn emit_set_variable(id: felt252, data: Span<felt252>) {
    let mut event_data: Array<felt252> = Default::default();
    data.serialize(ref event_data);
    emit_event_syscall([id].span(), event_data.span()).unwrap_syscall();
}


pub fn emit_rename_variable(id: felt252, name: ByteArray) {
    let mut data: Array<felt252> = Default::default();
    name.serialize(ref data);
    emit_event_syscall([id].span(), data.span()).unwrap_syscall();
}


pub fn emit_delete_variable(id: felt252) {
    emit_event_syscall([id].span(), [].span()).unwrap_syscall();
}
