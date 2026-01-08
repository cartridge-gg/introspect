use super::{
    DefaultIExtractor, IExtract, IntrospectItemTrait, ToTypeDef, ToTypeDefs, TypeDefVariant,
};
use crate::params::GenericParams;
use crate::type_def::{
    enum_def_tpl, variant_def_tpl, variant_default_def_tpl, variant_unit_def_tpl,
};
use crate::utils::string_to_keccak_felt;
use crate::{AsCairo, AsCairoBytes, CollectionsAsCairo, Enum, IAttribute, Ty, Variant};
use starknet_types_core::felt::Felt;

pub struct IEnum {
    pub name: String,
    pub attributes: Vec<IAttribute>,
    pub generic_params: GenericParams,
    pub variants: Vec<IVariant>,
}

pub struct IVariant {
    pub selector: Felt,
    pub name: String,
    pub attributes: Vec<IAttribute>,
    pub ty: Option<Ty>,
    pub type_def: TypeDefVariant,
}

impl ToTypeDef for IVariant {
    fn to_type_def(&self) -> String {
        let selector = &self.selector.as_cairo();
        let name = &self.name.as_cairo_byte_array();
        let attributes = &self.attributes.as_cairo_span();
        match (&self.type_def, &self.ty) {
            (TypeDefVariant::Default, None) => variant_unit_def_tpl(selector, name, attributes),
            (TypeDefVariant::Default, Some(ty)) => {
                variant_default_def_tpl(&selector, name, attributes, &ty.as_cairo())
            }
            (TypeDefVariant::TypeDef(type_def), _) => {
                variant_def_tpl(selector, name, attributes, &type_def.as_cairo())
            }
            (TypeDefVariant::Fn(call), _) => variant_def_tpl(selector, name, attributes, &call),
        }
    }
}

impl ToTypeDef for IEnum {
    fn to_type_def(&self) -> String {
        enum_def_tpl(
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_cairo_span(),
            &self.variants.to_type_defs_span(),
        )
    }
}

impl<'db> IntrospectItemTrait for IEnum {
    type ModuleType = Enum;
    fn kind(&self) -> &str {
        "Enum"
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn generic_params(&self) -> &GenericParams {
        &self.generic_params
    }
    fn child_types(&self) -> Vec<Ty> {
        self.variants.iter().filter_map(|v| v.ty.clone()).collect()
    }
}

impl IExtract<IVariant, Variant> for DefaultIExtractor {
    fn iextract(&self, module: &Variant) -> IVariant {
        IVariant {
            selector: string_to_keccak_felt(&module.name),
            name: module.name.clone(),
            attributes: vec![],
            ty: module.ty.clone(),
            type_def: TypeDefVariant::Default,
        }
    }
}

impl IExtract<IEnum, Enum> for DefaultIExtractor {
    fn iextract(&self, module: &Enum) -> IEnum {
        IEnum {
            name: module.name.clone(),
            attributes: vec![],
            generic_params: module.generic_params.clone(),
            variants: self.iparses(&module.variants),
        }
    }
}
