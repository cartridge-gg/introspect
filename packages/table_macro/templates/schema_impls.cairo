pub mod {{struct_name}}Columns{
    {{column_consts}}
}

impl {{struct_name}}Members of {{i_table_path}}::TableMembers {
    type Primary = {{primary_ty}};
    fn primary_def() -> {{i_path}}::PrimaryDef {
        {{i_path}}::primary_def({{primary_name}}, {{primary_attributes}}, {{primary_type_def}})
    }
    fn columns() -> Span<{{i_path}}::ColumnDef> {
        {{column_defs}}
    }
    fn child_defs() -> Array<(felt252, {{i_path}}::TypeDef)> {
        {{child_defs}}
    }
}

{{member_impls}}

impl {{struct_name}}RecordValuesSpan<impl T: {{i_table_path}}::Table[Record: {{struct_name}}]> of {{i_table_path}}::RecordValuesSpanTrait<T, T::Record> {
    fn serialize_values(self: @T::Record, ref data: Array<felt252>) {
        {{serialize_member_calls}}
    }
}

