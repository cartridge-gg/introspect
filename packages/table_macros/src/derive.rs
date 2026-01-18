use cairo_lang_macro::{ProcMacroResult, TokenStream, derive_macro};
use introspect_macros::i_type::{DefaultIExtractor, IExtract};
use introspect_macros::utils::str_to_token_stream;
use introspect_macros::{AttributeCallType, Struct, SyntaxItemTrait};

use crate::I_TABLE_PATH;
use crate::structure::TableStructure;

#[allow(non_snake_case)]
#[derive_macro]
fn Table(token_stream: TokenStream) -> ProcMacroResult {
    let extractor = DefaultIExtractor::new(AttributeCallType::Derive);
    let mut struct_item = Struct::from_token_stream(token_stream).unwrap();
    let table_struct: TableStructure = extractor.iextract(&mut struct_item).unwrap();
    println!("{:#?}", table_struct);
    let string = table_struct.get_structure_impl(I_TABLE_PATH);
    println!("{}\n", string);
    ProcMacroResult::new(str_to_token_stream(&string))
}
