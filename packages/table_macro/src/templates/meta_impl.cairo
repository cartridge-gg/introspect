pub impl {{meta_impl}} of introspect_table::TableMeta {
    const ID: felt252 = {{table_id}};
    fn name() -> ByteArray {
        "{{table_name}}"
    }
    fn attributes() -> Span<introspect_types::Attribute> {
        {{attributes}}
    }
}
