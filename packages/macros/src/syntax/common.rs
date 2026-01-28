use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::TokenTreeNode;
use salsa::Database;

use crate::syntax::expr::Expr;
use crate::{
    AsCairo, FromAst, syntax_terminal_bool, syntax_terminal_enum, syntax_type,
    vec_from_element_list,
};

syntax_type! {
    Param{
        modifiers: Vec<Modifier>,
        name: String,
        type_clause: Option<Expr>,
    }
}

syntax_type! {
    Identifier[PatternIdentifier]{
        modifiers: Vec<Modifier>,
        name: String,
    }
}

syntax_terminal_enum! {
    Visibility{
        Default[VisibilityDefault],
        Pub[VisibilityPub],
    }
}

syntax_terminal_enum! {
    Modifier{
        Ref,
        Mut,
    }
}

vec_from_element_list!(ModifierList, Modifier);
vec_from_element_list!(ParamList, Param);

syntax_terminal_bool! {Const}
syntax_terminal_bool! {ColonColon}

impl<'db> FromAst<'db, TokenTreeNode<'db>> for String {
    fn from_ast(node: TokenTreeNode<'db>, db: &'db dyn Database) -> Self {
        node.as_syntax_node().get_text(db).to_string()
    }
}

impl AsCairo for Visibility {
    fn as_cairo(&self) -> String {
        match self {
            Visibility::Default => "".to_string(),
            Visibility::Pub => "pub ".to_string(),
        }
    }
}

impl AsCairo for Modifier {
    fn as_cairo(&self) -> String {
        match self {
            Modifier::Ref => "ref".to_string(),
            Modifier::Mut => "mut ".to_string(),
        }
    }
}

impl AsCairo for Vec<Modifier> {
    fn as_cairo(&self) -> String {
        self.into_iter().map(|m| m.as_cairo_suffixed(" ")).collect()
    }
}

impl AsCairo for Param {
    fn as_cairo(&self) -> String {
        let type_clause = match &self.type_clause {
            Some(ty) => format!(":{}", ty.as_cairo()),
            None => "".to_string(),
        };
        format!("{}{}{}", self.modifiers.as_cairo(), self.name, type_clause,)
    }
}
