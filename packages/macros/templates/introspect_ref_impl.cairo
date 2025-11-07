mod {{name}}_introspect_ref_inner{
    pub impl Inner{{impl_params}} of introspect::Introspect<super::{{name}}{{params}}> {
        fn type_def() -> introspect::types::TypeDef {
            introspect::types::TypeDef::{{item}}({{type_def}})
        }

        fn child_defs() -> Array<(felt252, introspect::types::TypeDef)> {
            {{child_defs}}
        }
    }
}

pub impl {{name}}IntrospectImpl{{impl_params}} of introspect::Introspect<{{name}}{{params}}> {
    fn type_def() -> introspect::types::TypeDef {
        introspect::types::TypeDef::Ref({{name}}_introspect_ref_inner::Inner{{call_params}}::hash())
    }

    fn child_defs() -> Array<(felt252, introspect::types::TypeDef)> {
        let mut child_defs = {{name}}_introspect_ref_inner::Inner{{call_params}}::child_defs();
        child_defs.append((
            {{name}}_introspect_ref_inner::Inner{{call_params}}::hash(),
            {{name}}_introspect_ref_inner::Inner{{call_params}}::type_def()
        ));
        child_defs
    }
}