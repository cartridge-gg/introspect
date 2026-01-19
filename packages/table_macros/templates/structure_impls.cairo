pub mod {{columns_mod_name}}{
    {{column_consts}}
}

impl {{struct_impl_name}} of {{i_table_path}}::TableStructure {
    type Primary = {{primary_ty}};
    type Record = {{struct_name}};
    fn primary() -> {{i_path}}::PrimaryDef {
        {{i_path}}::primary_def({{primary_name}}, {{primary_attributes}}, {{primary_type_def}})
    }
    fn columns() -> Span<{{i_path}}::ColumnDef> {
        {{column_defs}}
    }
    fn collect_child_defs(ref defs: {{i_path}}::ChildDefs) {
        {{child_defs}}
    }
}

{{member_impls}}

impl {{struct_name}}RecordValuesSpan of {{i_table_path}}::RecordValuesSpanTrait<{{struct_impl_name}}, {{struct_name}}> {
    fn serialize_values(self: @{{struct_name}}, ref data: Array<felt252>) {
        {{serialize_member_calls}}
    }
}

