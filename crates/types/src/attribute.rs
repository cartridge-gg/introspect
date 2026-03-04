use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub data: Option<Vec<u8>>,
}

impl Attribute {
    pub fn new_empty(name: String) -> Attribute {
        Attribute { name, data: None }
    }
}

pub trait Attributes {
    fn attributes(&self) -> &[Attribute];
    fn get_attribute(&self, name: &str) -> Option<Option<&[u8]>> {
        self.attributes()
            .iter()
            .find(|attr| attr.name == name)
            .map(|attr| attr.data.as_deref())
    }
    fn has_attribute(&self, name: &str) -> bool {
        self.attributes().iter().any(|attr| attr.name == name)
    }
}

impl<T> Attributes for T
where
    T: Deref<Target = [Attribute]>,
{
    fn attributes(&self) -> &[Attribute] {
        self.deref()
    }
}
