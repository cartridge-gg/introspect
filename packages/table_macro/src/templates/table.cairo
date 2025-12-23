impl {{table_name}}Table{{impl_params}} of introspect_framework::ITable<{{full_name}}> {
    const SELECTOR: felt252 = selector!("{{table_name}}");
    fn name() -> ByteArray {
        "{{table_name}}"
    }

    fn attributes() -> Span<introspect::Attribute> {
        [{{table_attributes}}].span()
    }

    fn primary() -> introspect::PrimaryDef {
        {{primary_def}}
    }

    fn columns() -> Span<introspect::ColumnDef> {
        [{{column_defs}}].span()
    }

    fn child_defs() -> Array<(felt252, introspect::TypeDef)> {
        {{child_defs}}
    }
}