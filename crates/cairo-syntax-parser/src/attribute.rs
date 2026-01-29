use crate::{
    CairoFormat, Expr, ExprPath, Modifier, from_typed_syntax_node, syntax_enum, syntax_option,
    syntax_type, terminal_to_string, typed_syntax_node_to_string_without_trivia,
    vec_from_element_list,
};
use delegate::delegate;
use std::mem;
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

const DERIVE_PATH: &str = "derive";

impl Attribute {
    /// Get cached string representation of path (zero allocation)
    pub fn path_str(&self) -> &str {
        &self.path_str
    }

    pub fn path_string(&self) -> String {
        self.path_str.clone()
    }

    pub fn get_derives(&self) -> Vec<String> {
        if self.path_str == DERIVE_PATH {
            if let Some(args) = &self.arguments {
                return args
                    .iter()
                    .filter_map(Arg::as_unnamed)
                    .map(|expr| expr.to_string())
                    .collect();
            }
        }
        vec![]
    }

    pub fn is_single_unnamed_arg(self) -> bool {
        match &self.arguments {
            Some(args) if args.len() == 1 => args[0].as_unnamed().is_some(),
            _ => false,
        }
    }
}

pub trait AttributesTrait {
    fn attributes_mut(&mut self) -> &mut Vec<Attribute>;
    fn attributes(&self) -> &[Attribute];
    fn has_attribute(&self, name: &str) -> bool {
        self.attributes().iter().any(|attr| attr.path_str() == name)
    }
    fn has_name_only_attribute(&self, name: &str) -> bool {
        self.attributes()
            .iter()
            .any(|attr| attr.path_str() == name && attr.arguments.is_none())
    }
    fn get_attribute(&self, name: &str) -> Option<&Attribute> {
        self.attributes()
            .iter()
            .find(|attr| attr.path_str() == name)
    }
    fn update_attributes(&mut self, attributes: Vec<Attribute>) {
        *self.attributes_mut() = attributes;
    }
    fn take_attributes(&mut self) -> Vec<Attribute> {
        mem::take(self.attributes_mut())
    }
    fn push_attribute(&mut self, attribute: Attribute) {
        self.attributes_mut().push(attribute);
    }
    fn derives(&self) -> Vec<String> {
        self.attributes()
            .iter()
            .flat_map(|attr| attr.get_derives())
            .collect()
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
            pub fn as_unnamed(&self) -> Option<&Expr>;
            pub fn as_shorthand(&self) -> Option<&str>;
            pub fn as_named(&self) -> Option<&NamedArg>;
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
