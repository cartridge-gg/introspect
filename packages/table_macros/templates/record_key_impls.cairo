impl {{struct_name}}RecordKey of {{i_table_path}}::RecordKey<{{struct_impl_name}}::Record, ({{key_types_ss}}), {{struct_impl_name}}> {
    type Key = ({{key_types}});
    fn record_key(self: @{{struct_impl_name}}::Record) -> ({{key_types_ss}}) {
        ({{self_key_members}})
    }
}

impl {{struct_name}}SerialisedKey<
    KS,
    {{generics}},
    +{{i_table_path}}::Snapable<@KS, ({{generics}})>,
    {{snappables}}
> of {{i_table_path}}::SerialisedKey<{{struct_impl_name}}::Record, KS, {{struct_impl_name}}> {
    fn serialize_key(self: @KS, ref data: Array<felt252>) {
        let ({{key_members}}) = self.snapshot();
        {{serialize_calls}}
    }
}
