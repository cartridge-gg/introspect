use cairo_lang_macro::{ProcMacroResult, TokenStream, derive_macro};
use introspect_macros::i_type::IExtract;
use introspect_macros::utils::str_to_token_stream;
use introspect_macros::{Struct, SyntaxItemTrait};

use crate::I_TABLE_PATH;
use crate::interface::TableInterface;
use crate::set::ColumnSet;
use crate::structure::TableStructure;

#[allow(non_snake_case)]
#[derive_macro]
fn Table(token_stream: TokenStream) -> ProcMacroResult {
    let mut struct_item = Struct::from_token_stream(token_stream).unwrap();
    let table_struct = TableStructure::iextract(&mut struct_item).unwrap();
    let table_interface = TableInterface::iextract(&mut struct_item).unwrap();
    let string = table_struct.get_structure_impl(I_TABLE_PATH, I_TABLE_PATH)
        + &table_interface.table_impl(I_TABLE_PATH, I_TABLE_PATH, &table_struct.impl_name);
    ProcMacroResult::new(str_to_token_stream(&string))
}

#[allow(non_snake_case)]
#[derive_macro]
fn ColumnSet(token_stream: TokenStream) -> ProcMacroResult {
    let mut struct_item = Struct::from_token_stream(token_stream).unwrap();
    let column_set = ColumnSet::iextract(&mut struct_item).unwrap();
    let string = column_set.column_set_impl(I_TABLE_PATH);
    ProcMacroResult::new(str_to_token_stream(&string))
}
