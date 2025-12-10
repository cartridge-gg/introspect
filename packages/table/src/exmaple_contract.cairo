#[starknet::contract]
mod example_contract {
    use crate::example_gen::{Bar, BarTable, Foo, Foo2Table, FooTable};
    #[storage]
    struct Storage {}


    #[constructor]
    fn constructor(ref self: ContractState) {
        FooTable::register_table();
        Foo2Table::register_table();
        BarTable::register_table();
    }

    #[external(v0)]
    fn update_foo(ref self: ContractState, a_key: u128, name: ByteArray, something: ByteArray) {
        let foo = Foo { a_key, name, something };
        FooTable::insert_record(@foo);
    }

    #[external(v0)]
    fn update_bar(ref self: ContractState, id: felt252, name: ByteArray, something: ByteArray) {
        let bar = Bar { name, something };
        BarTable::insert_record(@(id, @bar));
    }
}
