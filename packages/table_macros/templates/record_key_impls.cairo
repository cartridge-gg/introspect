pub impl {{struct_name}}Keyed of {{i_table_path}}::RecordKeySerialized<{{struct_impl_name}}> {
    type Value = ({{key_types}});
    type Snapped = ({{snapped_key_types}});
    fn record_key(self: @{{struct_impl_name}}::Record) -> Self::Snapped {
        ({{self_key_members}})
    }
    fn serialize_key(self: Self::Snapped, ref data: Array<felt252>) {
        let ({{key_members}}) = self;
        {{serialize_calls}}
    }
}