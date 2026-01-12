use crate::fuzzable::FuzzableImpl;
use crate::{Member, Struct};
use itertools::Itertools;

impl FuzzableImpl for Struct {
    fn fuzzable_body(&self) -> String {
        let members = self.members.iter().map(fuzzable_member).join(",");
        format!("{}{{{members}}}", self.name)
    }
}

fn fuzzable_member(member: &Member) -> String {
    format!(
        "{}: snforge_std::fuzzable::Fuzzable::generate()",
        member.name
    )
}
