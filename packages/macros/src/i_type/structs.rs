use super::{IExtract, IntrospectItemTrait, TypeDefVariant, TypeModTrait};
use crate::i_type::{ExtractAttributes, TypeModAndName};
use crate::type_def::{member_def_tpl, member_default_def_tpl, struct_def_tpl};
use crate::{
    AsCairoBytes, CairoElementDef, CairoElementDefs, CairoFormat, CairoTypeDef, GenericParams,
    IAttribute, IntrospectError, IntrospectResult, ItemTrait, Member, Struct, Ty,
};

pub struct IStruct {
    pub attributes: Vec<IAttribute>,
    pub name: String,
    pub generic_params: GenericParams,
    pub members: Vec<IMember>,
}

pub struct IMember {
    pub field: String,
    pub name: String,
    pub attributes: Vec<IAttribute>,
    pub ty: Ty,
    pub type_def: TypeDefVariant,
}

impl CairoElementDef for IMember {
    fn as_element_def(&self, i_path: &str) -> String {
        let name = &self.name.as_cairo_byte_array();
        let attributes = &self.attributes.as_element_defs_span(i_path);
        match &self.type_def {
            TypeDefVariant::Default => {
                member_default_def_tpl(i_path, name, attributes, &self.ty.to_cairo())
            }
            TypeDefVariant::TypeDef(type_def) => {
                member_def_tpl(i_path, name, attributes, &type_def.as_type_def(i_path))
            }
            TypeDefVariant::Fn(call) => member_def_tpl(i_path, name, attributes, call),
        }
    }
}

impl CairoElementDef for IStruct {
    fn as_element_def(&self, i_path: &str) -> String {
        struct_def_tpl(
            i_path,
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_element_defs_span(i_path),
            &self.members.as_element_defs_span(i_path),
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

impl IExtract for IMember {
    type SyntaxType = Member;
    type Error = IntrospectError;
    fn iextract(member: &mut Member) -> IntrospectResult<IMember> {
        let (TypeModAndName { type_mod, name }, intro_attrs) = member.extract_attributes()?;
        Ok(IMember {
            name: name.unwrap_or_else(|| member.name.clone()),
            field: member.name.clone(),
            ty: member.ty.clone(),
            attributes: intro_attrs,
            type_def: type_mod.get_type_def(&member.ty)?,
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
