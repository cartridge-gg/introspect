use cairo_syntax_parser::Attribute;

use crate::{IntrospectError, IntrospectResult};

use core::mem;

#[macro_export]
macro_rules! impl_attributes_trait {
    ($type:ty) => {
        $crate::impl_attributes_trait!($type, attributes);
    };
    ($type:ty, $field:ident) => {
        impl cairo_syntax_parser::AttributesTrait for $type {
            fn attributes_mut(&mut self) -> &mut Vec<cairo_syntax_parser::Attribute> {
                &mut self.$field
            }
            fn attributes(&self) -> &[cairo_syntax_parser::Attribute] {
                &self.$field
            }
        }
    };
}
