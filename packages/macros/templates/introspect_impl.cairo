pub impl {{name}}IntrospectImpl{{impl_params}} of introspect::Introspect<{{name}}{{params}}> {
    fn type_def() -> introspect::types::TypeDef {
        introspect::types::TypeDef::{{item}}({{type_def}})
    }

    fn child_defs() -> Array<(felt252, introspect::types::TypeDef)> {
        {{child_defs}}
    }
}