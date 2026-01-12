use crate::{AsCairo, AsCairoBytes, AsCairoWith, CollectionsAsCairo, I_PATH, IAttribute};
use introspect_types::{
    ByteArrayEDef, CustomDef, ElementDef, EnumDef, FixedArrayDef, MemberDef, ResultDef, StructDef,
    TupleDef, TypeDef, VariantDef,
};
use starknet_types_core::felt::Felt;

pub trait CairoElementDef {
    fn as_element_def(&self) -> String;
}

pub trait CairoElementDefWith<C> {
    fn as_element_def_with(&self, context: &C) -> String;
}

impl<T: ElementDef + CairoElementDef> AsCairo for T {
    fn as_cairo(&self) -> String {
        self.as_element_def()
    }
}

impl<T: ElementDef + CairoElementDefWith<C>, C> AsCairoWith<C> for T {
    fn as_cairo_with(&self, context: &C) -> String {
        self.as_element_def_with(context)
    }
}

pub trait CairoTypeDef {
    fn as_type_def(&self) -> String;
}

impl CairoTypeDef for TypeDef {
    fn as_type_def(&self) -> String {
        self.as_element_def()
    }
}

impl CairoElementDef for TypeDef {
    fn as_element_def(&self) -> String {
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
            | TypeDef::Utf8String(_) => as_unit_type_def(self.item_name()),
            TypeDef::Bytes31E(encoding)
            | TypeDef::ByteArrayE(ByteArrayEDef { encoding, mode: _ })
            | TypeDef::Custom(CustomDef { encoding }) => {
                as_type_def(self.item_name(), encoding.as_cairo_byte_array())
            }
            TypeDef::Struct(s) => s.as_type_def(),
            TypeDef::Enum(e) => e.as_type_def(),
            TypeDef::Tuple(e) => as_type_def("Tuple", e.as_cairo_span()),
            TypeDef::FixedArray(e) => e.as_type_def(),
            TypeDef::Array(e) => as_type_def_boxed("Array", e.as_type_def()),
            TypeDef::Option(e) => as_type_def_boxed("Option", e.as_type_def()),
            TypeDef::Nullable(e) => as_type_def_boxed("Nullable", e.as_type_def()),
            TypeDef::Felt252Dict(e) => as_type_def_boxed("Felt252Dict", e.as_type_def()),
            TypeDef::Result(e) => as_type_def_boxed("Result", e.as_cairo()),
            TypeDef::Ref(e) => as_type_def("Ref", e.as_cairo()),
        }
    }
}

pub fn as_type_def(variant: &str, type_def: String) -> String {
    format!("{I_PATH}::{}({})", variant, type_def)
}

pub fn as_unit_type_def(variant: &str) -> String {
    format!("{I_PATH}::TypeDef::{}", variant)
}

pub fn as_type_def_boxed(variant: &str, type_def: String) -> String {
    format!(
        "{I_PATH}::TypeDef::{}(BoxTrait::new({}))",
        variant, type_def
    )
}

pub fn attribute_data_tpl(name: &str, data: &str) -> String {
    format!("{I_PATH}::attribute_data({name}, {data})")
}

pub fn attribute_empty_tpl(name: &str) -> String {
    format!("{I_PATH}::attribute_empty({name})")
}

pub fn member_default_def_tpl(name: &str, attributes: &str, ty: &str) -> String {
    format!("{I_PATH}::member_default_def::<{ty}>({name}, {attributes})")
}

pub fn member_def_tpl(name: &str, attributes: &str, type_def: &str) -> String {
    format!("{I_PATH}::member_def({name}, {attributes}, {type_def})")
}

pub fn struct_def_tpl(name: &str, attributes: &str, members: &str) -> String {
    format!("{I_PATH}::struct_def({name}, {attributes}, {members})")
}

pub fn as_struct_type_def_tpl(name: &str, attributes: &str, members: &str) -> String {
    format!("{I_PATH}::struct_type_def({name}, {attributes}, {members})",)
}

pub fn variant_default_def_tpl(selector: &str, name: &str, attributes: &str, ty: &str) -> String {
    format!("{I_PATH}::variant_default_def::<{ty}>({selector}, {name}, {attributes})")
}

pub fn variant_def_tpl(selector: &str, name: &str, attributes: &str, type_def: &str) -> String {
    format!("{I_PATH}::variant_def({selector}, {name}, {attributes}, {type_def})")
}

pub fn variant_unit_def_tpl(selector: &str, name: &str, attributes: &str) -> String {
    format!("{I_PATH}::variant_unit_def({selector}, {name}, {attributes})")
}

pub fn enum_def_tpl(name: &str, attributes: &str, variants: &str) -> String {
    format!("{I_PATH}::enum_def({name}, {attributes}, {variants})")
}

pub fn enum_type_def_tpl(name: &str, attributes: &str, variants: &str) -> String {
    format!("{I_PATH}::enum_type_def({name}, {attributes}, {variants})",)
}

pub fn fixed_array_def_tpl(type_def: &str, size: u32) -> String {
    format!("{I_PATH}::fixed_array_def({type_def}, {size})")
}

pub fn fixed_array_type_def_tpl(type_def: &str, size: u32) -> String {
    format!("{I_PATH}::fixed_array_type_def({type_def}, {size})")
}

pub fn result_def_tpl(ok: &str, err: &str) -> String {
    format!("{I_PATH}::result_def({ok}, {err})")
}

pub fn result_type_def_tpl(ok: &str, err: &str) -> String {
    format!("{I_PATH}::result_type_def({ok}, {err})")
}

pub fn child_defs_tpl(type_str: &str) -> String {
    format!("{I_PATH}::child_defs::<{}>()", type_str)
}

impl AsCairo for IAttribute {
    fn as_cairo(&self) -> String {
        match &self.data {
            Some(data) => attribute_data_tpl(&self.name.as_cairo_byte_array(), &data.as_cairo()),
            None => attribute_empty_tpl(&self.name.as_cairo_byte_array()),
        }
    }
}

impl CairoElementDef for MemberDef {
    fn as_element_def(&self) -> String {
        member_def_tpl(
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_cairo_span(),
            &self.type_def.as_type_def(),
        )
    }
}

impl CairoElementDef for StructDef {
    fn as_element_def(&self) -> String {
        struct_def_tpl(
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_cairo_span(),
            &self.members.as_cairo_span(),
        )
    }
}

impl CairoElementDefWith<Felt> for VariantDef {
    fn as_element_def_with(&self, selector: &Felt) -> String {
        let selector = selector.as_cairo();
        let name = self.name.as_cairo_byte_array();
        let attributes = self.attributes.as_cairo_span();
        match &self.type_def {
            TypeDef::None => variant_unit_def_tpl(&selector, &name, &attributes),
            _ => variant_def_tpl(&selector, &name, &attributes, &self.type_def.as_type_def()),
        }
    }
}

impl CairoElementDef for FixedArrayDef {
    fn as_element_def(&self) -> String {
        fixed_array_def_tpl(&self.type_def.as_type_def(), self.size)
    }
}

impl CairoElementDef for ResultDef {
    fn as_element_def(&self) -> String {
        result_def_tpl(&self.ok.as_type_def(), &self.err.as_type_def())
    }
}

impl CairoTypeDef for StructDef {
    fn as_type_def(&self) -> String {
        as_struct_type_def_tpl(
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_cairo_span(),
            &self.members.as_cairo_span(),
        )
    }
}

impl CairoTypeDef for EnumDef {
    fn as_type_def(&self) -> String {
        enum_type_def_tpl(
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_cairo_span(),
            &self.variants.as_cairo_span(),
        )
    }
}

impl CairoTypeDef for ResultDef {
    fn as_type_def(&self) -> String {
        result_type_def_tpl(&self.ok.as_type_def(), &self.err.as_type_def())
    }
}

impl CairoTypeDef for FixedArrayDef {
    fn as_type_def(&self) -> String {
        fixed_array_type_def_tpl(&self.type_def.as_type_def(), self.size)
    }
}

impl CairoTypeDef for TupleDef {
    fn as_type_def(&self) -> String {
        format!(
            "{I_PATH}::TypeDef::Tuple({})",
            self.elements.as_cairo_span()
        )
    }
}
