mod {{name}}_introspect_ref_inner{
    pub impl Inner{{impl_params}} of introspect::Introspect<super::{{full_name}}> {
        fn type_def() -> introspect::types::TypeDef {
            introspect::types::TypeDef::{{kind}}({{type_def}})
        }

        fn child_defs() -> Array<(felt252, introspect::types::TypeDef)> {
            {{child_defs}}
        }
    }
}

pub impl Gen{{name}}IntrospectImpl{{impl_params}} of introspect::Introspect<{{full_name}}> {
    fn type_def() -> introspect::types::TypeDef {
        introspect::types::TypeDef::Ref({{name}}_introspect_ref_inner::Inner{{call_generics}}::hash())
    }

    fn child_defs() -> Array<(felt252, introspect::types::TypeDef)> {
        let mut child_defs = {{name}}_introspect_ref_inner::Inner{{call_generics}}::child_defs();
        child_defs.append((
            {{name}}_introspect_ref_inner::Inner{{call_generics}}::hash(),
            {{name}}_introspect_ref_inner::Inner{{call_generics}}::type_def()
        ));
        child_defs
    }
}