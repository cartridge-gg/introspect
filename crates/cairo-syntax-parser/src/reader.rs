use crate::utils::str_to_token_stream;
use crate::{CairoCollectionFormat, CairoFormat, Item, items_from_token_stream};
use cairo_lang_macro::{ProcMacroResult, TokenStream, attribute_macro, derive_macro};

#[attribute_macro]
pub fn parse(_attr: TokenStream, code: TokenStream) -> ProcMacroResult {
    // let db = SimpleParserDatabase::default();
    // let (node, _diagnostics) = db.parse_virtual_with_diagnostics(code);
    let items = items_from_token_stream(code);
    let string: String = items.to_cairo_block();
    let stream = str_to_token_stream(&string);
    println!("PARSE MACRO RAN");
    println!("{string}");
    ProcMacroResult::new(stream)
}
