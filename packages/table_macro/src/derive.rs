use cairo_lang_macro::{ProcMacroResult, TokenStream};
use introspect_macros::utils::str_to_token_stream;
use introspect_macros::{Struct, SyntaxItemTrait};

use crate::TableInterface;

#[allow(non_snake_case)]
#[derive_macro]
fn Table(token_stream: TokenStream) -> ProcMacroResult {
    let stuct_item = Struct::from_token_stream(token_stream).unwrap();
    let table = TableInterface::from_struct(&stuct_item);

    println!("{}", code);
    ProcMacroResult::new(str_to_token_stream(&string))
}
