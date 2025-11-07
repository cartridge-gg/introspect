pub impl {{name}}SchemaImpl{{impl_params}} of introspect::Schema<{{name}}{{params}}> {
    fn columns() -> Span<introspect::ColumnDef> {
        [{{column_defs}}].span()
    }

    fn child_defs() -> Array<(felt252, introspect::TypeDef)> {
        {{child_defs}}
    }
}