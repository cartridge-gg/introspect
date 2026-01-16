use super::IntrospectImpl;
use crate::i_type::DefaultIExtractor;
use crate::i_type::extraction::IExtractFromTokenStream;
use crate::utils::str_to_token_stream;
use crate::{AttributeCallType, IItem};
use cairo_lang_macro::{ProcMacroResult, TokenStream, derive_macro};

#[allow(non_snake_case)]
#[derive_macro]
fn Introspect(token_stream: TokenStream) -> ProcMacroResult {
    let extractor = DefaultIExtractor::new(AttributeCallType::Derive);
    let item: IItem = extractor.iextract_from_token_stream(token_stream).unwrap();
    ProcMacroResult::new(str_to_token_stream(&item.to_introspect_impl()))
}

#[allow(non_snake_case)]
#[derive_macro]
fn IntrospectRef(token_stream: TokenStream) -> ProcMacroResult {
    let extractor = DefaultIExtractor::new(AttributeCallType::Derive);
    let item: IItem = extractor.iextract_from_token_stream(token_stream).unwrap();
    ProcMacroResult::new(str_to_token_stream(&item.to_introspect_ref_impl()))
}
