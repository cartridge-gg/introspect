use crate::templates::{column_id_const, member_impl_name_tpl};
use crate::{IdVariant, TableError, TableResult};
use introspect_macros::i_type::default::{TypeMod, TypeModMemberTrait};
use introspect_macros::i_type::{
    AttributeParser, AttributeVariant, DefaultIExtractor, IExtract, TypeDefVariant, TypeModTrait,
};
use introspect_macros::table::column::column_def_tpl;
use introspect_macros::type_def::CairoElementDefWith;
use introspect_macros::utils::string_to_keccak_hex;
use introspect_macros::{AsCairoBytes, CairoElementDefs, IAttribute, Member, Ty};
use introspect_rust_macros::macro_attributes;

#[derive(Debug, Clone)]
pub struct Column {
    pub id: IdVariant,
    pub name: String,
    pub member: String,
    pub attributes: Vec<IAttribute>,
    pub ty: Ty,
    pub type_def: TypeDefVariant,
}

pub enum ColumnName {
    Default,
    Custom(String),
}

#[derive(Default)]
#[macro_attributes]
pub struct ColumnAttributes {
    #[skip_accessors]
    type_mod: TypeMod,
    name: String,
    id: IdVariant,
}

impl ColumnAttributes {}

impl TypeModMemberTrait for ColumnAttributes {
    fn get_mut_type_mod(&mut self) -> &mut Option<TypeMod> {
        &mut self.type_mod
    }
}

impl AttributeParser<Member, ColumnAttributes> for DefaultIExtractor {
    type Error = TableError;
    fn parse_attribute(
        &self,
        _item: &mut Member,
        macro_attributes: &mut ColumnAttributes,
        attribute: introspect_macros::Attribute,
    ) -> TableResult<Vec<AttributeVariant>> {
        if let Some(r) = macro_attributes.extract_type_mod_return_empty(&attribute) {
            return r.map_err(From::from);
        }
        match attribute.name.as_str() {
            "name" => macro_attributes.set_name_return_empty(attribute.single_unnamed_arg()?),
            "id" => {
                macro_attributes.set_id_return_empty(attribute.single_unnamed_arg()?.try_into()?)
            }
            "index" => AttributeVariant::lazy_empty_i_attribute("index".to_string()),
            _ => attribute.into(),
        }
    }
}

impl IExtract<Column> for DefaultIExtractor {
    type SyntaxType = Member;
    type Error = TableError;
    fn iextract(&self, member: &mut Member) -> TableResult<Column> {
        let (ColumnAttributes { name, id, type_mod }, attributes): (ColumnAttributes, _) =
            self.parse_attributes(member)?;
        Ok(Column {
            id: id.unwrap_or_else(|| IdVariant::Felt(string_to_keccak_hex(&member.name))),
            name: name.unwrap_or_else(|| member.name.clone()),
            member: member.name.clone(),
            ty: member.ty.clone(),
            attributes,
            type_def: type_mod.get_type_def(&member.ty)?,
        })
    }
}

impl Column {
    pub fn id_const(&self) -> String {
        let id = match &self.id {
            IdVariant::Felt(felt_str) => felt_str,
            IdVariant::Const(const_str) => &format!("super::{const_str}"),
        };
        column_id_const(&self.name, id)
    }
    pub fn serialize_member_impl_name(&self, struct_name: &str) -> String {
        member_impl_name_tpl(struct_name, &self.member)
    }
}

impl CairoElementDefWith for Column {
    type Context = String;
    fn as_element_def_with(&self, i_path: &str, column_mod_name: &String) -> String {
        column_def_tpl(
            &format!("{column_mod_name}::{}", self.member),
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_element_defs_span(i_path),
            &self.type_def.type_def(&self.ty, i_path),
        )
    }
}
