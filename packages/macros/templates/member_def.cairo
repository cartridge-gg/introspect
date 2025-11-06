introspect::types::MemberDef {
    name: "{{name}}",
    attrs: [{{attrs_str}}].span(),
    type_def: introspect::Introspect::<{{member_type}}>::type_def(),
}