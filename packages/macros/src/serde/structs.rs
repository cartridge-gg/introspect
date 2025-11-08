use crate::serde::{ISERDE_SERIALIZE_CALL, ToISerdeImpl};
use crate::{Member, Struct};

impl<'db> ToISerdeImpl for Struct<'db> {
    fn iserde_body(&self) -> String {
        self.members
            .iter()
            .map(make_iserde_body_for_member)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn make_iserde_body_for_member<'db>(member: &Member<'db>) -> String {
    format!("{ISERDE_SERIALIZE_CALL}(self.{}, ref output);", member.name)
}
