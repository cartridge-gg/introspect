pub mod enums;
pub mod structs;

use crate::utils::str_to_token_stream;
use crate::{Item, ItemTrait, SyntaxItemTrait};
use cairo_lang_macro::{ProcMacroResult, TokenStream, derive_macro};

const FUZZABLE_IMPL_TEMPLATE: &str = include_str!("../../templates/fuzzable_impl.cairo");

#[allow(non_snake_case)]
#[derive_macro]
fn Fuzzable(token_stream: TokenStream) -> ProcMacroResult {
    let item = Item::from_token_stream(token_stream).unwrap();
    let fuzzable_string = item.as_fuzzable_impl();
    let string = format!("{}", fuzzable_string);
    println!("{}", string);
    ProcMacroResult::new(str_to_token_stream(&string))
}

trait FuzzableImpl: ItemTrait {
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
        }
    }
}
