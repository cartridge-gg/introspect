use cairo_lang_macro::{ProcMacroResult, TextSpan, Token, TokenStream, TokenTree, derive_macro};
use cairo_lang_parser::printer::print_tree;
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_starknet_classes::keccak::starknet_keccak;
use cairo_lang_syntax::node::ast::{Modifier as AstModifier, Visibility as AstVisibility};
use starknet_types_core::felt::Felt;

use crate::AsCairo;

pub fn str_to_token_stream(s: &str) -> TokenStream {
    TokenStream::new(vec![TokenTree::Ident(Token::new(s, TextSpan::call_site()))])
}

#[derive(Copy, Clone, Debug, PartialEq)]

pub enum AttributeCallType {
    Derive,
    Attribute,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Visibility {
    Default,
    Pub,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Modifier {
    Ref,
    Mut,
}

impl AsCairo for Visibility {
    fn as_cairo(&self) -> String {
        match self {
            Visibility::Default => "".to_string(),
            Visibility::Pub => "pub ".to_string(),
        }
    }
}

impl AsCairo for Modifier {
    fn as_cairo(&self) -> String {
        match self {
            Modifier::Ref => "ref".to_string(),
            Modifier::Mut => "mut ".to_string(),
        }
    }
}

impl AsCairo for Vec<Modifier> {
    fn as_cairo(&self) -> String {
        self.into_iter().map(|m| m.as_cairo_suffixed(" ")).collect()
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

impl<'db> From<AstModifier<'db>> for Modifier {
    fn from(modifier: AstModifier<'db>) -> Self {
        match modifier {
            AstModifier::Ref(_) => Modifier::Ref,
            AstModifier::Mut(_) => Modifier::Mut,
        }
    }
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
