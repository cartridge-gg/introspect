use std::mem;

use super::{
    DefaultIExtractor, IExtract, IntrospectItemTrait, ToTypeDef, ToTypeDefs, TypeDefVariant,
};
use crate::type_def::{member_def_tpl, member_default_def_tpl, struct_def_tpl};
use crate::{
    AsCairo, AsCairoBytes, CollectionsAsCairo, GenericParams, IAttribute, Member, Result, Struct,
    Ty,
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

impl ToTypeDef for IMember {
    fn to_type_def(&self) -> String {
        let name = &self.name.as_cairo_byte_array();
        let attributes = &self.attributes.as_cairo_span();
        match &self.type_def {
            TypeDefVariant::Default => member_default_def_tpl(name, attributes, &self.ty),
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

impl IntrospectItemTrait for IStruct {
    type ModuleType = Struct;
    fn kind(&self) -> &str {
        "Struct"
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn generic_params(&self) -> &GenericParams {
        &self.generic_params
    }
    fn child_types(&self) -> Vec<Ty> {
        self.members.iter().map(|m| m.ty.clone()).collect()
    }
}

impl IExtract<IMember, Member> for DefaultIExtractor {
    fn iextract(&self, module: &mut Member) -> Result<IMember> {
        let (attrs, iattrs, mattrs) = self.extract_attributes(mem::take(&mut module.attributes))?;
        module.attributes = attrs;
        Ok(IMember {
            name: module.name.clone(),
            ty: module.ty.clone(),
            attributes: iattrs,
            type_def: self.parse_type_def(&module.ty, &mattrs),
        })
    }
}

impl IExtract<IStruct, Struct> for DefaultIExtractor {
    fn iextract(&self, module: &mut Struct) -> Result<IStruct> {
        let (attrs, iattrs, mattrs) = self.extract_attributes(mem::take(&mut module.attributes))?;
        module.attributes = attrs;
        Ok(IStruct {
            attributes: iattrs,
            name: module.name.clone(),
            generic_params: module.generic_params.clone(),
            members: self.iextracts(&mut module.members)?,
        })
    }
}
