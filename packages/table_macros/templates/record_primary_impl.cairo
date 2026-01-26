impl {{struct_name}}RecordPrimary of {{i_table_path}}::RecordPrimary<{{struct_impl_name}}, {{struct_name}}> {
    fn record_primary(self: @{{struct_name}}) -> @{{struct_impl_name}}::Primary {
        self.{{primary_member_name}}
    }
}