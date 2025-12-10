pub mod database;
// pub mod multipart;
pub mod types {
    // pub mod emitters;
    pub mod events;
    // pub use emitters::*;
    pub use events::{*, selectors};
}
pub mod variable {
    pub mod emitters;
    pub mod events;
    pub use emitters::*;
    pub use events::*;
}
pub mod utils;
pub use database::*;
// pub use multipart::*;
pub use types::*;
pub use variable::*;


pub trait EmitEvent<T> {
    fn emit_event(self: @T);
}
mod emit_event_impl {
    use starknet::SyscallResultTrait;
    pub impl EmitEventImpl<T, const SELECTOR: felt252, +starknet::Event<T>> of super::EmitEvent<T> {
        fn emit_event(self: @T) {
            let mut keys = array![SELECTOR];
            let mut data: Array<felt252> = Default::default();
            starknet::Event::append_keys_and_data(self, ref keys, ref data);
            starknet::syscalls::emit_event_syscall(keys.span(), data.span()).unwrap_syscall()
        }
    }
}
