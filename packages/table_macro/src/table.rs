use crate::templates::{
    column_enum_name_tpl, column_id_const, columns_impl_tpl, columns_mod_name_tpl,
    i_table_impl_name_tpl, i_table_impl_tpl, member_impl_name_tpl, member_impl_tpl,
    primary_default_impl_tpl, primary_impl_tpl, record_key_impl_tpl, required_no_id_impls_tpl,
    serialize_member_call_tpl, table_columns_impl_name_tpl, table_impl_name_tpl, table_impl_tpl,
    table_meta_impl_name_tpl, table_meta_tpl, table_primary_impl_name_tpl,
};
use crate::utils::felt_to_hex_string;
use crate::{I_TABLE_PATH, Result};
use introspect_macros::i_type::extraction::{
    AttributeVariant, MacroAttribute, sort_attribute_variants,
};
use introspect_macros::i_type::{IExtract, ITys, TypeDefVariant};
use introspect_macros::table::PrimaryDef;
use introspect_macros::table::column::ColumnDef;
use introspect_macros::table::primary::PrimaryTypeDefVariant;
use introspect_macros::utils::string_to_keccak_felt;
use introspect_macros::{
    AsCairo, AsCairoBytes, Attribute, CollectionsAsCairo, I_PATH, IAttribute, Member, Struct, Ty,
};
use itertools::Itertools;
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

pub struct Table {
    pub id: Felt,
    pub name: String,
    pub key: KeyType,
    pub columns: Vec<ColumnDef>,
    pub attributes: Vec<IAttribute>,
    pub struct_name: String,
    pub impl_name: String,
    pub table_impl: String,
    pub i_table_impl: String,
    pub columns_mod: String,
    pub column_enum: String,
    pub meta_impl: String,
    pub primary_impl: String,
    pub columns_impl: String,
}

trait TableMemberTrait {
    fn is_primary(&self) -> bool;
    fn is_key(&self) -> bool;
    fn to_key(&self) -> Key;
}

impl TableMemberTrait for Member {
    fn is_key(&self) -> bool {
        self.attributes.iter().any(|attr| attr.name == "key")
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

pub struct DefaultTableExtractor;

trait IExtractTable {
    fn extract_table(
        &self,
        struct_item: &mut Struct,
        table_name: Option<String>,
        impl_name: Option<String>,
        meta_impl: Option<String>,
        primary_impl: Option<String>,
        columns_impl: Option<String>,
    ) -> Result<Table>;

    fn extract_default_table(&self, struct_item: &mut Struct) -> Result<Table> {
        self.extract_table(struct_item, None, None, None, None, None)
    }

    fn parse_table_attributes(
        &self,
        attributes: Vec<Attribute>,
    ) -> Result<(Vec<Attribute>, Vec<IAttribute>, Vec<MacroAttribute>)>;
    fn parse_key(&self, struct_item: &Struct) -> Result<KeyType>;
    fn parse_primary(&self, member: &Member) -> Result<PrimaryDef>;
}

impl DefaultTableExtractor {
    pub fn extract_table_attributes(
        &self,
        attributes: Vec<Attribute>,
    ) -> Result<(Vec<Attribute>, Vec<IAttribute>, Vec<MacroAttribute>)> {
        attributes
            .into_iter()
            .map(|a| self.parse_table_attribute(a))
            .collect::<Result<Vec<_>>>()
            .map(|v| sort_attribute_variants(v.into_iter().flatten().collect()))
    }

    fn parse_table_attribute(&self, _attr: Attribute) -> Result<Vec<AttributeVariant>> {}
}

impl IExtractTable for DefaultTableExtractor {
    fn parse_key(&self, struct_item: &Struct) -> Result<KeyType> {
        let keys: Vec<&Member> = struct_item.members.iter().filter(|m| m.is_key()).collect();
        match keys.len() {
            0 => Ok(KeyType::None),
            1 if keys[0].is_primary() => Ok(KeyType::Primary(self.parse_primary(keys[0])?)),
            _ => Ok(KeyType::Custom(
                keys.into_iter().map(Member::to_key).collect(),
            )),
        }
    }
    fn parse_primary(&self, member: &Member) -> Result<PrimaryDef> {
        Ok(PrimaryDef {
            name: member.name.clone(),
            attributes: vec![],
            ty: member.ty.clone(),
            type_def: PrimaryTypeDefVariant::Default,
        })
    }

    fn extract_column(&self, member: &mut Member) -> Result<ColumnDef> {
        let (attrs, iattrs, mattrs) = self.extract_attributes(mem::take(&mut member.attributes))?;
        member.attributes = attrs;
        Ok(ColumnDef {
            member: member.name.clone(),
            ty: member.ty.clone(),
            attributes: iattrs,
            type_def: self.parse_type_def(&member.ty, &mattrs),
            id: string_to_keccak_felt(&member.name),
        })
    }

    fn extract_table(
        &self,
        struct_item: &mut Struct,
        table_name: Option<String>,
        impl_name: Option<String>,
        meta_impl: Option<String>,
        primary_impl: Option<String>,
        columns_impl: Option<String>,
    ) -> Result<Table> {
        let name = table_name.unwrap_or_else(|| struct_item.name.clone());
        let impl_name = impl_name.unwrap_or(name.clone());
        let id = string_to_keccak_felt(&name);
        let key = self.parse_key(struct_item)?;
        let columns = struct_item.members.iter().map(Member::to_column).collect();
        let attributes = struct_item.iattributes();
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
            attributes,
            struct_name: struct_item.name.clone(),
            impl_name,
            table_impl,
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
            .map(|col| column_id_const(&col.member, &col.id.to_fixed_hex_string()))
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
        table_impl_tpl(
            &self.table_impl,
            &self.meta_impl,
            &self.primary_impl,
            &self.columns_impl,
        )
    }

    pub fn generate_i_table_impl(&self) -> String {
        i_table_impl_tpl(&self.table_impl, &self.i_table_impl)
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
                &self.table_impl,
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
