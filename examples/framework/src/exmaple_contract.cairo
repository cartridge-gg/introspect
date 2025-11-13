#[starknet::contract]
mod example_contract {
    use crate::example_gen::{Bar, BarTable, Foo, FooTable};
    #[storage]
    struct Storage {}


    #[constructor]
    fn constructor(ref self: ContractState) {
        FooTable::register_table();
    }

    #[external(v0)]
    fn update_foo(ref self: ContractState, id: felt252, name: ByteArray, something: ByteArray) {
        let foo = Foo { id: id, name: name, something: something };
        FooTable::insert_record(@foo);
    }

    #[external(v0)]
    fn update_bar(ref self: ContractState, id: felt252, name: ByteArray, something: ByteArray) {
        let bar = Bar { name: name, something: something };
        BarTable::insert_record(@(id, @bar));
    }
}
