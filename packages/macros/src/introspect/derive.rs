use super::IntrospectImpl;
use crate::i_type::extraction::IExtractFromTokenStream;
use crate::i_type::{DefaultIExtractor, IExtract};
use crate::serde::ToISerdeImpl;
use crate::utils::str_to_token_stream;
use crate::{AttributeCallType, IItem, Item, SyntaxItemTrait};
use cairo_lang_macro::{ProcMacroResult, TokenStream, derive_macro};

#[allow(non_snake_case)]
#[derive_macro]
fn Introspect(token_stream: TokenStream) -> ProcMacroResult {
    let extractor = DefaultIExtractor::new(AttributeCallType::Derive);
    let mut item = Item::from_token_stream(token_stream).unwrap();
    let introspect_item = extractor.iextract(&mut item).unwrap();
    let introspect_string = item.to_introspect_impl();
    let iserde_string = item.to_iserde_impl();
    let string = format!("{}\n\n{}", introspect_string, iserde_string);
    ProcMacroResult::new(str_to_token_stream(&string))
}

#[allow(non_snake_case)]
#[derive_macro]
fn IntrospectRef(token_stream: TokenStream) -> ProcMacroResult {
    let extractor = DefaultIExtractor::new(AttributeCallType::Derive);
    let item: IItem = extractor.iextract_from_token_stream(token_stream).unwrap();
    let introspect_string = item.to_introspect_ref_impl();
    let iserde_string = item.to_iserde_impl();
    let string = format!("{}\n\n{}", introspect_string, iserde_string);
    ProcMacroResult::new(str_to_token_stream(&string))
}
