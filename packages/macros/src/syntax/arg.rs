use crate::syntax::Expr;
use crate::{
    CairoFormat, Modifier, from_typed_syntax_node, syntax_enum, syntax_option, syntax_type,
    terminal_to_string, typed_syntax_node_to_string_without_trivia, vec_from_element_list,
};

syntax_type! {
    Arg {
        modifiers: Vec<Modifier>,
        clause[arg_clause]: ArgClause,
    }
}

syntax_enum! {
    ArgClause{
        Unnamed(Expr),
        Named(NamedArg),
        Shorthand[FieldInitShorthand](String),
    }
}

syntax_type! {
    NamedArg[ArgClauseNamed]{
        name: String,
        value: Expr,
    }
}

from_typed_syntax_node!(ArgClauseUnnamed.value, Expr);
terminal_to_string! {ArgClauseFieldInitShorthand.name.name,}
typed_syntax_node_to_string_without_trivia! {ArgClauseUnnamed.value}
vec_from_element_list!(ArgListParenthesized.arguments, Arg);
syntax_option! {OptionArgListParenthesized{ArgListParenthesized: Vec<Arg>}}

impl Arg {
    pub fn as_unnamed(&self) -> Option<String> {
        self.clause.as_unnamed()
    }
}

impl ArgClause {
    pub fn as_unnamed(&self) -> Option<String> {
        match self {
            ArgClause::Unnamed(value) => Some(value.to_cairo()),
            _ => None,
        }
    }
}
