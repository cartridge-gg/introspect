use cairo_lang_macro::{ProcMacroResult, TextSpan, Token, TokenStream, TokenTree, derive_macro};
use cairo_lang_parser::printer::print_tree;
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_starknet_classes::keccak::starknet_keccak;
use cairo_lang_syntax::node::ast::Visibility as AstVisibility;
use indent::indent_all_by;
use starknet_types_core::felt::Felt;

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

pub fn string_to_keccak_hex(s: &str) -> String {
    format!("0x{}", starknet_keccak(s.as_bytes()).to_str_radix(16))
}

pub fn string_to_keccak_felt(s: &str) -> Felt {
    starknet_keccak(s.as_bytes()).into()
}

pub fn spanify(elements: Vec<String>) -> String {
    match elements.len() {
        0 => "[].span()".to_string(),
        1 => format!("[{}].span()", elements[0]),
        _ => format!("[\n{}\n].span()", indent_all_by(4, elements.join(",\n"))),
    }
}

pub fn get_inner_type(type_name: &str) -> String {
    let start = type_name.find('<').unwrap();
    type_name[start + 1..type_name.len() - 1].to_string()
}

pub fn get_fixed_array_inner_type(type_name: &str) -> &str {
    type_name[1..type_name.len() - 1]
        .rsplitn(2, ';')
        .last()
        .unwrap()
        .trim()
}

pub fn get_tuple_inner_types(type_name: &str) -> Vec<String> {
    let inner = &type_name[1..type_name.len() - 1];
    let types: Vec<String> = inner
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    types
}

pub fn is_of_base_types(type_name: &str) -> bool {
    if type_name.ends_with(">")
        && (["Span<", "Array<", "Option<"]
            .iter()
            .any(|g| type_name.starts_with(g)))
    {
        is_of_base_types(&get_inner_type(&type_name))
    } else if type_name.starts_with("[") && type_name.ends_with("]") {
        is_of_base_types(get_fixed_array_inner_type(type_name))
    } else if type_name.starts_with("(") && type_name.ends_with(")") {
        get_tuple_inner_types(type_name)
            .iter()
            .all(|e| is_of_base_types(e))
    } else {
        is_base_type(type_name)
    }
}

pub fn is_base_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "felt252"
            | "bool"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "u256"
            | "u512"
            | "core::integer::u512"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "bytes31"
            | "ClassHash"
            | "starknet::ClassHash"
            | "ContractAddress"
            | "starknet::ContractAddress"
            | "EthAddress"
            | "starknet::EthAddress"
            | "StorageAddress"
            | "starknet::StorageAddress"
            | "StorageBaseAddress"
            | "starknet::storage_access::StorageBaseAddress"
            | "ByteArray"
    )
}
