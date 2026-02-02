pub mod enums;
pub mod structs;
pub mod utils;

use crate::Ty;
use crate::i_type::IntrospectItemTrait;
use cairo_lang_macro::{ProcMacroResult, TokenStream, derive_macro};
use cairo_syntax_parser::{Item, str_to_token_stream};
use itertools::Itertools;

const FUZZABLE_IMPL_TEMPLATE: &str = include_str!("../../templates/fuzzable_impl.cairo");
const FUZZABLE_GENERATE_CALL: &str = "snforge_std::fuzzable::Fuzzable::generate()";

#[allow(non_snake_case)]
#[derive_macro]
fn Fuzzable(token_stream: TokenStream) -> ProcMacroResult {
    let item = Item::from_token_stream(token_stream).unwrap();
    let fuzzable_string = item.as_fuzzable_impl();
    let string = format!("{}", fuzzable_string);
    ProcMacroResult::new(str_to_token_stream(&string))
}

trait FuzzableImpl: IntrospectItemTrait {
    fn as_fuzzable_impl(&self) -> String {
        FUZZABLE_IMPL_TEMPLATE
            .replace("{{name}}", self.name())
            .replace("{{full_name}}", &self.full_name())
            .replace(
                "{{impl_params}}",
                &self
                    .generics_with_traits(&["core::fmt::Debug", "snforge_std::fuzzable::Fuzzable"]),
            )
            .replace("{{body}}", &self.fuzzable_body())
    }
    fn fuzzable_body(&self) -> String;
}

impl FuzzableImpl for Item {
    fn fuzzable_body(&self) -> String {
        match self {
            Item::Struct(s) => s.fuzzable_body(),
            Item::Enum(e) => e.fuzzable_body(),
            _ => panic!("Fuzzable can only be derived for structs and enums"),
        }
    }
}

impl Ty {
    pub fn generate_fuzzable(&self) -> String {
        match self {
            Ty::Item(_) => FUZZABLE_GENERATE_CALL.to_string(),
            Ty::Tuple(tup) => tup.iter().map(|_| FUZZABLE_GENERATE_CALL).join(", "),
            Ty::FixedArray(_) => {
                panic!("Fixed arrays are not supported yet in fuzzable generation");
            }
        }
    }
}
