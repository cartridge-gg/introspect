use super::IntrospectImpl;
use crate::i_type::extraction::IExtractFromTokenStream;
use crate::utils::str_to_token_stream;
use crate::{I_PATH, IItem};
use cairo_lang_macro::{ProcMacroResult, TokenStream, derive_macro};

#[allow(non_snake_case)]
#[derive_macro]
fn Introspect(token_stream: TokenStream) -> ProcMacroResult {
    let item = IItem::iextract_from_token_stream(token_stream).unwrap();
    let string = item.to_introspect_impl::<false>(I_PATH);
    ProcMacroResult::new(str_to_token_stream(&string))
}

#[allow(non_snake_case)]
#[derive_macro]
fn IntrospectRef(token_stream: TokenStream) -> ProcMacroResult {
    let item = IItem::iextract_from_token_stream(token_stream).unwrap();
    let string = item.to_introspect_impl::<true>(I_PATH);
    ProcMacroResult::new(str_to_token_stream(&string))
}
