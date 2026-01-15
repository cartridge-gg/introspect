use super::{
    AttributeParser, AttributeVariant, IExtract, IntrospectItemTrait, ToTypeDef, ToTypeDefs,
    TypeDefVariant,
};
use crate::i_type::ExtractTypeDef;
use crate::i_type::attribute::MacroAttributeTrait;
use crate::i_type::default::DefaultIExtractor;
use crate::type_def::{member_def_tpl, member_default_def_tpl, struct_def_tpl};
use crate::{
    AsCairo, AsCairoBytes, Attribute, CollectionsAsCairo, GenericParams, IAttribute,
    IntrospectError, ItemTrait, Member, Result, Struct, Ty,
};

pub struct IStruct {
    pub attributes: Vec<IAttribute>,
    pub name: String,
    pub generic_params: GenericParams,
    pub members: Vec<IMember>,
}

pub struct IMember {
    pub name: String,
    pub attributes: Vec<IAttribute>,
    pub ty: Ty,
    pub type_def: TypeDefVariant,
}

#[derive(Default)]
pub struct StructAttributes {}
impl MacroAttributeTrait for StructAttributes {}

impl ToTypeDef for IMember {
    fn to_type_def(&self) -> String {
        let name = &self.name.as_cairo_byte_array();
        let attributes = &self.attributes.as_cairo_span();
        match &self.type_def {
            TypeDefVariant::Default => {
                member_default_def_tpl(name, attributes, &self.ty.as_cairo())
            }
            TypeDefVariant::TypeDef(type_def) => {
                member_def_tpl(name, attributes, &type_def.as_cairo())
            }
            TypeDefVariant::Fn(call) => member_def_tpl(name, attributes, call),
        }
    }
}

impl ToTypeDef for IStruct {
    fn to_type_def(&self) -> String {
        struct_def_tpl(
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_cairo_span(),
            &self.members.to_type_defs_span(),
        )
    }
}

impl ItemTrait for IStruct {
    fn name(&self) -> &str {
        &self.name
    }
    fn generic_params(&self) -> &GenericParams {
        &self.generic_params
    }
}

impl IntrospectItemTrait for IStruct {
    type ModuleType = Struct;
    fn kind(&self) -> &str {
        "Struct"
    }
    fn child_types(&self) -> Vec<&Ty> {
        self.members.iter().map(|m| &m.ty).collect()
    }
}

impl IExtract<IMember> for DefaultIExtractor {
    type SyntaxType = Member;
    type Error = IntrospectError;
    fn iextract(&self, member: &mut Member) -> Result<IMember> {
        let (intro_attrs, macro_attrs) = self.parse_attributes(member)?;

        Ok(IMember {
            name: member.name.clone(),
            ty: member.ty.clone(),
            attributes: intro_attrs,
            type_def: self.extract_type_def(&member.ty, &macro_attrs)?,
        })
    }
}

impl IExtract<IStruct> for DefaultIExtractor {
    type SyntaxType = Struct;
    type Error = IntrospectError;
    fn iextract(&self, item: &mut Struct) -> Result<IStruct> {
        let (intro_attrs, _macro_attrs): (_, Vec<StructAttributes>) =
            self.parse_attributes(item)?;
        Ok(IStruct {
            attributes: intro_attrs,
            name: item.name.clone(),
            generic_params: item.generic_params.clone(),
            members: self.iextracts(&mut item.members)?,
        })
    }
}

impl AttributeParser<Struct, StructAttributes> for DefaultIExtractor {
    type Error = IntrospectError;
    fn parse_attribute(
        &self,
        _item: &mut Struct,
        attribute: Attribute,
    ) -> Result<Vec<AttributeVariant<StructAttributes>>> {
        Ok(vec![AttributeVariant::Cairo(attribute)])
    }
}
