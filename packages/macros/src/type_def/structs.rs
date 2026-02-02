use super::CWriteTypeDef;
use crate::i_type::attribute::IAttributesTrait;
use crate::i_type::byte_array::CWriteIBytes;
use crate::item::ItemTrait;
use crate::type_def::FieldsTrait;
use crate::{IMember, IStruct};
use cairo_syntax_parser::{CairoWrite, CairoWriteSlice};
use std::fmt::{Result as FmtResult, Write};

impl CWriteTypeDef for IStruct {
    fn cwrite_type_def<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        let name = &self.name;
        let name_size = name.len().div_ceil(31);
        let attribute_size: u32 = self.attributes.size();
        let member_count = self.members.len();
        let member_prefix = &format!("{name}Member");
        let member_names = self
            .members
            .iter()
            .map(|m| m.name.as_str())
            .collect::<Vec<&str>>();
        let instantiated_name = self.instantiated_name();
        let generics_with_traits = self.generics_with_traits(&[&format!("{i_path}::TypeDef")]);
        let generics_call = self.generics_call();
        write!(
            buf,
            "pub impl {name}TypeDef{generics_with_traits} of {i_path}::StructDef<{instantiated_name}, {name_size}, {attribute_size}>{{\n"
        )?;
        name.to_const_byte_array(buf, "NAME")?;
        self.attributes.cwrite_attribute_count(buf)?;
        self.attributes.cwrite_const_attributes(buf)?;
        write!(buf, "const MEMBERS_COUNT: u32 = {member_count};\n")?;
        buf.write_str("const REF: bool = false;\n")?;
        member_names.cwrite_serialize(buf, "member", &generics_call, member_prefix)?;
        member_names.cwrite_collect_children(
            buf,
            i_path,
            "member",
            &generics_call,
            member_prefix,
        )?;
        member_names.write_serialize_with_children(
            buf,
            i_path,
            "member",
            &generics_call,
            member_prefix,
        )?;
        buf.write_str("}\n")?;

        writeln!(buf, "mod {member_prefix} {{")?;
        for member in &self.members {
            member.write_member_type_def(buf, i_path, &generics_with_traits)?;
        }
        buf.write_str("}\n")
    }
}

impl IMember {
    fn write_member_type_def<W: Write>(
        &self,
        buf: &mut W,
        i_path: &str,
        generics_with_traits: &str,
    ) -> FmtResult {
        let size = self.name.len().div_ceil(31) as u32 + self.attributes.size() + 1;
        let name = &self.name;
        let attribute_count = self.attributes.len();
        write!(
            buf,
            "pub impl {name}{generics_with_traits} of {i_path}::MemberDef<{size}> {{
            const META_DATA: [felt252; {size}] = ["
        )?;
        self.name.to_serialized_bytes(buf)?;
        write!(buf, ", {attribute_count}, ")?;
        self.attributes.cwrite_csv(buf)?;
        buf.write_str("];\ntype Type = ")?;
        self.ty.cwrite(buf)?;
        buf.write_str(";\n}\n")
    }
}
