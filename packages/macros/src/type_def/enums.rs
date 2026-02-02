use super::CWriteTypeDef;
use crate::i_type::attribute::IAttributesTrait;
use crate::i_type::byte_array::CWriteIBytes;
use crate::item::ItemTrait;
use crate::type_def::FieldsTrait;
use crate::{IEnum, IVariant};
use cairo_syntax_parser::{CairoWrite, CairoWriteSlice};
use std::fmt::{Result as FmtResult, Write};

impl CWriteTypeDef for IEnum {
    fn cwrite_type_def<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        let name = &self.name;
        let name_size = name.len().div_ceil(31);
        let attribute_size: u32 = self.attributes.size();
        let variant_count = self.variants.len();
        let variant_prefix = &format!("{name}Variant");
        let variant_names = self
            .variants
            .iter()
            .map(|v| v.name.as_str())
            .collect::<Vec<&str>>();
        let instantiated_name = self.instantiated_name();
        let generics_with_traits = self.generics_with_traits(&[&format!("{i_path}::TypeDef")]);
        let generics_call = self.generics_call();
        write!(
            buf,
            "pub impl {name}TypeDef{generics_with_traits} of {i_path}::EnumDef<{instantiated_name}, {name_size}, {attribute_size}>{{\n"
        )?;
        name.to_const_byte_array(buf, "NAME")?;
        self.attributes.cwrite_attribute_count(buf)?;
        self.attributes.cwrite_const_attributes(buf)?;
        writeln!(buf, "const VARIANTS_COUNT: u32 = {variant_count};")?;
        buf.write_str("const REF: bool = false;\n")?;
        variant_names.cwrite_serialize(buf, "variant", &generics_call, variant_prefix)?;
        variant_names.cwrite_collect_children(
            buf,
            i_path,
            "variant",
            &generics_call,
            variant_prefix,
        )?;
        variant_names.write_serialize_with_children(
            buf,
            i_path,
            "variant",
            &generics_call,
            variant_prefix,
        )?;
        buf.write_str("}\n")?;

        writeln!(buf, "mod {variant_prefix} {{")?;
        for variant in &self.variants {
            variant.cwrite_type_def(buf, i_path)?;
        }
        buf.write_str("}\n")
    }
}

impl CWriteTypeDef for IVariant {
    fn cwrite_type_def<W: Write>(&self, buf: &mut W, i_path: &str) -> FmtResult {
        let size = self.name.len().div_ceil(31) as u32 + self.attributes.size() + 1;
        let selector = &self.selector.to_fixed_hex_string();
        let name = &self.name;
        let attribute_count = self.attributes.len();
        write!(
            buf,
            "pub impl {name} of {i_path}::VariantDef<{size}> {{
            const SELECTOR: felt252 = {selector};
            const META_DATA: [felt252; {size}] = [
            "
        )?;
        self.name.to_serialized_bytes(buf)?;
        write!(buf, ", {attribute_count}, ")?;
        self.attributes.cwrite_csv(buf)?;
        buf.write_str("];\ntype Type = ")?;
        self.ty.as_deref().unwrap_or("()").cwrite(buf)?;
        buf.write_str(";\n}\n")
    }
}
