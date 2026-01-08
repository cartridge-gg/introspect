use indent::indent_by;

use crate::items::{IntrospectItemTrait, ToTypeDef};

pub mod attribute;
pub mod derive;
pub mod item;

const INTROSPECT_IMPL_TPL: &str = include_str!("../../templates/introspect_impl.cairo");
const INTROSPECT_REF_IMPL_TPL: &str = include_str!("../../templates/introspect_ref_impl.cairo");
pub trait IntrospectImpl {
    fn to_introspect_impl(&mut self) -> String;
    fn to_introspect_ref_impl(&mut self) -> String;
}

impl<T> IntrospectImpl for T
where
    T: ToTypeDef + IntrospectItemTrait,
{
    fn to_introspect_impl(&mut self) -> String {
        INTROSPECT_IMPL_TPL
            .replace("{{kind}}", self.kind())
            .replace("{{name}}", self.name())
            .replace("{{full_name}}", &self.full_name())
            .replace(
                "{{impl_params}}",
                &self.generics_with_traits(&["introspect::Introspect"]),
            )
            .replace("{{type_def}}", &indent_by(8, pad_nl(&self.to_type_def())))
            .replace(
                "{{child_defs}}",
                &indent_by(8, merge_defs(self.child_defs())),
            )
    }

    fn to_introspect_ref_impl(&mut self) -> String {
        INTROSPECT_REF_IMPL_TPL
            .replace("{{kind}}", self.kind())
            .replace("{{name}}", self.name())
            .replace("{{full_name}}", &self.full_name())
            .replace("{{call_generics}}", &self.generics_call())
            .replace(
                "{{impl_params}}",
                &self.generics_with_traits(&["introspect::Introspect"]),
            )
            .replace("{{type_def}}", &indent_by(8, pad_nl(&self.to_type_def())))
            .replace(
                "{{child_defs}}",
                &indent_by(8, merge_defs(self.child_defs())),
            )
    }
}
