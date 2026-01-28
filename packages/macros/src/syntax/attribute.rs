use crate::syntax::ExprPath;
use crate::{Arg, IntrospectError, IntrospectResult, syntax_type, vec_from_element_list};

use core::mem;

syntax_type! {
    Attribute{
        path[attr]: ExprPath,
        arguments: Option<Vec<Arg>>,
        path_str[attr]: String,
    }
}

vec_from_element_list!(AttributeList, Attribute);

impl Attribute {
    /// Get cached string representation of path (zero allocation)
    pub fn path_str(&self) -> &str {
        &self.path_str
    }

    pub fn path_string(&self) -> String {
        self.path_str.clone()
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
        IntrospectError::InvalidIntrospectAttributeFormat(self.path_string())
    }
    pub fn format_err<T>(&self) -> IntrospectResult<T> {
        Err(self.format_error())
    }
    pub fn single_unnamed_arg(&self) -> IntrospectResult<String> {
        match &self.arguments {
            Some(args) if args.len() == 1 => args[0].as_unnamed().ok_or(self.format_error()),
            _ => Err(self.format_error()),
        }
    }
    pub fn all_unnamed_args(&self) -> IntrospectResult<Vec<String>> {
        match &self.arguments {
            Some(args) => args
                .iter()
                .map(|arg| arg.as_unnamed().ok_or(self.format_error()))
                .collect(),
            None => Ok(vec![]),
        }
    }
}
