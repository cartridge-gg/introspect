introspect::ColumnDef {
    id: {{id}},
    name: "{{name}}",
    attrs: [{{attrs_str}}].span(),
    type_def: introspect::Introspect::<{{type_def}}>::type_def(),
}