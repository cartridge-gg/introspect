


impl {{struct_name}}RecordKey of {{i_table_path}}::RecordKey<{{struct_impl_name}}::Record, @{{ty}}, {{struct_impl_name}}> {
    type Key = {{ty}};
    fn record_key(self: @{{struct_impl_name}}::Record) -> @{{ty}} {
        self.{{key_member}}
    }
}

impl {{struct_name}}SerialisedKey<
    K, +{{i_table_path}}::Snapable<@KS, {{ty}}>,
> of {{i_table_path}}::SerialisedKey<{{struct_impl_name}}::Record, K, {{struct_impl_name}}> {
    fn serialize_key(self: @K, ref data: Array<felt252>) {
        {{member_impl_name}}::serialize_member(self, ref data);
    }
}
