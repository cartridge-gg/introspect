use cairo_lang_macro::{ProcMacroResult, TokenStream, inline_macro};
use cairo_lang_parser::printer::print_tree;
use cairo_lang_parser::utils::SimpleParserDatabase;

use crate::utils::str_to_token_stream;

#[inline_macro]
fn emit_schemas(token_stream: TokenStream) -> ProcMacroResult {
    let db = SimpleParserDatabase::default();
    let (parsed, _diag) = db.parse_virtual_with_diagnostics(token_stream.clone());
    println!("{}", print_tree(&db, &parsed, true, true));
    let text = parsed
        .descendants(&db)
        .next()
        .unwrap()
        .get_text_without_all_comment_trivia(&db);
    let args = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .map(|s| s.trim().to_string());

    println!("{:?}", args.collect::<Vec<String>>());
    ProcMacroResult::new(str_to_token_stream(""))
}
