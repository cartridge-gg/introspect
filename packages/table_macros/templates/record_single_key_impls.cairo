pub impl {{struct_name}}Keyed of {{i_table_path}}::RecordKeySerialized<{{struct_impl_name}}> {
    type Value = {{ty}};
    type Snapped = @{{ty}};
    fn record_key(self: @{{struct_impl_name}}::Record) -> Self::Snapped {
        self.{{key_member}}
    }
    fn serialize_key(self: Self::Snapped, ref data: Array<felt252>) {
        {{member_impl_name}}::serialize_member(self, ref data);
    }
}