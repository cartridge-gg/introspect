use crate::introspect::IntrospectImpl;
use crate::utils::str_to_token_stream;
use cairo_lang_macro::{ProcMacroResult, TokenStream, attribute_macro};
use cairo_lang_parser::utils::SimpleParserDatabase;

#[attribute_macro]
pub fn introspect(attr: TokenStream, code: TokenStream) -> ProcMacroResult {
    let db = SimpleParserDatabase::default();
    let (code, _diag) = db.parse_virtual_with_diagnostics(code);
    let (_attr, _diag) = db.parse_virtual_with_diagnostics(attr);
    let mut item = get_introspection_type(&db, code).unwrap();
    let impl_string = item.to_introspect_impl();
    let item_string = item.to_string();
    let string = format!("{}\n\n{}", item_string, impl_string);
    ProcMacroResult::new(str_to_token_stream(&string))
}
