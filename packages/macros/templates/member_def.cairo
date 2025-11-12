introspect::types::MemberDef {
    name: "{{name}}",
    attributes: [{{attributes_str}}].span(),
    type_def: introspect::Introspect::<{{member_type}}>::type_def(),
}