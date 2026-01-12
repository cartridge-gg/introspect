impl {{columns_impl}} of introspect_table::TableColumns {
    type Column = {{column_enum}};
    fn columns() -> Span<introspect_types::ColumnDef> {
        {{column_defs}}
    }
    fn child_defs() -> Array<(felt252, introspect_types::TypeDef)> {
        {{child_defs}}
    }
}