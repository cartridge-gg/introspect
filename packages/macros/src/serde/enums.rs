use crate::serde::CWriteISerde;
use crate::{IEnum, IVariant};
use std::fmt::{Result as FmtResult, Write};

impl CWriteISerde for IEnum {
    fn cwrite_iserialize<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        buf.write_str("match self {\n")?;
        for variant in &self.variants {
            variant.cwrite_serialize_variant(buf, i_path, &self.name)?;
        }
        buf.write_str("}\n")
    }

    fn cwrite_ideserialize<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        writeln!(buf, "match *serialized.pop_front()? {{")?;
        for variant in &self.variants {
            variant.cwrite_deserialize_variant(buf, i_path, &self.name)?;
        }
        buf.write_str("_ => None\n}")
    }
}

impl IVariant {
    pub fn cwrite_serialize_variant<W: Write>(
        &self,
        buf: &mut W,
        i_path: &str,
        enum_name: &str,
    ) -> FmtResult {
        let selector = self.selector.to_fixed_hex_string();
        let variant_name = &self.field;
        match self.ty {
            None => writeln!(
                buf,
                "{enum_name}::{variant_name} => output.append({selector}),"
            ),
            Some(_) => writeln!(
                buf,
                "{enum_name}::{variant_name}(value) => {i_path}::iserialize_keyed_type({selector}, value, ref output),",
            ),
        }
    }

    pub fn cwrite_deserialize_variant<W: Write>(
        &self,
        buf: &mut W,
        i_path: &str,
        enum_name: &str,
    ) -> FmtResult {
        let selector = self.selector.to_fixed_hex_string();
        let field = &self.field;
        match &self.ty {
            None => writeln!(buf, "{selector} => Some({enum_name}::{field}),"),
            Some(_) => writeln!(
                buf,
                "{selector} => Some({enum_name}::{field}({i_path}::ideserialize(ref serialized)?)),",
            ),
        }
    }
}
