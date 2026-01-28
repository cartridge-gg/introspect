use cairo_lang_macro::{ProcMacroResult, TokenStream, attribute_macro};
use cairo_lang_parser::printer::print_tree;
use cairo_lang_parser::utils::SimpleParserDatabase;
use introspect_macros::i_type::IExtract;
use introspect_macros::syntax::attribute::parse_attribute_args;
use introspect_macros::utils::str_to_token_stream;
use introspect_macros::{CairoFormat, Struct, SyntaxItemTrait};

use crate::I_TABLE_PATH;
use crate::interface::TableInterface;
use crate::set::ColumnSet;
use crate::structure::TableStructure;

#[attribute_macro]
fn table(args: TokenStream, module: TokenStream) -> ProcMacroResult {
    // let db = SimpleParserDatabase::default();
    let mut struct_item = Struct::from_token_stream(module).unwrap();
    let table_struct = TableStructure::iextract(&mut struct_item).unwrap();
    let table_interface = TableInterface::iextract(&mut struct_item).unwrap();
    let args = parse_attribute_args(args);
    // println!("Parsed Args: {:?}", args);
    let string = struct_item.to_cairo()
        + &table_struct.get_structure_impl(I_TABLE_PATH, I_TABLE_PATH)
        + &table_interface.table_impl(I_TABLE_PATH, I_TABLE_PATH, &table_struct.impl_name);
    ProcMacroResult::new(str_to_token_stream(&string))
}

#[attribute_macro]
fn column_set(_: TokenStream, token_stream: TokenStream) -> ProcMacroResult {
    let mut struct_item = Struct::from_token_stream(token_stream).unwrap();
    let column_set = ColumnSet::iextract(&mut struct_item).unwrap();
    let string = column_set.column_set_impl(I_TABLE_PATH);
    println!("{}\n", string);
    ProcMacroResult::new(str_to_token_stream(&string))
}
