use cairo_lang_macro::{ProcMacroResult, TextSpan, Token, TokenStream, TokenTree, derive_macro};
use cairo_lang_parser::printer::print_tree;
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_starknet_classes::keccak::starknet_keccak;
use starknet_types_core::felt::Felt;

pub fn str_to_token_stream(s: &str) -> TokenStream {
    TokenStream::new(vec![TokenTree::Ident(Token::new(s, TextSpan::call_site()))])
}

#[derive(Copy, Clone, Debug, PartialEq)]

pub enum AttributeCallType {
    Derive,
    Attribute,
}

#[allow(non_snake_case)]
#[derive_macro]
pub fn PrintAll(token_stream: TokenStream) -> ProcMacroResult {
    let db = SimpleParserDatabase::default();
    let (parsed, _diag) = db.parse_virtual_with_diagnostics(token_stream.clone());
    println!("{}", print_tree(&db, &parsed, true, true));
    ProcMacroResult::new(str_to_token_stream("mod something {}"))
}

pub fn string_to_keccak_hex(s: &str) -> String {
    format!("0x{}", starknet_keccak(s.as_bytes()).to_str_radix(16))
}

pub fn string_to_keccak_felt(s: &str) -> Felt {
    starknet_keccak(s.as_bytes()).into()
}

pub trait Quoted {
    fn quoted(&self) -> String;
}

impl Quoted for String {
    fn quoted(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl Quoted for &str {
    fn quoted(&self) -> String {
        format!("\"{}\"", self)
    }
}

pub fn create_single_token(content: impl AsRef<str>) -> TokenTree {
    TokenTree::Ident(Token::new(content, TextSpan::call_site()))
}
