use crate::{Enum, IntrospectError, Result, Struct, TryFromAst};
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::kind::SyntaxKind;
pub struct SyntaxModule<'db> {
    pub node: SyntaxNode<'db>,
    pub db: &'db dyn salsa::Database,
}

pub enum ModuleItem {
    Struct(Struct),
    Enum(Enum),
}

impl<'db> SyntaxModule<'db> {
    pub fn new(db: &'db dyn salsa::Database, node: SyntaxNode<'db>) -> Self {
        Self { db, node }
    }

    pub fn get_struct(&self) -> Result<Struct> {
        for child in self.node.get_children(self.db)[0].get_children(self.db) {
            let kind = child.kind(self.db);
            match kind {
                SyntaxKind::ItemStruct => {
                    return Struct::try_from_syntax_node(self.db, *child);
                }
                _ => continue,
            }
        }
        Err(IntrospectError::NoStruct())
    }

    pub fn get_enum(&self) -> Result<Enum> {
        for child in self.node.get_children(self.db)[0].get_children(self.db) {
            let kind = child.kind(self.db);
            match kind {
                SyntaxKind::ItemEnum => {
                    return Enum::try_from_syntax_node(self.db, *child);
                }
                _ => continue,
            }
        }
        Err(IntrospectError::NoItem())
    }

    pub fn get_item(&self) -> Result<ModuleItem> {
        for child in self.node.get_children(self.db)[0].get_children(self.db) {
            let kind = child.kind(self.db);
            match kind {
                SyntaxKind::ItemStruct => {
                    return Struct::try_from_syntax_node(self.db, *child).map(ModuleItem::Struct);
                }
                SyntaxKind::ItemEnum => {
                    return Enum::try_from_syntax_node(self.db, *child).map(ModuleItem::Enum);
                }
                _ => continue,
            }
        }
        Err(IntrospectError::NoItem())
    }
}
