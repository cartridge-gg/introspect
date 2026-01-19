impl {{struct_name}}ColumnGroupImpl<
    impl Struct: introspect_table::TableStructure,
    {{member_impls}}
    impl SK: introspect_table::SerialisedKey<Struct::Record, {{key_type_ss}}, Struct>,
    impl KI: introspect_table::KeySpanToId<Struct::Record, Struct>,
> of introspect_table::IdColumnGroup<{{struct_name}}, {{size}}, Struct> {
    const GROUP_ID: felt252 = {{group_id}};
    const COLUMN_IDS: [felt252; {{size}}] = [{{column_ids}}];
    fn group_tuple(self: @{{struct_name}}) -> (felt252, Span<felt252>) {
        let mut data: Array<felt252> = Default::default();
        SK::serialize_key(@{{self_key_members}}, ref data);
        let id = KI::key_span_to_id(data.span());
        {{serialize_member_calls}}
        (id, data.span())
    }
}
