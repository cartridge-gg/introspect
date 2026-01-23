use super::{IExtract, IntrospectItemTrait, TypeDefVariant, TypeMod};
use crate::i_type::TypeModTrait;
use crate::i_type::attribute::ExtractAttributes;
use crate::params::GenericParams;
use crate::type_def::{
    enum_def_tpl, variant_def_tpl, variant_default_def_tpl, variant_unit_def_tpl,
};
use crate::utils::string_to_keccak_felt;
use crate::{
    AsCairo, AsCairoBytes, CairoElementDef, CairoElementDefs, CairoTypeDef, Enum, IAttribute,
    IntrospectError, IntrospectResult, ItemTrait, Ty, Variant,
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
    pub field: String,
    pub name: String,
    pub attributes: Vec<IAttribute>,
    pub ty: Option<Ty>,
    pub type_def: TypeDefVariant,
}

impl CairoElementDef for IVariant {
    fn as_element_def(&self, i_path: &str) -> String {
        let selector = &self.selector.as_cairo();
        let name = &self.name.as_cairo_byte_array();
        let attributes = &self.attributes.as_element_defs_span(i_path);
        match (&self.type_def, &self.ty) {
            (TypeDefVariant::Default, None) => {
                variant_unit_def_tpl(i_path, selector, name, attributes)
            }
            (TypeDefVariant::Default, Some(ty)) => {
                variant_default_def_tpl(i_path, &selector, name, attributes, &ty.as_cairo())
            }
            (TypeDefVariant::TypeDef(type_def), _) => variant_def_tpl(
                i_path,
                selector,
                name,
                attributes,
                &type_def.as_type_def(i_path),
            ),
            (TypeDefVariant::Fn(call), _) => {
                variant_def_tpl(i_path, selector, name, attributes, &call)
            }
        }
    }
}

impl CairoElementDef for IEnum {
    fn as_element_def(&self, i_path: &str) -> String {
        enum_def_tpl(
            i_path,
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_element_defs_span(i_path),
            &self.variants.as_element_defs_span(i_path),
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

impl IExtract for IVariant {
    type SyntaxType = Variant;
    type Error = IntrospectError;
    fn iextract(variant: &mut Variant) -> IntrospectResult<IVariant> {
        let (type_mod, attributes) = variant.extract_attributes::<TypeMod>()?;
        Ok(IVariant {
            selector: string_to_keccak_felt(&variant.name),
            name: variant.name.clone(),
            field: variant.name.clone(),
            attributes,
            ty: variant.ty.clone(),
            type_def: type_mod.get_type_def_option(&variant.ty)?,
        })
    }
}

impl IExtract for IEnum {
    type SyntaxType = Enum;
    type Error = IntrospectError;
    fn iextract(item: &mut Enum) -> IntrospectResult<IEnum> {
        Ok(IEnum {
            name: item.name.clone(),
            attributes: vec![],
            generic_params: item.generic_params.clone(),
            variants: IVariant::iextracts(&mut item.variants)?,
        })
    }
}
