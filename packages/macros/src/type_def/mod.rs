pub mod derive;
pub mod enums;
pub mod structs;

use crate::IntrospectItem;
use crate::i_type::attribute::IAttributesTrait;
use crate::i_type::byte_array::CWriteIBytes;
use crate::i_type::{IFieldTrait, IFieldsTrait};
use crate::item::ItemTrait;
use cairo_syntax_parser::{CairoWriteSlice, NameTrait};
use std::fmt::{Result as FmtResult, Write};

pub trait TypeDefField: IAttributesTrait + IFieldTrait {
    fn cwrite_pre_meta_data<W: Write>(&self, _buf: &mut W) -> FmtResult {
        Ok(())
    }
    fn write_field_def<W: Write>(
        &self,
        buf: &mut W,
        i_path: &str,
        impl_name: &str,
        generics_with_traits: &str,
    ) -> FmtResult {
        let ty = self.ty();
        write!(
            buf,
            "impl {impl_name}{generics_with_traits} = {i_path}::FieldDef<{ty}, _, {{["
        )?;
        self.cwrite_pre_meta_data(buf)?;
        self.name().to_iserialized_bytes(buf)?;
        let attribute_count = self.iattributes().len();
        write!(buf, ", {attribute_count}")?;
        if attribute_count > 0 {
            buf.write_char(',')?;
            self.iattributes().cwrite_csv(buf)?;
        }
        buf.write_str("]}>;\n")
    }
}

pub trait CWriteTypeDef {
    fn cwrite_type_def<W: Write>(
        &self,
        buf: &mut W,
        i_path: &str,
        default_as_ref: bool,
    ) -> FmtResult;
    fn type_def_impl_to_string(&self, i_path: &str, default_as_ref: bool) -> String {
        let mut buf = String::new();
        self.cwrite_type_def(&mut buf, i_path, default_as_ref)
            .unwrap();
        buf
    }
    fn cwrite_field_calls<W: Write>(
        &self,
        buf: &mut W,
        impl_names: &[String],
        function_call: &str,
        generics_call: &str,
        field_function_call: &str,
    ) -> FmtResult {
        writeln!(buf, "fn {function_call} {{")?;
        for impl_name in impl_names {
            writeln!(buf, "{impl_name}{generics_call}::{field_function_call};")?;
        }
        buf.write_str("}\n")
    }
}
impl<T> CWriteTypeDef for T
where
    T: NameTrait + IAttributesTrait + IFieldsTrait + ItemTrait,
    <T as IFieldsTrait>::Field: TypeDefField,
{
    fn cwrite_type_def<W: Write>(
        &self,
        buf: &mut W,
        i_path: &str,
        default_as_ref: bool,
    ) -> FmtResult {
        let name = self.name();
        let name_size = name.len().div_ceil(31);
        let fields_count = self.fields().len();
        let instantiated_name = self.instantiated_name();
        let generics_with_traits = self.generics_with_traits(&[&format!("{i_path}::TypeDef")]);
        let generics_call = &self.generics_call();
        let attribute_size: u32 = self.iattributes_size();
        let def_selector = self.type_selector();
        let mut impl_names = Vec::new();
        for field in self.fields() {
            let impl_name = format!("{name}_{}_Def", field.name());
            field.write_field_def(buf, i_path, &impl_name, &generics_with_traits)?;
            impl_names.push(impl_name);
        }
        writeln!(
            buf,
            "pub impl {name}TypeDef{generics_with_traits} of {i_path}::CompoundDef<{instantiated_name}, {name_size}, {attribute_size}>{{"
        )?;
        writeln!(buf, "const DEF_SELECTOR: felt252 = {def_selector};")?;

        name.to_const_byte_array(buf, "NAME")?;
        self.cwrite_attribute_count(buf)?;
        self.cwrite_const_attributes(buf)?;
        writeln!(buf, "const FIELDS_COUNT: u32 = {fields_count};")?;
        self.cwrite_field_calls(
            buf,
            &impl_names,
            "serialize_fields(ref output: Array<felt252>)",
            generics_call,
            "serialize(ref output)",
        )?;
        self.cwrite_field_calls(
            buf,
            &impl_names,
            &format!("collect_field_children(ref children: {i_path}::ChildDefs)"),
            generics_call,
            "collect_children(ref children)",
        )?;
        self.cwrite_field_calls(
            buf,
            &impl_names,
            &format!("serialize_fields_with_children(ref type_def: Array<felt252>, ref children: {i_path}::ChildDefs)"),
            generics_call,
"serialize_with_children(ref type_def, ref children)"
        )?;
        buf.write_str("}\n")?;
        if default_as_ref {
            writeln!(
                buf,
                "pub impl {name}DefaultToRef{generics_with_traits} of {i_path}::DefaultToRef<{instantiated_name}>;"
            )?;
        }
        Ok(())
    }
}

impl CWriteTypeDef for IntrospectItem {
    fn cwrite_type_def<W: Write>(
        &self,
        buf: &mut W,
        i_path: &str,
        default_as_ref: bool,
    ) -> FmtResult {
        match self {
            IntrospectItem::Struct(s) => s.cwrite_type_def(buf, i_path, default_as_ref),
            IntrospectItem::Enum(e) => e.cwrite_type_def(buf, i_path, default_as_ref),
        }
    }
}
