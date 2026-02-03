use crate::i_type::{IFieldTrait, IFieldsTrait};
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
    fn cwrite_size_hint<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        match self.variants.len() {
            0 => buf.write_str("Some(1);\n"),
            1 => writeln!(
                buf,
                "{i_path}::size_hint_add_checked::<{}>(1);",
                self.variants[0].ty()
            ),
            _ => {
                write!(buf, "{i_path}::add_checked(")?;
                let tys = self.field_tys();
                let [variants @ .., second_last, last] = tys.as_slice() else {
                    unreachable!()
                };
                variants
                    .iter()
                    .try_for_each(|m| write!(buf, "{i_path}::match_size_hint::<{m}>("))?;
                write!(buf, "{i_path}::match_size_hints::<{second_last}, {last}>()",)?;
                (0..variants.len()).try_for_each(|_| buf.write_char(')'))?;
                buf.write_str(", 1);\n")
            }
        }
    }
    fn cwrite_isize<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        if self.variants.is_empty() {
            buf.write_str("0\n")
        } else {
            buf.write_str("match self {\n")?;
            self.variants
                .iter()
                .try_for_each(|f| f.cwrite_vairant_isize(buf, i_path, &self.name))?;
            buf.write_str("}\n")
        }
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

    pub fn cwrite_vairant_isize<W: Write>(
        &self,
        buf: &mut W,
        i_path: &str,
        enum_name: &str,
    ) -> FmtResult {
        let variant_name = &self.field;
        match &self.ty {
            None => writeln!(buf, "{enum_name}::{variant_name} => 1,"),
            Some(_) => writeln!(
                buf,
                "{enum_name}::{variant_name}(value) => {i_path}::iserialized_size(value) + 1,",
            ),
        }
    }
}
