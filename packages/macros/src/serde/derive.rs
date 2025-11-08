use cairo_lang_macro::{ProcMacroResult, TokenStream, derive_macro};
use cairo_lang_parser::utils::SimpleParserDatabase;

use crate::introspect::item::get_introspection_type;
use crate::serde::ToISerdeImpl;
use crate::utils::str_to_token_stream;

#[allow(non_snake_case)]
#[derive_macro]
fn ISerde(token_stream: TokenStream) -> ProcMacroResult {
    let db = SimpleParserDatabase::default();
    let (parsed, _diag) = db.parse_virtual_with_diagnostics(token_stream.clone());
    let item = get_introspection_type(&db, parsed).unwrap();
    let string = item.to_iserde_impl();
    println!("{}", string);
    ProcMacroResult::new(str_to_token_stream(&string))
}
