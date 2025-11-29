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
