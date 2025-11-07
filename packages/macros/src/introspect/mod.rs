use indent::indent_by;

use crate::params::make_params;
use crate::type_def::{ItemTrait, ToTypeDef, merge_defs, pad_nl};

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
    T: ToTypeDef + ItemTrait,
{
    fn to_introspect_impl(&mut self) -> String {
        INTROSPECT_IMPL_TPL
            .replace("{{item}}", Self::ITEM)
            .replace("{{name}}", self.name())
            .replace(
                "{{params}}",
                make_params(self.generic_params(), &[], false).as_str(),
            )
            .replace(
                "{{impl_params}}",
                make_params(self.generic_params(), &["introspect::Introspect"], false).as_str(),
            )
            .replace(
                "{{type_def}}",
                indent_by(8, pad_nl(&self.to_type_def())).as_str(),
            )
            .replace(
                "{{child_defs}}",
                indent_by(8, merge_defs(self.child_defs())).as_str(),
            )
    }

    fn to_introspect_ref_impl(&mut self) -> String {
        INTROSPECT_REF_IMPL_TPL
            .replace("{{item}}", Self::ITEM)
            .replace("{{name}}", self.name())
            .replace(
                "{{call_params}}",
                make_params(self.generic_params(), &[], true).as_str(),
            )
            .replace(
                "{{params}}",
                make_params(self.generic_params(), &[], false).as_str(),
            )
            .replace(
                "{{impl_params}}",
                make_params(self.generic_params(), &["introspect::Introspect"], false).as_str(),
            )
            .replace(
                "{{type_def}}",
                indent_by(8, pad_nl(&self.to_type_def())).as_str(),
            )
            .replace(
                "{{child_defs}}",
                indent_by(8, merge_defs(self.child_defs())).as_str(),
            )
    }
}
