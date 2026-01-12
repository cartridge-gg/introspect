use crate::i_type::{IntrospectItemTrait, ToTypeDef};
use crate::{I_PATH, ItemTrait};
// pub mod attribute;
pub mod derive;
pub mod item;

const INTROSPECT_IMPL_TPL: &str = include_str!("../../templates/introspect_impl.cairo");
const INTROSPECT_REF_IMPL_TPL: &str = include_str!("../../templates/introspect_ref_impl.cairo");
pub trait IntrospectImpl {
    fn to_introspect_impl(&self) -> String;
    fn to_introspect_ref_impl(&self) -> String;
}

impl<T> IntrospectImpl for T
where
    T: ToTypeDef + IntrospectItemTrait + ItemTrait,
{
    fn to_introspect_impl(&self) -> String {
        INTROSPECT_IMPL_TPL
            .replace("{{i_path}}", I_PATH)
            .replace("{{kind}}", self.kind())
            .replace("{{name}}", self.name())
            .replace("{{full_name}}", &self.full_name())
            .replace(
                "{{impl_params}}",
                &self.generics_with_traits(&["introspect::Introspect"]),
            )
            .replace("{{type_def}}", &self.to_type_def())
            .replace("{{child_defs}}", &self.child_defs())
    }

    fn to_introspect_ref_impl(&self) -> String {
        INTROSPECT_REF_IMPL_TPL
            .replace("{{i_path}}", I_PATH)
            .replace("{{kind}}", self.kind())
            .replace("{{name}}", self.name())
            .replace("{{full_name}}", &self.full_name())
            .replace("{{call_generics}}", &self.generics_call())
            .replace(
                "{{impl_params}}",
                &self.generics_with_traits(&["introspect::Introspect"]),
            )
            .replace("{{type_def}}", &self.to_type_def())
            .replace("{{child_defs}}", &self.child_defs())
    }
}
