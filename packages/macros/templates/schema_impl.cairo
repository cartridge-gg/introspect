pub impl Gen{{name}}SchemaImpl{{impl_params}} of introspect::Schema<{{full_name}}> {
    fn columns() -> Span<introspect::ColumnDef> {
        [{{column_defs}}].span()
    }

    fn child_defs() -> Array<(felt252, introspect::TypeDef)> {
        {{child_defs}}
    }

    fn record_data(self: @{{full_name}}) -> Span<felt252> {
        self.iserialize_inline()
    }
}