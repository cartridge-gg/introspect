use crate::templates::{
    column_enum_name_tpl, column_id_const, columns_impl_tpl, columns_mod_name_tpl,
    i_table_impl_name_tpl, member_impl_name_tpl, member_impl_tpl, members_impl_name_tpl,
    primary_default_impl_tpl, primary_impl_tpl, required_no_id_impls_tpl,
    serialize_member_call_tpl, table_columns_impl_name_tpl, table_impl_name_tpl, table_impl_tpl,
    table_meta_impl_name_tpl, table_meta_tpl, table_primary_impl_name_tpl, table_schema_impl_tpl,
};
use crate::utils::felt_to_hex_string;
use crate::{Column, I_TABLE_PATH, IdVariant, TableError, TableResult};
use introspect_macros::i_type::attribute::MacroAttributeTrait;
use introspect_macros::i_type::default::{TypeMod, TypeModMemberTrait};
use introspect_macros::i_type::{
    AttributeParser, AttributeVariant, DefaultIExtractor, IExtract, ITys, TypeDefVariant,
};
use introspect_macros::table::column::ColumnDef;
use introspect_macros::table::primary::PrimaryTypeDefVariant;
use introspect_macros::table::{PrimaryDef, column};
use introspect_macros::utils::{string_to_keccak_felt, string_to_keccak_hex};
use introspect_macros::{
    AsCairo, AsCairoBytes, Attribute, AttributeArg, AttributeArgClause, AttributesTrait,
    CollectionsAsCairo, I_PATH, IAttribute, Member, Struct, Ty, type_def,
};
use introspect_rust_macros::macro_attributes;
use introspect_types::utils::ascii_str_to_felt;
use starknet_types_core::felt::Felt;

pub enum KeyType {
    None,
    Primary(PrimaryDef),
    Custom(Vec<Key>),
}

pub struct Key {
    pub name: String,
    pub ty: Ty,
}

pub struct Schema {
    pub name: String,
    pub key: KeyType,
    pub columns: Vec<Column>,
    pub members_impl_name: String,
    pub columns_name: String,
}

trait TableMemberTrait {
    fn is_primary(&self) -> bool;
    fn is_key(&self) -> bool;
    fn to_key(&self) -> Key;
}

impl TableMemberTrait for Member {
    fn is_key(&self) -> bool {
        self.has_name_only_attribute("key")
    }

    fn is_primary(&self) -> bool {
        self.ty.is_primary_type()
    }

    fn to_key(&self) -> Key {
        Key {
            name: self.name.clone(),
            ty: self.ty.clone(),
        }
    }
}

#[derive(Default)]
#[macro_attributes]
pub struct TableAttribute {
    name: String,
    id: IdVariant,
}

impl IExtract<Schema> for DefaultIExtractor {
    type SyntaxType = Struct;
    type Error = TableError;
    fn iextract(&self, module: &mut Self::SyntaxType) -> Result<Schema, Self::Error> {
        Ok(Schema {
            name: module.name.clone(),
            key: KeyType::None,
            columns: vec![],
            members_impl_name: members_impl_name_tpl(&module.name),
            columns_name: columns_mod_name_tpl(&module.name),
        })
    }
}

trait IExtractTable {
    fn extract_table(
        &self,
        struct_item: &mut Struct,
        table_id: Option<String>,
        table_name: Option<String>,
        impl_name: Option<String>,
        meta_impl: Option<String>,
        primary_impl: Option<String>,
        columns_impl: Option<String>,
    ) -> TableResult<Table>;

    fn extract_default_table(&self, struct_item: &mut Struct) -> TableResult<Table> {
        self.extract_table(struct_item, None, None, None, None, None, None)
    }

    fn extract_key_and_columns(
        &self,
        struct_item: &mut Struct,
    ) -> TableResult<(KeyType, Vec<ColumnDef>)>;
    fn extract_primary(&self, member: &mut Member) -> TableResult<PrimaryDef>;
}

fn get_keys(members: &[Member]) -> TableResult<Option<Vec<Key>>> {
    let mut position = 0;
    for (i, member) in members.iter().enumerate() {
        if member.is_key() {
            match position == i {
                true => position += 1,
                false => return Err(TableError::KeysNotFirst),
            }
        }
    }
    match position == 1 && members[0].is_primary() {
        true => Ok(None),
        false => Ok(Some(
            members[..position].iter().map(Member::to_key).collect(),
        )),
    }
}

impl IExtractTable for DefaultIExtractor {
    fn extract_key_and_columns(
        &self,
        struct_item: &mut Struct,
    ) -> TableResult<(KeyType, Vec<ColumnDef>)> {
        let keys = get_keys(&struct_item.members)?;
        let mut members = struct_item.members.iter_mut();
        let key_type = match keys {
            None => KeyType::Primary(self.extract_primary(&mut members.next().unwrap())?),
            Some(keys) if keys.is_empty() => KeyType::None,
            Some(keys) => KeyType::Custom(keys),
        };
        let columns = members
            .map(|m| self.extract_column(m))
            .collect::<TableResult<Vec<_>>>()?;
        Ok((key_type, columns))
    }
    fn extract_primary(&self, member: &mut Member) -> TableResult<PrimaryDef> {
        let (_intro_attrs, _macro_attrs): (_, Vec<PrimaryAttribute>) =
            self.parse_attributes(member)?;
        Ok(PrimaryDef {
            name: member.name.clone(),
            attributes: vec![],
            ty: member.ty.clone(),
            type_def: PrimaryTypeDefVariant::Default,
        })
    }

    fn extract_table(
        &self,
        struct_item: &mut Struct,
        table_id: Option<String>,
        table_name: Option<String>,
        impl_name: Option<String>,
        meta_impl: Option<String>,
        primary_impl: Option<String>,
        columns_impl: Option<String>,
    ) -> TableResult<Table> {
        let name = table_name.unwrap_or_else(|| struct_item.name.clone());
        let impl_name = impl_name.unwrap_or(name.clone());
        let id = table_id.unwrap_or_else(|| string_to_keccak_hex(&name));
        let (key, columns) = self.extract_key_and_columns(struct_item)?;
        let (i_attributes, macro_attributes) = self.parse_attributes(struct_item)?;
        let table_impl = table_impl_name_tpl(&impl_name);
        let i_table_impl = i_table_impl_name_tpl(&impl_name);
        let columns_mod = columns_mod_name_tpl(&impl_name);
        let column_enum = column_enum_name_tpl(&impl_name);
        let meta_impl = meta_impl.unwrap_or_else(|| table_meta_impl_name_tpl(&impl_name));
        let primary_impl = primary_impl.unwrap_or_else(|| table_primary_impl_name_tpl(&impl_name));
        let columns_impl = columns_impl.unwrap_or_else(|| table_columns_impl_name_tpl(&impl_name));
        Ok(Table {
            id,
            name,
            key,
            columns,
            attributes: i_attributes,
            struct_name: struct_item.name.clone(),
            impl_name,
            table_schema_impl: table_impl,
            i_table_impl,
            columns_mod,
            column_enum,
            meta_impl,
            primary_impl,
            columns_impl,
        })
    }
}

impl Table {
    pub fn id_hex(&self) -> String {
        felt_to_hex_string(&self.id)
    }
    fn generate_column_mods(&self) -> String {
        let column_ids = self
            .columns
            .iter()
            .map(|col| column_id_const(&col.member, &col.id))
            .collect::<Vec<_>>()
            .join("\n");
        format!("pub mod {}{{\n{column_ids}\n}}", self.columns_mod)
    }

    pub fn generate_meta_impl(&self) -> String {
        table_meta_tpl(
            &self.meta_impl,
            &self.id_hex(),
            &self.name.as_cairo_byte_array(),
            &self.attributes.as_cairo_span(),
        )
    }

    pub fn generate_primary_impl(&self) -> String {
        match &self.key {
            KeyType::Primary(key) => primary_impl_tpl(
                I_TABLE_PATH,
                &self.primary_impl,
                &self.name.as_cairo_byte_array(),
                &self.attributes.as_cairo_span(),
                &key.ty.as_cairo(),
            ),
            _ => primary_default_impl_tpl(I_TABLE_PATH, &self.primary_impl),
        }
    }

    pub fn generate_columns_impl(&self) -> String {
        let column_defs = self.columns.as_cairo_span();
        let child_defs = self
            .columns
            .iter()
            .map(|col| &col.ty)
            .collect::<Vec<_>>()
            .child_defs();
        columns_impl_tpl(&self.columns_impl, &column_defs, &child_defs)
    }

    pub fn generate_required_impls(&self) -> String {
        match self.key {
            KeyType::None => self.generate_required_no_id_impls(),
            _ => "".to_string(),
        }
    }

    pub fn generate_table_impl(&self) -> String {
        table_schema_impl_tpl(
            &self.table_schema_impl,
            &self.meta_impl,
            &self.primary_impl,
            &self.columns_impl,
        )
    }

    pub fn generate_i_table_impl(&self) -> String {
        table_impl_tpl(&self.table_schema_impl, &self.i_table_impl)
    }

    // pub fn generate_record_key_impl(&self) -> String {
    //     let keys = match &self.key {
    //         KeyType::Custom(keys) => keys,
    //         _ => panic!("Only custom keys are supported for record key impl"),
    //     };
    //     record_key_impl_tpl(
    //         &self.impl_name,
    //         &self.struct_name,
    //         &key_types_ss,
    //         &key_type,
    //         &key_expr,
    //     )
    // }

    pub fn generate_required_no_id_impls(&self) -> String {
        let mut serialize_member_calls = String::new();
        let mut member_impls = String::new();
        for col in &self.columns {
            let member_impl_name = member_impl_name_tpl(&self.impl_name, &col.member);
            member_impls.push_str(&member_impl_tpl(
                &self.table_schema_impl,
                &member_impl_name,
                &self.columns_mod,
                &col.member,
                &col.ty.as_cairo(),
            ));
            serialize_member_calls
                .push_str(&serialize_member_call_tpl(&member_impl_name, &col.member));
        }

        required_no_id_impls_tpl(
            &self.impl_name,
            &self.struct_name,
            &self.meta_impl,
            &self.primary_impl,
            &self.columns_impl,
            &member_impls,
            &serialize_member_calls,
        )
    }
}

// trait TableColumn {
//     fn generate_member_impl(&self, table_name: &str, columns_mod: &str) -> String;
// }

// impl TableColumn for ColumnDef {
//     fn generate_member_impl(&self, table_name: &str, columns_mod: &str) -> String {
//         member_impl_tpl(
//             I_TABLE_PATH,
//             table_name,
//             columns_mod,
//             &self.member,
//             &self.ty.as_cairo(),
//         )
//     }
// }
