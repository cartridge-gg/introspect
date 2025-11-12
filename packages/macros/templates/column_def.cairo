introspect::ColumnDef {
    id: {{id}},
    name: "{{name}}",
    attributes: [{{attributes_str}}].span(),
    type_def: introspect::Introspect::<{{type_def}}>::type_def(),
}