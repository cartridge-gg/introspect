pub impl {{table_impl_name}} of {{i_table_path}}::ITable {
    impl Struct = {{struct_impl_name}};
    const ID: felt252 = {{table_id}};
    fn name() -> ByteArray {
        {{table_name}}
    }
    fn append_table_attributes(ref attributes: Array<{{i_path}}::Attribute>) {
        {{table_attributes}}
    }
}