impl {{impl_name}}<impl Table: {{i_table_path}}::TableStructure, {{member_impls}}> of {{i_table_path}}::ValueColumnSet<Table, {{struct_name}}, {{size}}> {
    const COLUMN_IDS: [felt252; {{size}}] = [{{column_ids}}];
    const GROUP_ID: felt252 = {{i_table_path}}::poseidon_hash_fixed_array(Self::COLUMN_IDS);

    fn serialize_set_value(self: @{{struct_name}}, ref data: Array<felt252>) {
        {{serialize_member_calls}}
    }

    fn column_ids() -> Span<felt252> {
        Self::COLUMN_IDS.span()
    }
}
