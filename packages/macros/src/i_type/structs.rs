use super::{IExtract, TypeDefVariant, TypeModTrait};
use crate::i_type::item::{IFieldTrait, IFieldsTrait};
use crate::i_type::{ExtractAttributes, IAttributesTrait, TypeModAndName};
use crate::item::ItemTrait;
use crate::{IAttribute, IntrospectError, IntrospectResult};
use cairo_syntax_parser::{GenericParam, GenericParamsTrait, Member, NameTrait, Struct};

pub struct IStruct {
    pub attributes: Vec<IAttribute>,
    pub name: String,
    pub generic_params: Option<Vec<GenericParam>>,
    pub members: Vec<IMember>,
}

impl NameTrait for IStruct {
    fn name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}
impl GenericParamsTrait for IStruct {
    fn generic_params(&self) -> &Option<Vec<GenericParam>> {
        &self.generic_params
    }
}

impl IAttributesTrait for IStruct {
    fn iattributes(&self) -> &[IAttribute] {
        &self.attributes
    }
}

impl IAttributesTrait for IMember {
    fn iattributes(&self) -> &[IAttribute] {
        &self.attributes
    }
}

impl IFieldTrait for IMember {
    fn field(&self) -> &str {
        &self.field
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn ty(&self) -> &str {
        &self.ty
    }
}

impl IFieldsTrait for IStruct {
    type Field = IMember;
    fn fields(&self) -> &[Self::Field] {
        &self.members
    }
}

impl ItemTrait for IStruct {
    fn type_selector(&self) -> &'static str {
        "'struct'"
    }
}

pub struct IMember {
    pub field: String,
    pub name: String,
    pub attributes: Vec<IAttribute>,
    pub ty: String,
    pub type_def: TypeDefVariant,
}

impl IExtract for IMember {
    type SyntaxType = Member;
    type Error = IntrospectError;
    fn iextract(member: &mut Member) -> IntrospectResult<IMember> {
        let (TypeModAndName { type_mod, name }, intro_attrs) = member.extract_attributes()?;
        let ty = member.ty.to_string();
        Ok(IMember {
            name: name.unwrap_or_else(|| member.name.clone()),
            field: member.name.clone(),
            attributes: intro_attrs,
            type_def: type_mod.get_type_def(&ty)?,
            ty: ty,
        })
    }
}

impl IExtract for IStruct {
    type SyntaxType = Struct;
    type Error = IntrospectError;
    fn iextract(item: &mut Struct) -> IntrospectResult<IStruct> {
        Ok(IStruct {
            attributes: vec![],
            name: item.name.clone(),
            generic_params: item.generic_params.clone(),
            members: IMember::iextracts(&mut item.members)?,
        })
    }
}
