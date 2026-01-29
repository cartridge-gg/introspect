use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::TokenTreeNode;
use salsa::Database;

use crate::{
    Expr, FromAst, from_typed_syntax_node, syntax_enum, syntax_option, syntax_terminal_bool,
    syntax_terminal_enum, syntax_type, terminal_to_string, vec_from_element_list,
};

pub trait NameTrait {
    fn name(&self) -> &str;
    fn set_name(&mut self, new_name: String);
}

pub trait VisibilityTrait {
    fn visibility(&self) -> &Visibility;
    fn visibility_mut(&mut self) -> &mut Visibility;
}

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

syntax_enum! {
    Visibility{
        Default,
        Pub(Option<String>),
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

terminal_to_string! {VisibilityPubArgumentClause.argument}

from_typed_syntax_node!(VisibilityPub.argument_clause, Option<String>);

syntax_option! {OptionVisibilityPubArgumentClause{VisibilityPubArgumentClause: String}}

impl<'db> FromAst<'db, TokenTreeNode<'db>> for String {
    fn from_ast(node: TokenTreeNode<'db>, db: &'db dyn Database) -> Self {
        node.as_syntax_node().get_text(db).to_string()
    }
}
