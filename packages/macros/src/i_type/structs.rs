use super::{IExtract, TypeDefVariant, TypeModTrait};
use crate::i_type::{ExtractAttributes, TypeModAndName};
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

impl ItemTrait for IStruct {}

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
