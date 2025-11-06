use crate::introspect::IntrospectImpl;
use crate::utils::str_to_token_stream;
use crate::{IntrospectType, get_introspection_type};
use cairo_lang_macro::{ProcMacroResult, TokenStream, derive_macro};
use cairo_lang_parser::utils::SimpleParserDatabase;

#[derive_macro]
fn introspect(token_stream: TokenStream) -> ProcMacroResult {
    let db = SimpleParserDatabase::default();
    let (parsed, _diag) = db.parse_virtual_with_diagnostics(token_stream.clone());
    let string = match get_introspection_type(&db, parsed).unwrap() {
        IntrospectType::Struct(struct_item) => struct_item.to_impl(),
        IntrospectType::Enum(enum_item) => enum_item.to_impl(),
    };
    println!("{}", string);
    ProcMacroResult::new(str_to_token_stream(&string))
}
