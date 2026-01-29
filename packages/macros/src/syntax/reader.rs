use crate::syntax::items_from_token_stream;
use crate::syntax::module::Item;
use crate::utils::str_to_token_stream;
use crate::{CairoCollectionFormat, CairoFormat};
use cairo_lang_macro::{ProcMacroResult, TokenStream, attribute_macro, derive_macro};

#[attribute_macro]
pub fn parse(_attr: TokenStream, code: TokenStream) -> ProcMacroResult {
    // let db = SimpleParserDatabase::default();
    // let (node, _diagnostics) = db.parse_virtual_with_diagnostics(code);
    let items = items_from_token_stream(code);
    let string: String = items.to_cairo_block();
    let stream = str_to_token_stream(&string);
    println!("PARSE MACRO RAN");
    println!("{string}");
    ProcMacroResult::new(stream)
}

#[attribute_macro]
pub fn second(_attr: TokenStream, code: TokenStream) -> ProcMacroResult {
    println!("---------------------second---------------------");
    ProcMacroResult::new(code)
}

#[attribute_macro]
pub fn first(_attr: TokenStream, code: TokenStream) -> ProcMacroResult {
    let mut items = items_from_token_stream(code);
    println!("FIRST MACRO RAN");

    // Remove second attribute
    for item in &mut items {
        if let Item::Struct(m) = item {
            m.attributes
                .retain(|attr| CairoFormat::<String>::to_cairo(&attr.path) != "second");
        }
    }

    println!(
        "------------------------------\n{}",
        CairoCollectionFormat::<String>::to_cairo_block(&items)
    );

    let string: String = items.to_cairo_block();
    ProcMacroResult::new(str_to_token_stream(&string))
}

#[derive_macro]
#[allow(non_snake_case)]
pub fn Something(token_stream: TokenStream) -> ProcMacroResult {
    let mut _items = items_from_token_stream(token_stream);
    println!("------------------something------------------");
    let string = "impl MyTraitImpl of MyTrait<Bloop>{ }";
    ProcMacroResult::new(str_to_token_stream(string))
}
