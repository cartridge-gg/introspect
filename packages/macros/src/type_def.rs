use crate::{AsCairo, AsCairoBytes, AsCairoWith, CollectionsAsCairo, IAttribute};
use introspect_types::{
    ByteArrayEDef, CustomDef, EnumDef, FixedArrayDef, MemberDef, ResultDef, StructDef, TypeDef,
    VariantDef,
};
use starknet_types_core::felt::Felt;

impl AsCairo for TypeDef {
    fn as_cairo(&self) -> String {
        match self {
            TypeDef::None
            | TypeDef::Felt252
            | TypeDef::ShortUtf8
            | TypeDef::Bytes31
            | TypeDef::Bool
            | TypeDef::U8
            | TypeDef::U16
            | TypeDef::U32
            | TypeDef::U64
            | TypeDef::U128
            | TypeDef::U256
            | TypeDef::U512
            | TypeDef::I8
            | TypeDef::I16
            | TypeDef::I32
            | TypeDef::I64
            | TypeDef::I128
            | TypeDef::ClassHash
            | TypeDef::ContractAddress
            | TypeDef::EthAddress
            | TypeDef::StorageAddress
            | TypeDef::StorageBaseAddress
            | TypeDef::ByteArray(_)
            | TypeDef::Utf8String(_) => format!("introspect::TypeDef::{}", self.item_name()),
            TypeDef::Bytes31E(encoding)
            | TypeDef::ByteArrayE(ByteArrayEDef { encoding, mode: _ })
            | TypeDef::Custom(CustomDef { encoding }) => {
                as_type_def(self.item_name(), encoding.as_cairo_byte_array())
            }
            TypeDef::Struct(s) => as_type_def("Struct", s.as_cairo()),
            TypeDef::Enum(e) => as_type_def("Enum", e.as_cairo()),
            TypeDef::Tuple(e) => as_type_def("Tuple", e.as_cairo_span()),
            TypeDef::FixedArray(e) => as_type_def_boxed("FixedArray", e.as_cairo()),
            TypeDef::Array(e) => as_type_def_boxed("Array", e.as_cairo()),
            TypeDef::Option(e) => as_type_def_boxed("Option", e.as_cairo()),
            TypeDef::Nullable(e) => as_type_def_boxed("Nullable", e.as_cairo()),
            TypeDef::Felt252Dict(e) => as_type_def_boxed("Felt252Dict", e.as_cairo()),
            TypeDef::Result(e) => as_type_def_boxed("Result", e.as_cairo()),
            TypeDef::Ref(e) => as_type_def("Ref", e.as_cairo()),
        }
    }
}

pub fn as_type_def(variant: &str, type_def: String) -> String {
    format!("introspect::TypeDef::{}({})", variant, type_def)
}

pub fn as_type_def_boxed(variant: &str, type_def: String) -> String {
    format!(
        "introspect::TypeDef::{}(BoxTrait::new({}))",
        variant, type_def
    )
}

pub fn attribute_data_tpl(name: &str, data: &str) -> String {
    format!("introspect::attribute_data({name}, {data})")
}

pub fn attribute_empty_tpl(name: &str) -> String {
    format!("introspect::attribute_empty({name})")
}

pub fn member_default_def_tpl(name: &str, attributes: &str, ty: &str) -> String {
    format!("introspect::member_default_def::<{ty}>({name}, {attributes})")
}

pub fn member_def_tpl(name: &str, attributes: &str, type_def: &str) -> String {
    format!("introspect::member_def({name}, {attributes}, {type_def})")
}

pub fn struct_def_tpl(name: &str, attributes: &str, members: &str) -> String {
    format!("introspect::struct_def({name}, {attributes}, {members})")
}

pub fn variant_default_def_tpl(selector: &str, name: &str, attributes: &str, ty: &str) -> String {
    format!("introspect::variant_default_def::<{ty}>({selector}, {name}, {attributes})")
}

pub fn variant_def_tpl(selector: &str, name: &str, attributes: &str, type_def: &str) -> String {
    format!("introspect::variant_def({selector}, {name}, {attributes}, {type_def})")
}

pub fn variant_unit_def_tpl(selector: &str, name: &str, attributes: &str) -> String {
    format!("introspect::variant_unit_def({selector}, {name}, {attributes})")
}

pub fn enum_def_tpl(name: &str, attributes: &str, variants: &str) -> String {
    format!("introspect::enum_def({name}, {attributes}, {variants})")
}

pub fn fixed_array_def_tpl(type_def: &str, size: u32) -> String {
    format!("introspect::FixedArrayDef {{ type_def: {type_def}, size: {size} }}))")
}
pub fn result_def_tpl(ok: &str, err: &str) -> String {
    format!("introspect::ResultDef {{ ok: {ok}, err: {err} }}",)
}

impl AsCairo for IAttribute {
    fn as_cairo(&self) -> String {
        match &self.data {
            Some(data) => attribute_data_tpl(&self.name.as_cairo_byte_array(), &data.as_cairo()),
            None => attribute_empty_tpl(&self.name.as_cairo_byte_array()),
        }
    }
}

impl AsCairo for MemberDef {
    fn as_cairo(&self) -> String {
        member_def_tpl(
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_cairo_span(),
            &self.type_def.as_cairo(),
        )
    }
}

impl AsCairo for StructDef {
    fn as_cairo(&self) -> String {
        struct_def_tpl(
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_cairo_span(),
            &self.members.as_cairo_span(),
        )
    }
}

impl AsCairo for EnumDef {
    fn as_cairo(&self) -> String {
        enum_def_tpl(
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_cairo_span(),
            &self.variants.as_cairo_span(),
        )
    }
}

impl AsCairoWith<Felt> for VariantDef {
    fn as_cairo_with(&self, selector: &Felt) -> String {
        let selector = selector.as_cairo();
        let name = self.name.as_cairo_byte_array();
        let attributes = self.attributes.as_cairo_span();
        match &self.type_def {
            TypeDef::None => variant_unit_def_tpl(&selector, &name, &attributes),
            _ => variant_def_tpl(&selector, &name, &attributes, &self.type_def.as_cairo()),
        }
    }
}

impl AsCairo for FixedArrayDef {
    fn as_cairo(&self) -> String {
        fixed_array_def_tpl(&self.type_def.as_cairo(), self.size)
    }
}

impl AsCairo for ResultDef {
    fn as_cairo(&self) -> String {
        result_def_tpl(&self.ok.as_cairo(), &self.err.as_cairo())
    }
}
