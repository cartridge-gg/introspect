use cairo_lang_syntax::node::{SyntaxNode, ast::{ItemEnum, ItemStruct}};
use crate::{Result, IntrospectError};
use cairo_lang_macro::{ProcMacroResult, TokenStream, derive_macro};
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_parser::printer::print_tree;
use salsa::Database;
pub enum IntrospectAbleType<'db> {
    Struct(ItemStruct<'db>),
    Enum(ItemEnum<'db>),
}

// fn get_introspect_type<'db>(db: &'db dyn Database, file: SyntaxNode<'db>) -> Result<IntrospectAbleType<'db>> {
//     let child = file.get_children(db)[0].get_children(db)[0];
    

    
// }


#[derive_macro]
pub fn print_all(token_stream: TokenStream) -> ProcMacroResult {
    let db = SimpleParserDatabase::default();
    let (parsed, _diag) = db.parse_virtual_with_diagnostics(token_stream);
    let child = parsed.get_children(&db)[0].get_children(&db)[0];
    println!("{}", print_tree(&db, &child, true, false));

    ProcMacroResult::new(TokenStream::new(vec![]))
}