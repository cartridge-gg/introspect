use super::CWriteTypeDef;
use crate::{I_PATH, IntrospectItem};
use cairo_lang_macro::{ProcMacroResult, TokenStream, derive_macro};
use cairo_syntax_parser::str_to_token_stream;

#[allow(non_snake_case)]
#[derive_macro]
fn TypeDef(token_stream: TokenStream) -> ProcMacroResult {
    let item = IntrospectItem::from_token_stream(token_stream).unwrap();
    let string = item.type_def_impl_to_string(I_PATH, false);
    println!("{string}");
    ProcMacroResult::new(str_to_token_stream(&string))
}

#[allow(non_snake_case)]
#[derive_macro]
fn TypeDefRef(token_stream: TokenStream) -> ProcMacroResult {
    let item = IntrospectItem::from_token_stream(token_stream).unwrap();
    let string = item.type_def_impl_to_string(I_PATH, true);
    println!("{string}");
    ProcMacroResult::new(str_to_token_stream(&string))
}
