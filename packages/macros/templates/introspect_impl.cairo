pub impl Gen{{name}}IntrospectImpl{{impl_params}} of introspect::Introspect<{{full_name}}> {
    fn type_def() -> introspect::types::TypeDef {
        introspect::types::TypeDef::{{kind}}({{type_def}})
    }

    fn child_defs() -> Array<(felt252, introspect::types::TypeDef)> {
        {{child_defs}}
    }
}