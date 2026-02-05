pub mod database;
pub mod multipart;
pub mod types;
pub mod utils;
pub mod variable;
pub use database::*;
pub use types::*;
pub use variable::*;


pub trait EmitEvent<T> {
    fn emit(self: @T);
}
use starknet::SyscallResultTrait;
mod emit_event_impl {
    use starknet::SyscallResultTrait;
    pub impl EmitEventImpl<T, const SELECTOR: felt252, +starknet::Event<T>> of super::EmitEvent<T> {
        fn emit(self: @T) {
            let mut keys = array![SELECTOR];
            let mut data: Array<felt252> = Default::default();
            starknet::Event::append_keys_and_data(self, ref keys, ref data);
            starknet::syscalls::emit_event_syscall(keys.span(), data.span()).unwrap_syscall()
        }
    }
}

pub trait EmitRawEvent<T> {
    fn emit_event_data(self: @felt252, data: T);
}

impl EmitEventDataSpanImpl of EmitRawEvent<Span<felt252>> {
    fn emit_event_data(self: @felt252, data: Span<felt252>) {
        emit_ispec_event(*self, data);
    }
}

impl EmitEventDataArrayImpl of EmitRawEvent<Array<felt252>> {
    fn emit_event_data(self: @felt252, data: Array<felt252>) {
        emit_ispec_event(*self, data.span());
    }
}

impl EmitEventDataFixedSizeArrayImpl<const SIZE: u32> of EmitRawEvent<[felt252; SIZE]> {
    fn emit_event_data(self: @felt252, data: [felt252; SIZE]) {
        emit_ispec_event(*self, data.span());
    }
}

pub fn emit_ispec_event(key: felt252, data: Span<felt252>) {
    starknet::syscalls::emit_event_syscall([key].span(), data).unwrap_syscall();
}
