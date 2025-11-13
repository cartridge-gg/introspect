use crate::schema::schema::ToSchemaImpl;
use crate::serde::ToISerdeImpl;
use crate::structs::get_struct;
use crate::utils::str_to_token_stream;
use cairo_lang_macro::{ProcMacroResult, TokenStream, derive_macro};
use cairo_lang_parser::utils::SimpleParserDatabase;

#[allow(non_snake_case)]
#[derive_macro]
fn Schema(token_stream: TokenStream) -> ProcMacroResult {
    let db = SimpleParserDatabase::default();
    let (parsed, _diag) = db.parse_virtual_with_diagnostics(token_stream.clone());
    let mut item = get_struct(&db, parsed).unwrap();
    let schema_string = item.to_schema_impl();
    let iserde_string = item.to_iserde_impl();
    let string = format!("{}\n\n{}", schema_string, iserde_string);
    println!("{}", string);
    ProcMacroResult::new(str_to_token_stream(&string))
}
