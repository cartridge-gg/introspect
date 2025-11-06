use cairo_lang_macro::{ProcMacroResult, TextSpan, Token, TokenStream, TokenTree, derive_macro};
use cairo_lang_parser::printer::print_tree;
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::ast::Visibility as AstVisibility;

pub fn str_to_token_stream(s: &str) -> TokenStream {
    TokenStream::new(vec![TokenTree::Ident(Token::new(s, TextSpan::call_site()))])
}

#[derive(Clone, Debug)]
pub enum Visibility {
    Default,
    Pub,
}

#[derive(Clone, Debug)]
pub enum Modifier {
    Ref,
    Mut,
}

impl Visibility {
    pub fn to_code_string(&self) -> String {
        match self {
            Visibility::Default => "".to_string(),
            Visibility::Pub => "pub ".to_string(),
        }
    }
}

impl<'db> From<AstVisibility<'db>> for Visibility {
    fn from(visibility: AstVisibility<'db>) -> Self {
        match visibility {
            AstVisibility::Default(_) => Visibility::Default,
            AstVisibility::Pub(_) => Visibility::Pub,
        }
    }
}

#[derive_macro]
pub fn print_all(token_stream: TokenStream) -> ProcMacroResult {
    let db = SimpleParserDatabase::default();
    let (parsed, _diag) = db.parse_virtual_with_diagnostics(token_stream.clone());
    println!("{}", print_tree(&db, &parsed, true, true));
    ProcMacroResult::new(str_to_token_stream("mod something {}"))
}
