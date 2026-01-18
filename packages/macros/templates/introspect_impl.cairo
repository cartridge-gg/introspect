pub impl Gen{{name}}IntrospectImpl{{impl_params}} of {{i_path}}::Introspect<{{full_name}}> {
    fn type_def() -> {{i_path}}::TypeDef {
        {{i_path}}::TypeDef::{{kind}}({{type_def}})
    }
    fn collect_child_defs(ref defs: {{i_path}}::ChildDefs) {
        {{collect_child_defs}}
    }
}