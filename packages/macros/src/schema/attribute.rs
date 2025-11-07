use cairo_lang_macro::{ProcMacroResult, TokenStream, attribute_macro};
use cairo_lang_parser::printer::print_tree;
use cairo_lang_parser::utils::SimpleParserDatabase;

use crate::schema::schema::ToSchemaImpl;
use crate::structs::get_struct;
use crate::utils::str_to_token_stream;

#[attribute_macro]
pub fn schema(attr: TokenStream, code: TokenStream) -> ProcMacroResult {
    println!("Schema attribute macro called.");
    let db = SimpleParserDatabase::default();
    let (code, _diag) = db.parse_virtual_with_diagnostics(code);
    let (_attr, _diag) = db.parse_virtual_with_diagnostics(attr);
    println!("{}", print_tree(&db, &code, true, true));
    let mut item = get_struct(&db, code).unwrap();
    let impl_string = item.to_schema_impl();
    let item_string = item.to_string();
    let string = format!("{}\n\n{}", item_string, impl_string);
    println!("\n{}\n", string);
    ProcMacroResult::new(str_to_token_stream(&string))
}
