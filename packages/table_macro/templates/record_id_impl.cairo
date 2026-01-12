impl {{impl_name}}RecordId of {{i_path}}::RecordId<{}, {{table_impl}}> {
    fn record_id(self: @Character) -> felt252 {
        {{record_id_expr}}
    }
}