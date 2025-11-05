use cairo_lang_macro::derive_macro;
use cairo_lang_macro::{ProcMacroResult, TokenStream};
use cairo_lang_parser::utils::SimpleParserDatabase;



// #[derive_macro]
// pub fn introspect_macro(input: TokenStream) -> ProcMacroResult {
//     let db = SimpleParserDatabase::default();
//     let (parsed, _diag) = db.parse_virtual_with_diagnostics(input);
//     let mut nodes = parsed.descendants(&db);
// }
