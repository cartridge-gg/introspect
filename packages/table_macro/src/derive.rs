use cairo_lang_macro::{ProcMacroResult, TokenStream};
use introspect_macros::utils::str_to_token_stream;
use introspect_macros::{Struct, SyntaxItemTrait};

use crate::Table;

#[allow(non_snake_case)]
#[derive_macro]
fn Table(token_stream: TokenStream) -> ProcMacroResult {
    let stuct_item = Struct::from_token_stream(token_stream).unwrap();
    let table = Table::from_struct(&stuct_item);
    let code = table.generate_meta_impl()
        + &table.generate_primary_impl()
        + &table.generate_columns_impl()
        + &table.generate_required_impls();
    println!("{}", code);
    ProcMacroResult::new(str_to_token_stream(&string))
}
