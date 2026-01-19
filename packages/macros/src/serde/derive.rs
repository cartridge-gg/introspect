use crate::IItem;
use crate::i_type::extraction::IExtractFromTokenStream;
use crate::serde::ToISerdeImpl;
use crate::utils::str_to_token_stream;
use cairo_lang_macro::{ProcMacroResult, TokenStream, derive_macro};

#[allow(non_snake_case)]
#[derive_macro]
fn ISerde(token_stream: TokenStream) -> ProcMacroResult {
    let item = IItem::iextract_from_token_stream(token_stream).unwrap();
    let string = item.to_iserde_impl();
    ProcMacroResult::new(str_to_token_stream(&string))
}
