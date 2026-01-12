impl {{columns_impl}} of {{i_table_path}}::TableColumns {
    fn columns() -> Span<{{i_path}}::ColumnDef> {
        {{column_defs}}
    }
    fn child_defs() -> Array<(felt252, {{i_path}}::TypeDef)> {
        {{child_defs}}
    }
}