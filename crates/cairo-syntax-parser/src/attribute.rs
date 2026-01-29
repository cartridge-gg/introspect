use crate::{
    Expr, ExprPath, Modifier, from_typed_syntax_node, syntax_enum, syntax_option, syntax_type,
    terminal_to_string, typed_syntax_node_to_string_without_trivia, vec_from_element_list,
};
use delegate::delegate;
use std::ops::Deref;

syntax_type! {
    Attribute{
        path[attr]: ExprPath,
        arguments: Option<Vec<Arg>>,
        path_str[attr]: String,
    }
}

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

vec_from_element_list!(AttributeList, Attribute);
vec_from_element_list!(ArgListParenthesized.arguments, Arg);

syntax_option! {OptionArgListParenthesized{ArgListParenthesized: Vec<Arg>}}

impl Attribute {
    /// Get cached string representation of path (zero allocation)
    pub fn path_str(&self) -> &str {
        &self.path_str
    }

    pub fn path_string(&self) -> String {
        self.path_str.clone()
    }
}

impl Deref for Arg {
    type Target = ArgClause;

    fn deref(&self) -> &Self::Target {
        &self.clause
    }
}

impl Arg {
    delegate! {
        to self.clause {
            // Consuming methods that Deref can't handle
            pub fn to_unnamed(self) -> Option<Expr>;
            pub fn to_shorthand(self) -> Option<String>;
            pub fn to_named(self) -> Option<NamedArg>;
        }
    }
}

impl ArgClause {
    pub fn as_unnamed(&self) -> Option<&Expr> {
        match self {
            ArgClause::Unnamed(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_shorthand(&self) -> Option<&str> {
        match self {
            ArgClause::Shorthand(name) => Some(name),
            _ => None,
        }
    }

    pub fn as_named(&self) -> Option<&NamedArg> {
        match self {
            ArgClause::Named(named_arg) => Some(named_arg),
            _ => None,
        }
    }

    pub fn to_unnamed(self) -> Option<Expr> {
        match self {
            ArgClause::Unnamed(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_shorthand(self) -> Option<String> {
        match self {
            ArgClause::Shorthand(name) => Some(name),
            _ => None,
        }
    }

    pub fn to_named(self) -> Option<NamedArg> {
        match self {
            ArgClause::Named(named_arg) => Some(named_arg),
            _ => None,
        }
    }
}
