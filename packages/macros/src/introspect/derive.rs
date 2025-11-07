use crate::introspect::IntrospectImpl;
use crate::introspect::item::get_introspection_type;
use crate::utils::str_to_token_stream;
use cairo_lang_macro::{ProcMacroResult, TokenStream, derive_macro};
use cairo_lang_parser::utils::SimpleParserDatabase;

#[allow(non_snake_case)]
#[derive_macro]
fn Introspect(token_stream: TokenStream) -> ProcMacroResult {
    let db = SimpleParserDatabase::default();
    let (parsed, _diag) = db.parse_virtual_with_diagnostics(token_stream.clone());
    let mut item = get_introspection_type(&db, parsed).unwrap();
    let string = item.to_introspect_impl();
    println!("{}", string);
    ProcMacroResult::new(str_to_token_stream(&string))
}

#[allow(non_snake_case)]
#[derive_macro]
fn IntrospectRef(token_stream: TokenStream) -> ProcMacroResult {
    let db = SimpleParserDatabase::default();
    let (parsed, _diag) = db.parse_virtual_with_diagnostics(token_stream.clone());
    let mut item = get_introspection_type(&db, parsed).unwrap();
    let string = item.to_introspect_ref_impl();
    println!("{}", string);
    ProcMacroResult::new(str_to_token_stream(&string))
}
