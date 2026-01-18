impl {{impl_name}}RecordKey of {{i_path}}::RecordKey<{{struct_name}}, {{key_types_ss}}, FooTable> {
    type Key = {{key_type}};
    fn record_key(self: @{{struct_name}}) -> {{key_types_ss}} {
        {{self_key_expr}}
    }
}

impl {{impl_name}}SerialisedKey<
    KS,
    {{ks}}
    +introspect_table::Snapable<@KS, {{}}>,
    {{snapables}}
> of introspect_table::SerialisedKey<{{table_impl}}::Record, KS, {{table_impl}}> {
    fn serialize_key(self: @KS, ref data: Array<felt252>) {
        let {{key_expr}} = self.snapshot();
        {{serialize_calls}}
    }
}