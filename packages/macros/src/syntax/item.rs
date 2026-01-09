use crate::{Enum, IntrospectError, Result, Struct, TryFromAst};
use cairo_lang_macro::TokenStream;
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::kind::SyntaxKind;
use salsa::Database;

pub enum Item {
    Struct(Struct),
    Enum(Enum),
}

pub trait SyntaxItemTrait
where
    Self: Sized,
{
    fn from_file_node<'db>(db: &'db dyn Database, node: SyntaxNode<'db>) -> Result<Self>;
    fn from_token_stream(token_stream: TokenStream) -> Result<Self> {
        let db = SimpleParserDatabase::default();
        let (node, _diagnostics) = db.parse_virtual_with_diagnostics(token_stream.clone());
        Self::from_file_node(&db, node)
    }
}

impl SyntaxItemTrait for Item {
    fn from_file_node<'db>(db: &'db dyn Database, node: SyntaxNode<'db>) -> Result<Self> {
        for child in node.get_children(db)[0].get_children(db) {
            let kind = child.kind(db);
            match kind {
                SyntaxKind::ItemStruct => {
                    return Struct::try_from_syntax_node(db, *child).map(Item::Struct);
                }
                SyntaxKind::ItemEnum => {
                    return Enum::try_from_syntax_node(db, *child).map(Item::Enum);
                }
                _ => continue,
            }
        }
        Err(IntrospectError::NoItem())
    }
}
