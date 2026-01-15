impl {{impl_name}}RecordId of {{i_table_path}}::RecordId<{{impl_name}}Table::Record, {{impl_name}}Table> {
    fn record_id(self: @{{struct_name}}) -> felt252 {
        introspect_types::PrimaryTrait::to_felt252(self.{{primary_field}})
    }
}
