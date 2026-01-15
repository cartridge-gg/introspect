use super::{
    DefaultIExtractor, IExtract, IntrospectItemTrait, ToTypeDef, ToTypeDefs, TypeDefVariant,
};
use crate::i_type::attribute::MacroAttributeTrait;
use crate::i_type::{AttributeParser, AttributeVariant, ExtractTypeDef};
use crate::params::GenericParams;
use crate::type_def::{
    enum_def_tpl, variant_def_tpl, variant_default_def_tpl, variant_unit_def_tpl,
};
use crate::utils::string_to_keccak_felt;
use crate::{
    AsCairo, AsCairoBytes, Attribute, CollectionsAsCairo, Enum, IAttribute, IntrospectError,
    ItemTrait, Result, Ty, Variant,
};
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

pub enum EnumMacroAttribute {}

impl MacroAttributeTrait for EnumMacroAttribute {}

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

impl ItemTrait for IEnum {
    fn name(&self) -> &str {
        &self.name
    }
    fn generic_params(&self) -> &GenericParams {
        &self.generic_params
    }
}

impl<'db> IntrospectItemTrait for IEnum {
    type ModuleType = Enum;
    fn kind(&self) -> &str {
        "Enum"
    }
    fn child_types(&self) -> Vec<&Ty> {
        self.variants.iter().filter_map(|v| v.ty.as_ref()).collect()
    }
}

impl IExtract<IVariant> for DefaultIExtractor {
    type SyntaxType = Variant;
    type Error = IntrospectError;
    fn iextract(&self, variant: &mut Variant) -> Result<IVariant> {
        let (iattrs, mattrs) = self.parse_attributes(variant)?;
        Ok(IVariant {
            selector: string_to_keccak_felt(&variant.name),
            name: variant.name.clone(),
            attributes: iattrs,
            ty: variant.ty.clone(),
            type_def: self.extract_option_type_def(&variant.ty, &mattrs)?,
        })
    }
}

impl IExtract<IEnum> for DefaultIExtractor {
    type SyntaxType = Enum;
    type Error = IntrospectError;
    fn iextract(&self, item: &mut Enum) -> Result<IEnum> {
        let (intro_attrs, _macro_attrs): (_, Vec<EnumMacroAttribute>) =
            self.parse_attributes(item)?;
        Ok(IEnum {
            name: item.name.clone(),
            attributes: intro_attrs,
            generic_params: item.generic_params.clone(),
            variants: self.iextracts(&mut item.variants)?,
        })
    }
}

impl AttributeParser<Enum, EnumMacroAttribute> for DefaultIExtractor {
    type Error = IntrospectError;
    fn parse_attribute(
        &self,
        _item: &mut Enum,
        attribute: Attribute,
    ) -> Result<Vec<AttributeVariant<EnumMacroAttribute>>> {
        attribute.into()
    }
}
