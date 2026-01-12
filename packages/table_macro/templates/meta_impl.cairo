pub impl {{meta_impl}} of {{i_table_path}}::TableMeta {
    const ID: felt252 = {{table_id}};
    fn name() -> ByteArray {
        {{table_name}}
    }
    fn attributes() -> Span<{{i_path}}::Attribute> {
        {{attributes}}
    }
}
