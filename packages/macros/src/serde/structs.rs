use itertools::Itertools;

use crate::serde::{ISERDE_DESERIALIZE_CALL, ISERDE_SERIALIZE_CALL, ToISerdeImpl};
use crate::{IMember, IStruct};

impl ToISerdeImpl for IStruct {
    fn serialize_body(&self) -> String {
        self.members
            .iter()
            .map(IMember::serialize_member)
            .join("\n")
    }
    fn deserialize_body(&self) -> String {
        let members: Vec<_> = self.members.iter().map(|m| m.name.as_str()).collect();
        members.iter().cloned().map(deserialize_member).join("\n")
            + &format!("\nSome({}{{{}}})", self.name, members.join(","))
    }
}

pub fn deserialize_member(name: &str) -> String {
    format!("let {name} = {ISERDE_DESERIALIZE_CALL}(ref serialized)?;")
}

impl IMember {
    pub fn serialize_member(&self) -> String {
        format!("{ISERDE_SERIALIZE_CALL}(self.{}, ref output);", self.name)
    }
    pub fn deserialize_member(&self) -> String {
        deserialize_member(&self.name)
    }
}
