use crate::serde::{ISERDE_SERIALIZE_CALL, ToISerdeImpl};
use crate::{IMember, IStruct};

impl ToISerdeImpl for IStruct {
    fn iserde_body(&self) -> String {
        self.members
            .iter()
            .map(IMember::iserde_member)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl IMember {
    pub fn iserde_member(&self) -> String {
        format!("{ISERDE_SERIALIZE_CALL}(self.{}, ref output);", self.name)
    }
}
