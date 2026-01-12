mod {{name}}_introspect_ref_inner{
    pub impl Inner{{impl_params}} of {{i_path}}::Introspect<super::{{full_name}}> {
        fn type_def() -> {{i_path}}::TypeDef {
            {{i_path}}::TypeDef::{{kind}}({{type_def}})
        }

        fn child_defs() -> Array<(felt252, {{i_path}}::TypeDef)> {
            {{child_defs}}
        }
    }
}

pub impl Gen{{name}}IntrospectImpl{{impl_params}} of {{i_path}}::Introspect<{{full_name}}> {
    fn type_def() -> {{i_path}}::TypeDef {
        {{i_path}}::TypeDef::Ref({{name}}_introspect_ref_inner::Inner{{call_generics}}::hash())
    }

    fn child_defs() -> Array<(felt252, {{i_path}}::TypeDef)> {
        let mut child_defs = {{name}}_introspect_ref_inner::Inner{{call_generics}}::child_defs();
        child_defs.append((
            {{name}}_introspect_ref_inner::Inner{{call_generics}}::hash(),
            {{name}}_introspect_ref_inner::Inner{{call_generics}}::type_def()
        ));
        child_defs
    }
}