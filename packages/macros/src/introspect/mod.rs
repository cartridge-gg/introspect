use indent::indent_by;

use crate::introspect::utils::{make_params, merge_defs};

pub mod attribute;
pub mod derive;
pub mod enums;
pub mod structs;
pub mod utils;

const INTROSPECT_IMPL_TPL: &str = include_str!("../../templates/introspect_impl.cairo");

pub trait ToTypeDef {
    fn to_type_def(&self) -> String;
}

pub trait ItemTrait {
    const ITEM: &'static str;
    fn name(&self) -> &str;
    fn generic_params(&self) -> &Option<Vec<String>>;
    fn child_defs(&self) -> Vec<String>;
}

pub trait IntrospectImpl {
    fn to_impl(&self) -> String;
}

pub fn type_child_defs(ty: &str) -> String {
    format!("introspect::Introspect::<{}>::child_defs()", ty)
}

impl<T> IntrospectImpl for T
where
    T: ToTypeDef + ItemTrait,
{
    fn to_impl(&self) -> String {
        INTROSPECT_IMPL_TPL
            .replace("{{item}}", Self::ITEM)
            .replace("{{name}}", self.name())
            .replace(
                "{{impl_params}}",
                make_params(self.generic_params(), &["introspect::Introspect"]).as_str(),
            )
            .replace(
                "{{params}}",
                make_params(self.generic_params(), &[]).as_str(),
            )
            .replace("{{type_def}}", indent_by(12, self.to_type_def()).as_str())
            .replace(
                "{{child_defs}}",
                indent_by(8, merge_defs(self.child_defs())).as_str(),
            )
    }
}
