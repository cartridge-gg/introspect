use cairo_lang_macro::{ProcMacroResult, TokenStream, attribute_macro};
use cairo_lang_parser::printer::print_tree;
use cairo_lang_parser::utils::SimpleParserDatabase;

use crate::CairoCollectionFormat;
use crate::syntax::items_from_token_stream;
use crate::utils::str_to_token_stream;

#[attribute_macro]
pub fn parse(_attr: TokenStream, code: TokenStream) -> ProcMacroResult {
    // let db = SimpleParserDatabase::default();
    // let (node, _diagnostics) = db.parse_virtual_with_diagnostics(code);
    let items = items_from_token_stream(code);
    // println!("{}", print_tree(&db, &node, true, false));
    let string = items.to_cairo_block();
    println!("{}", string);
    ProcMacroResult::new(str_to_token_stream(&string))
}
