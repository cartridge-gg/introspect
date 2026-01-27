use crate::id::id_string_to_felt;
use crate::templates::{
    column_id_const, column_mod_name_tpl, member_impl_name_tpl, member_impl_tpl,
    serialize_member_call_tpl, serialize_struct_member_call_tpl,
};
use crate::{TableError, TableResult};
use introspect_macros::i_type::attribute::ExtractAttributes;
use introspect_macros::i_type::extraction::IExtractWith;
use introspect_macros::i_type::{
    AttributeParser, AttributeVariant, TypeDefVariant, TypeMod, TypeModMemberTrait, TypeModTrait,
};
use introspect_macros::table::column::column_def_tpl;
use introspect_macros::type_def::CairoElementDefWith;
use introspect_macros::utils::{Quoted, string_to_keccak_hex};
use introspect_macros::{
    AsCairo, AsCairoBytes, AttributesTrait, CairoElementDefs, IAttribute, IntrospectError, Member,
    Ty,
};
use introspect_rust_macros::macro_attributes;
#[derive(Debug, Clone)]
pub struct Column {
    pub id: String,
    pub key: bool,
    pub name: String,
    pub member: String,
    pub selector: String,
    pub attributes: Vec<IAttribute>,
    pub ty: Ty,
    pub type_def: TypeDefVariant,
    pub member_impl_name: String,
}

pub enum ColumnName {
    Default,
    Custom(String),
}

#[derive(Default)]
#[macro_attributes]
pub struct ColumnAttributes {
    #[skip]
    type_mod: TypeMod,
    name: String,
    id: String,
}

impl ColumnAttributes {}

impl TypeModMemberTrait for ColumnAttributes {
    fn get_mut_type_mod(&mut self) -> &mut TypeMod {
        &mut self.type_mod
    }
}

impl AttributeParser<Member> for ColumnAttributes {
    type Error = TableError;
    fn parse_attribute(
        &mut self,
        _module: &mut Member,
        attribute: introspect_macros::Attribute,
    ) -> TableResult<Vec<AttributeVariant>> {
        if let Some(r) = self.extract_type_mod_return_empty(&attribute) {
            return r.map_err(From::from);
        }
        match attribute.name.as_str() {
            "name" => self.set_name_return_empty(attribute.single_unnamed_arg()?),
            "id" => self.set_id_return_empty(id_string_to_felt(attribute.single_unnamed_arg()?)),
            "index" => AttributeVariant::lazy_empty_i_attribute("index".to_string()),
            _ => attribute.into(),
        }
    }
}

impl IExtractWith for Column {
    type SyntaxType = Member;
    type Error = TableError;
    type Context = String;
    fn iextract_with(member: &mut Member, struct_name: &String) -> TableResult<Column> {
        let (ColumnAttributes { name, id, type_mod }, attributes) = member.extract_attributes()?;
        let member_impl_name = member_impl_name_tpl(struct_name, &member.name);
        let selector = string_to_keccak_hex(&member.name);
        // println!("Column Member: {:?}", name);
        Ok(Column {
            id: id.unwrap_or_else(|| selector.clone()),
            name: name.unwrap_or_else(|| member.name.quoted()),
            member: member.name.clone(),
            key: member.has_name_only_attribute("key"),
            ty: member.ty.clone(),
            attributes,
            type_def: type_mod.get_type_def(&member.ty)?,
            member_impl_name,
            selector,
        })
    }
}

impl Column {
    pub fn id_const(&self) -> String {
        column_id_const(&self.member, &self.member_impl_name)
    }
    pub fn member_impl_name(&self, struct_name: &str) -> String {
        member_impl_name_tpl(struct_name, &self.member)
    }
    pub fn serialize_member_call<const SELF: bool>(&self) -> String {
        match SELF {
            true => serialize_struct_member_call_tpl(&self.member_impl_name, &self.member),
            false => serialize_member_call_tpl(&self.member_impl_name, &self.member),
        }
    }

    pub fn member_impl(&self, i_table_path: &str, struct_impl_name: &str) -> String {
        member_impl_tpl(
            i_table_path,
            &self.member_impl_name,
            struct_impl_name,
            &self.ty.as_cairo(),
            &self.id,
        )
    }
}

impl CairoElementDefWith for Column {
    type Context = String;
    fn as_element_def_with(&self, i_path: &str, column_mod_name: &String) -> String {
        column_def_tpl(
            i_path,
            &column_mod_name_tpl(column_mod_name, &self.member),
            &self.name,
            &self.attributes.as_element_defs_span(i_path),
            &self.type_def.type_def(&self.ty, i_path),
        )
    }
}
