use crate::{IdVariant, IntrospectResult, TableError, TableResult};
use introspect_macros::i_type::default::{TypeMod, TypeModMemberTrait};
use introspect_macros::i_type::{
    AttributeParser, AttributeVariant, DefaultIExtractor, IExtract, TypeDefVariant, TypeModTrait,
};
use introspect_macros::utils::string_to_keccak_felt;
use introspect_macros::{IAttribute, Member, Ty};
use introspect_rust_macros::macro_attributes;

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
            id: id.unwrap_or_else(|| IdVariant::Felt(string_to_keccak_felt(&member.name))),
            name: name.unwrap_or_else(|| member.name.clone()),
            member: member.name.clone(),
            ty: member.ty.clone(),
            attributes,
            type_def: type_mod.get_type_def(&member.ty)?,
        })
    }
}
