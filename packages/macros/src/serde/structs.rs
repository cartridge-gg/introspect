use crate::IStruct;
use crate::i_type::item::IFieldsTrait;
use crate::serde::CWriteISerde;
use cairo_syntax_parser::CairoWriteSlice;
use std::fmt::{Result as FmtResult, Write};

impl CWriteISerde for IStruct {
    fn cwrite_iserialize<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        self.field_fields()
            .into_iter()
            .map(|m| serialize_member(buf, i_path, m))
            .collect::<FmtResult>()
    }

    fn cwrite_ideserialize<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        let fields = self.field_fields();
        fields
            .iter()
            .map(|m| deserialize_member(buf, i_path, m))
            .collect::<FmtResult>()?;
        write!(buf, "Some({}", self.name)?;
        fields.cwrite_csv_braced(buf)?;
        buf.write_str(")")
    }
}

pub fn serialize_member<W: Write>(buf: &mut W, i_path: &str, field: &str) -> FmtResult {
    writeln!(buf, "{i_path}::iserialize(self.{}, ref output);", field)
}
pub fn deserialize_member<W: Write>(buf: &mut W, i_path: &str, field: &str) -> FmtResult {
    writeln!(
        buf,
        "let {field} = {i_path}::ideserialize(ref serialized)?;"
    )
}
