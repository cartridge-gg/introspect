pub impl Gen{{name}}IntrospectImpl{{impl_params}} of {{i_path}}::Introspect<{{full_name}}> {
    fn type_def() -> {{i_path}}::TypeDef {
        {{i_path}}::TypeDef::{{kind}}({{type_def}})
    }

    fn child_defs() -> Array<(felt252, {{i_path}}::TypeDef)> {
        {{child_defs}}
    }
}