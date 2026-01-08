use crate::attribute::{Attribute, AttributeArg};
use crate::{AsCairo, IntrospectError, Result};
use std::ops::Deref;

const DERIVE_MACRO_NAME: &str = "derive";

impl Attribute {
    pub fn to_derives(self) -> Result<Vec<String>> {
        self.args
            .ok_or(IntrospectError::DeriveMacroMissingArgs)?
            .into_iter()
            .map(AttributeArg::to_unnamed)
            .collect::<Option<Vec<String>>>()
            .ok_or(IntrospectError::InvalidDerivesArgumentFormat)
    }

    pub fn is_derive(&self) -> bool {
        self.name == DERIVE_MACRO_NAME
    }
}
pub struct Derives(pub Vec<String>);

impl Deref for Derives {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Derives {
    pub fn split_derives(attributes: Vec<Attribute>) -> Result<(Vec<Attribute>, Self)> {
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

impl AsCairo for Derives {
    fn as_cairo(&self) -> String {
        match self.0.is_empty() {
            true => "".to_string(),
            false => format!("#[derive({})]\n", self.0.join(", ")),
        }
    }
}

// pub fn split_derives_attribute<'db>(
//     attributes: AttributeList<'db>,
//     db: &'db dyn Database,
// ) -> Result<(Vec<Attribute<'db>>, Vec<String>)> {
//     let attributes = parse_attributes(attributes, db);
//     let mut other_attributes = Vec::new();
//     let mut derives = Vec::new();
//     for attr in attributes {
//         if attr.name == DERIVE_MACRO_NAME {
//             for derive in get_derives_from_macro(&attr)? {
//                 derives.push(derive)
//             }
//         } else {
//             other_attributes.push(attr);
//         }
//     }
//     Ok((other_attributes, derives))
// }
