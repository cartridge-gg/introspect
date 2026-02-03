use super::{IExtract, TypeDefVariant};
use crate::i_type::attribute::ExtractAttributes;
use crate::i_type::item::{IFieldTrait, IFieldsTrait};
use crate::i_type::{IAttributesTrait, TypeModAndName, TypeModTrait};
use crate::item::ItemTrait;
use crate::utils::string_to_keccak_felt;
use crate::{IAttribute, IntrospectError, IntrospectResult};
use cairo_syntax_parser::{Enum, GenericParam, GenericParamsTrait, NameTrait, Variant};
use starknet_types_core::felt::Felt;

pub struct IEnum {
    pub name: String,
    pub attributes: Vec<IAttribute>,
    pub generic_params: Option<Vec<GenericParam>>,
    pub variants: Vec<IVariant>,
}

pub struct IVariant {
    pub selector: Felt,
    pub field: String,
    pub name: String,
    pub attributes: Vec<IAttribute>,
    pub ty: Option<String>,
    pub type_def: TypeDefVariant,
}

impl NameTrait for IEnum {
    fn name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}

impl GenericParamsTrait for IEnum {
    fn generic_params(&self) -> &Option<Vec<GenericParam>> {
        &self.generic_params
    }
}

impl IAttributesTrait for IEnum {
    fn iattributes(&self) -> &[IAttribute] {
        &self.attributes
    }
}

impl IAttributesTrait for IVariant {
    fn iattributes(&self) -> &[IAttribute] {
        &self.attributes
    }
}

impl IFieldTrait for IVariant {
    fn field(&self) -> &str {
        &self.field
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn ty(&self) -> &str {
        self.ty.as_deref().unwrap_or("()")
    }
}

impl IFieldsTrait for IEnum {
    type Field = IVariant;
    fn fields(&self) -> &[Self::Field] {
        &self.variants
    }
}

impl ItemTrait for IEnum {
    fn type_selector(&self) -> &'static str {
        "'enum'"
    }
}

impl IExtract for IVariant {
    type SyntaxType = Variant;
    type Error = IntrospectError;
    fn iextract(variant: &mut Variant) -> IntrospectResult<IVariant> {
        let (TypeModAndName { type_mod, name }, attributes) = variant.extract_attributes()?;
        let ty = variant.type_clause.as_ref().map(|e| e.to_string());
        Ok(IVariant {
            selector: string_to_keccak_felt(&variant.name),
            name: name.unwrap_or_else(|| variant.name.clone()),
            field: variant.name.clone(),
            attributes,
            type_def: type_mod.get_type_def_option(&ty)?,
            ty,
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
