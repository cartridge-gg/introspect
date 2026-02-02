use crate::serde::CWriteISerde;
use crate::{IMember, IStruct};
use cairo_syntax_parser::CairoWriteSlice;
use std::fmt::{Result as FmtResult, Write};

impl CWriteISerde for IStruct {
    fn cwrite_iserialize<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        self.members
            .iter()
            .map(|m| m.serialize_member(buf, i_path))
            .collect::<FmtResult>()
    }

    fn cwrite_ideserialize<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        self.members
            .iter()
            .map(|m| m.deserialize_member(buf, i_path))
            .collect::<FmtResult>()?;
        write!(buf, "Some({}", self.name)?;
        let names = self
            .members
            .iter()
            .map(|m| m.name.as_str())
            .collect::<Vec<&str>>();
        names.cwrite_csv_braced(buf)?;
        buf.write_str(")")
    }
}

impl IMember {
    pub fn serialize_member<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        writeln!(buf, "{i_path}::iserialize(self.{}, ref output);", self.name)
    }
    pub fn deserialize_member<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        let name = &self.name;
        writeln!(buf, "let {name} = {i_path}::ideserialize(ref serialized)?;")
    }
}
