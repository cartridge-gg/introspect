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
    fn cwrite_size_hint<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        match self.members.len() {
            0 => buf.write_str("Some(0);\n"),
            1 => {
                writeln!(buf, "{i_path}::size_hint::<{}>();", self.members[0].ty)
            }
            _ => {
                let tys = self.field_tys();
                let [members @ .., second_last, last] = tys.as_slice() else {
                    unreachable!()
                };
                members
                    .iter()
                    .try_for_each(|m| write!(buf, "{i_path}::add_size_hint::<{m}>("))?;
                write!(buf, "{i_path}::match_size_hints::<{second_last}, {last}>()",)?;
                (0..members.len()).try_for_each(|_| buf.write_char(')'))?;
                buf.write_str(";\n")
            }
        }
    }
    fn cwrite_isize<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        if self.members.is_empty() {
            buf.write_str("0\n")
        } else {
            let fields = self.field_fields();
            let (last, members) = fields.split_last().unwrap();
            members
                .iter()
                .try_for_each(|f| write!(buf, "{i_path}::iserialized_size(self.{f}) + "))?;
            writeln!(buf, "{i_path}::iserialized_size(self.{last})")
        }
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
