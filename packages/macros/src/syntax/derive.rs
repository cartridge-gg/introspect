use crate::syntax::fmt::CodeBuffer;
use crate::syntax::{ExprPath, PathSegment};
use crate::{
    Arg, Attribute, CairoCollectionFormat, CairoFormat, IntrospectError, IntrospectResult,
};
use std::ops::Deref;

const DERIVE_MACRO_NAME: &str = "derive";

impl Attribute {
    pub fn to_derives(self) -> IntrospectResult<Vec<String>> {
        self.arguments
            .ok_or(IntrospectError::DeriveMacroMissingArgs)?
            .iter()
            .map(Arg::as_unnamed)
            .collect::<Option<Vec<String>>>()
            .ok_or(IntrospectError::InvalidDerivesArgumentFormat)
    }

    pub fn is_derive(&self) -> bool {
        self.path
            == ExprPath {
                dollar: false,
                path: vec![PathSegment::Simple(DERIVE_MACRO_NAME.to_string())],
            }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Derives(pub Vec<String>);

impl Deref for Derives {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Derives {
    pub fn split_derives(attributes: Vec<Attribute>) -> IntrospectResult<(Vec<Attribute>, Self)> {
        let mut other_attributes = Vec::new();
        let mut derives = Vec::new();
        for attr in attributes {
            if attr.is_derive() {
                derives.extend(attr.to_derives()?);
            } else {
                other_attributes.push(attr);
            }
        }
        Ok((other_attributes, derives.into()))
    }
}

impl From<Vec<String>> for Derives {
    fn from(value: Vec<String>) -> Self {
        Derives(value)
    }
}

impl<T: CodeBuffer> CairoFormat<T> for Derives {
    fn cfmt(&self, buf: &mut T) {
        if !self.is_empty() {
            buf.push_token_char('#');
            buf.push_token_char('[');
            buf.push_token_str("derive");
            self.cfmt_csv_parenthesized(buf);
            buf.push_token_str("]\n");
        }
    }
}
