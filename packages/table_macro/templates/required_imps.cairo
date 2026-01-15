
{{columns_mod}}

pub impl {{impl_name}}TableSchema = {{i_table_path}}::TableSchemaImpl<{{struct_name}}, {{meta_impl_name}}, {{primary_impl_name}}, {{columns_impl_name}}>;

pub impl {{impl_name}}Table = {{i_table_path}}::TableImpl<{{impl_name}}TableSchema>;

{{member_impls}}

impl {{impl_name}}RecordValuesSpan<impl T: {{i_table_path}}::Table[Record: {{struct_name}}]> of {{i_table_path}}::RecordValuesSpanTrait<T, T::Record> {
    fn serialize_values(self: @T::Record, ref data: Array<felt252>) {
        {{serialize_member_calls}}
    }
}