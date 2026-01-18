pub impl Gen{{name}}IntrospectRefImpl{{impl_params}} of {{i_path}}::IntrospectRef<{{full_name}}> {
    fn ref_type_def() -> {{i_path}}::TypeDef {
        {{i_path}}::TypeDef::{{kind}}({{type_def}})
    }
    fn collect_ref_child_defs(ref defs: {{i_path}}::ChildDefs) {
        {{collect_child_defs}}
    }
}