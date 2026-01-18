pub impl {{table_impl_name}} of {{i_path}}::ITable {
    impl Struct = {{struct_impl_name}};
    const ID: felt252 = {{table_id}};
    fn name() -> ByteArray {
        {{table_name}}
    }
}