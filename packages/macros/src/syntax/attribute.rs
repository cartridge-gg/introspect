use crate::{
    Arg, AsCairo, AstInto, FromAst, IntrospectError, IntrospectResult, vec_from_element_list,
};
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::Attribute as AstAttribute;
use core::mem;
use salsa::Database;

#[derive(Clone, Debug, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub args: Option<Vec<Arg>>,
}

pub trait AttributesTrait {
    fn attributes_mut(&mut self) -> &mut Vec<Attribute>;
    fn attributes(&self) -> &[Attribute];
    fn has_attribute(&self, name: &str) -> bool {
        self.attributes()
            .iter()
            .any(|attr| attr.name.as_str() == name)
    }
    fn has_name_only_attribute(&self, name: &str) -> bool {
        self.attributes()
            .iter()
            .any(|attr| attr.name.as_str() == name && attr.args.is_none())
    }
    fn get_attribute(&self, name: &str) -> Option<&Attribute> {
        self.attributes()
            .iter()
            .find(|attr| attr.name.as_str() == name)
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
}

#[macro_export]
macro_rules! impl_attributes_trait {
    ($type:ty) => {
        $crate::impl_attributes_trait!($type, attributes);
    };
    ($type:ty, $field:ident) => {
        impl $crate::syntax::attribute::AttributesTrait for $type {
            fn attributes_mut(&mut self) -> &mut Vec<$crate::syntax::attribute::Attribute> {
                &mut self.$field
            }
            fn attributes(&self) -> &[$crate::syntax::attribute::Attribute] {
                &self.$field
            }
        }
    };
}

impl Attribute {
    pub fn format_error(&self) -> IntrospectError {
        IntrospectError::InvalidIntrospectAttributeFormat(self.name.clone())
    }
    pub fn format_err<T>(&self) -> IntrospectResult<T> {
        Err(self.format_error())
    }
    pub fn single_unnamed_arg(&self) -> IntrospectResult<String> {
        match &self.args {
            Some(args) if args.len() == 1 => args[0].as_unnamed().ok_or(self.format_error()),
            _ => Err(self.format_error()),
        }
    }
    pub fn all_unnamed_args(&self) -> IntrospectResult<Vec<String>> {
        match &self.args {
            Some(args) => args
                .iter()
                .map(|arg| arg.as_unnamed().ok_or(self.format_error()))
                .collect(),
            None => Ok(vec![]),
        }
    }
}

impl AsCairo for Attribute {
    fn as_cairo(&self) -> String {
        format!("#[{}{}]", self.name, self.args.as_cairo())
    }
}

impl<'db> FromAst<'db, AstAttribute<'db>> for Attribute {
    fn from_ast(ast_attribute: AstAttribute<'db>, db: &'db dyn Database) -> Self {
        Self {
            name: ast_attribute
                .attr(db)
                .as_syntax_node()
                .get_text_without_trivia(db)
                .to_string(db),
            args: ast_attribute.arguments(db).ast_into(db),
        }
    }
}

vec_from_element_list!(AttributeList, Attribute);
