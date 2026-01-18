use std::collections::HashMap;

use crate::{AsCairo, AsCairoBytes, IAttribute};
use introspect_types::{
    ArrayDef, ByteArrayEDef, Bytes31EDef, CustomDef, EnumDef, Felt252DictDef, FixedArrayDef,
    MemberDef, NullableDef, OptionDef, RefDef, ResultDef, StructDef, TupleDef, TypeDef, VariantDef,
};
use starknet_types_core::felt::Felt;

pub mod templates;
pub use templates::{
    array_type_def_tpl, as_struct_type_def_tpl, as_type_def, as_unit_type_def, attribute_data_tpl,
    attribute_empty_tpl, collect_child_defs_tpl, enum_def_tpl, enum_type_def_tpl,
    felt252_dict_type_def_tpl, fixed_array_type_def_tpl, member_def_tpl, member_default_def_tpl,
    nullable_type_def_tpl, option_type_def_tpl, ref_type_def_tpl, result_type_def_tpl,
    struct_def_tpl, tuple_def_tpl, variant_def_tpl, variant_default_def_tpl, variant_unit_def_tpl,
};

pub trait CairoElementDef {
    fn as_element_def(&self, i_path: &str) -> String;
}

pub trait CairoElementDefs {
    fn as_element_defs(&self, i_path: &str) -> Vec<String>;
    fn as_element_defs_span(&self, i_path: &str) -> String {
        format!("[{}].span()", self.as_element_defs(i_path).join(","))
    }
}

pub trait CairoElementDefWith {
    type Context;
    fn as_element_def_with(&self, i_path: &str, context: &Self::Context) -> String;
}
impl<T: CairoElementDef> CairoElementDefs for Vec<T> {
    fn as_element_defs(&self, i_path: &str) -> Vec<String> {
        self.iter()
            .map(|element| element.as_element_def(i_path))
            .collect()
    }
}

pub trait CairoElementDefsWith {
    type Context;
    fn as_element_defs_with(&self, i_path: &str, context: &Self::Context) -> Vec<String>;
    fn as_element_defs_span_with(&self, i_path: &str, context: &Self::Context) -> String {
        format!(
            "[{}].span()",
            self.as_element_defs_with(i_path, context).join(",")
        )
    }
}

impl<T: CairoElementDefWith> CairoElementDefsWith for Vec<T> {
    type Context = T::Context;
    fn as_element_defs_with(&self, i_path: &str, context: &T::Context) -> Vec<String> {
        self.iter()
            .map(|element| element.as_element_def_with(i_path, context))
            .collect()
    }
}

impl<T: CairoElementDefWith> CairoElementDefs for HashMap<T::Context, T> {
    fn as_element_defs(&self, i_path: &str) -> Vec<String> {
        self.iter()
            .map(|(context, element)| element.as_element_def_with(i_path, context))
            .collect()
    }
}

pub trait CairoTypeDef {
    fn as_type_def(&self, i_path: &str) -> String;
}

pub trait CarioTypeDefs {
    fn as_type_defs(&self, i_path: &str) -> Vec<String>;
    fn as_type_def_span(&self, i_path: &str) -> String {
        format!("[{}].span()", self.as_type_defs(i_path).join(","))
    }
}

impl CairoTypeDef for TypeDef {
    fn as_type_def(&self, i_path: &str) -> String {
        self.as_element_def(i_path)
    }
}

impl CarioTypeDefs for Vec<TypeDef> {
    fn as_type_defs(&self, i_path: &str) -> Vec<String> {
        self.iter()
            .map(|element| element.as_type_def(i_path))
            .collect()
    }
}

impl CairoElementDef for TypeDef {
    fn as_element_def(&self, i_path: &str) -> String {
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
            | TypeDef::ByteArray
            | TypeDef::Utf8String => as_unit_type_def(i_path, self.item_name()),
            TypeDef::Bytes31E(Bytes31EDef { encoding })
            | TypeDef::ByteArrayE(ByteArrayEDef { encoding })
            | TypeDef::Custom(CustomDef { encoding }) => {
                as_type_def(i_path, self.item_name(), encoding.as_cairo_byte_array())
            }
            TypeDef::Struct(s) => s.as_type_def(i_path),
            TypeDef::Enum(e) => e.as_type_def(i_path),
            TypeDef::Tuple(e) => e.as_type_def(i_path),
            TypeDef::FixedArray(e) => e.as_type_def(i_path),
            TypeDef::Array(e) => e.as_type_def(i_path),
            TypeDef::Option(e) => e.as_type_def(i_path),
            TypeDef::Nullable(e) => e.as_type_def(i_path),
            TypeDef::Felt252Dict(e) => e.as_type_def(i_path),
            TypeDef::Result(e) => e.as_type_def(i_path),
            TypeDef::Ref(e) => e.as_type_def(i_path),
        }
    }
}

impl CairoElementDef for IAttribute {
    fn as_element_def(&self, i_path: &str) -> String {
        match &self.data {
            Some(data) => {
                attribute_data_tpl(i_path, &self.name.as_cairo_byte_array(), &data.as_cairo())
            }
            None => attribute_empty_tpl(i_path, &self.name.as_cairo_byte_array()),
        }
    }
}

impl CairoElementDef for MemberDef {
    fn as_element_def(&self, i_path: &str) -> String {
        member_def_tpl(
            i_path,
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_element_defs_span(i_path),
            &self.type_def.as_type_def(i_path),
        )
    }
}

impl CairoElementDef for StructDef {
    fn as_element_def(&self, i_path: &str) -> String {
        struct_def_tpl(
            i_path,
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_element_defs_span(i_path),
            &self.members.as_element_defs_span(i_path),
        )
    }
}

impl CairoElementDefWith for VariantDef {
    type Context = Felt;
    fn as_element_def_with(&self, i_path: &str, selector: &Felt) -> String {
        let selector = selector.as_cairo();
        let name = self.name.as_cairo_byte_array();
        let attributes = self.attributes.as_element_defs_span(i_path);
        match &self.type_def {
            TypeDef::None => variant_unit_def_tpl(i_path, &selector, &name, &attributes),
            _ => variant_def_tpl(
                i_path,
                &selector,
                &name,
                &attributes,
                &self.type_def.as_type_def(i_path),
            ),
        }
    }
}

impl CairoTypeDef for StructDef {
    fn as_type_def(&self, i_path: &str) -> String {
        as_struct_type_def_tpl(
            i_path,
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_element_defs_span(i_path),
            &self.members.as_element_defs_span(i_path),
        )
    }
}

impl CairoTypeDef for EnumDef {
    fn as_type_def(&self, i_path: &str) -> String {
        enum_type_def_tpl(
            i_path,
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_element_defs_span(i_path),
            &self.variants.as_element_defs_span(i_path),
        )
    }
}

impl CairoTypeDef for FixedArrayDef {
    fn as_type_def(&self, i_path: &str) -> String {
        fixed_array_type_def_tpl(i_path, &self.type_def.as_type_def(i_path), self.size)
    }
}

impl CairoTypeDef for ArrayDef {
    fn as_type_def(&self, i_path: &str) -> String {
        array_type_def_tpl(i_path, &self.type_def.as_type_def(i_path))
    }
}

impl CairoTypeDef for Felt252DictDef {
    fn as_type_def(&self, i_path: &str) -> String {
        felt252_dict_type_def_tpl(i_path, &self.type_def.as_type_def(i_path))
    }
}

impl CairoTypeDef for OptionDef {
    fn as_type_def(&self, i_path: &str) -> String {
        option_type_def_tpl(i_path, &self.type_def.as_type_def(i_path))
    }
}

impl CairoTypeDef for ResultDef {
    fn as_type_def(&self, i_path: &str) -> String {
        result_type_def_tpl(
            i_path,
            &self.ok.as_type_def(i_path),
            &self.err.as_type_def(i_path),
        )
    }
}

impl CairoTypeDef for NullableDef {
    fn as_type_def(&self, i_path: &str) -> String {
        nullable_type_def_tpl(i_path, &self.type_def.as_type_def(i_path))
    }
}

impl CairoTypeDef for TupleDef {
    fn as_type_def(&self, i_path: &str) -> String {
        tuple_def_tpl(i_path, &self.elements.as_type_def_span(i_path))
    }
}

impl CairoTypeDef for RefDef {
    fn as_type_def(&self, i_path: &str) -> String {
        ref_type_def_tpl(i_path, &self.id.as_cairo())
    }
}
