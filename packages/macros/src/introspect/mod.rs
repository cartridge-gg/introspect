use crate::i_type::IntrospectItemTrait;
use crate::{CairoElementDef, ItemTrait};
// pub mod attribute;
pub mod derive;
pub mod item;

const INTROSPECT_IMPL_TPL: &str = include_str!("../../templates/introspect_impl.cairo");
const INTROSPECT_REF_IMPL_TPL: &str = include_str!("../../templates/introspect_ref_impl.cairo");
pub trait IntrospectImpl {
    fn to_introspect_impl<const IS_REF: bool>(&self, i_path: &str) -> String;
}

impl<T> IntrospectImpl for T
where
    T: CairoElementDef + IntrospectItemTrait + ItemTrait,
{
    fn to_introspect_impl<const IS_REF: bool>(&self, i_path: &str) -> String {
        // let introspect_path = i_path.with_simple_segment("Introspect");
        // let type_def_path = i_path.with_simple_segment("TypeDef");
        // let type_def_path = type_def_path.with_simple_segment(self.kind());
        // let mut impl_ = Impl {
        //     attributes: vec![],
        //     visibility: self.visibility(),
        //     name: format!("__{}IntrospectImpl", self.name()),
        //     generic_params: None,
        //     trait_path: introspect_path,
        //     body: Some(vec![ImplItem::Function(FunctionWithBody {
        //         attributes: vec![],
        //         visibility: Visibility::Default,
        //         declaration: crate::syntax::module::FunctionDeclaration {
        //             is_const: true,
        //             name: "type_def".to_string(),
        //             generic_params: None,
        //             signature: crate::syntax::module::FunctionSignature {
        //                 parameters: vec![],
        //                 return_type: Some(Expr::Path(type_def_path)),
        //                 implicits_clause: None,
        //                 no_panic: false,
        //             },
        //         },
        //         body: vec![],
        //     })]),
        // };

        let tpl = match IS_REF {
            true => INTROSPECT_REF_IMPL_TPL,
            false => INTROSPECT_IMPL_TPL,
        };
        tpl.replace("{{i_path}}", i_path)
            .replace("{{kind}}", self.kind())
            .replace("{{name}}", self.name())
            .replace("{{full_name}}", &self.full_name())
            .replace(
                "{{impl_params}}",
                &self.generics_with_traits(&["introspect::Introspect"]),
            )
            .replace("{{type_def}}", &self.as_element_def(i_path))
            .replace("{{collect_child_defs}}", &self.child_defs(i_path))
    }
}
