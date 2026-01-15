impl {{struct_name}}RecordId of {{i_table_path}}::RecordId<{{struct_name}}, {{struct_impl_name}}> {
    fn record_id(self: @{{struct_name}}) -> felt252 {
        introspect_types::PrimaryTrait::to_felt252(self.{{primary_member}})
    }
}