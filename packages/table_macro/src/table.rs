use crate::utils::felt_to_hex_string;
use indent::indent_by;
use introspect_macros::attribute::{IAttribute, iattributes_to_span};
use introspect_macros::child_defs::combined_type_child_defs;
use introspect_macros::column::make_column_def;
use introspect_macros::introsepct_items::type_child_defs;
use introspect_macros::utils::{spanify, string_to_keccak_felt};
use introspect_macros::{Member, Struct};
use starknet_types_core::felt::Felt;

const TABLE_IMPL_NAME: &str = "{{impl_name}}Table";
const COLUMNS_MOD_NAME: &str = "{{impl_name}}Columns";
const COLUMN_ENUM_NAME: &str = "{{impl_name}}Column";
const META_IMPL_NAME: &str = "{{impl_name}}TableMeta";
const PRIMARY_IMPL_NAME: &str = "{{impl_name}}TablePrimary";
const COLUMNS_IMPL_NAME: &str = "{{impl_name}}TableColumns";
const COLUMNS_MOD_TPL: &str = include_str!("./templates/columns_mod.cairo");
const COLUMN_ID_TPL: &str = "pub const {{member}}: felt252 = {{id}};";
const META_IMPL_TPL: &str = include_str!("./templates/meta_impl.cairo");
const PRIMARY_IMPL_TPL: &str = include_str!("./templates/primary_impl.cairo");
const COLUMNS_IMPL_TPL: &str = include_str!("./templates/columns_impl.cairo");
pub enum KeyType {
    None,
    Primary(Key),
    Custom(Vec<Key>),
}

pub struct Key {
    pub name: String,
    pub c_type: String,
}

pub struct Column {
    pub id: Felt,
    pub attributes: Vec<IAttribute>,
    pub name: String,
    pub member: String,
    pub c_type: String,
}

pub struct Primary {
    pub name: String,
    pub c_type: String,
}

pub struct Table {
    pub id: Felt,
    pub name: String,
    pub key: KeyType,
    pub columns: Vec<Column>,
    pub attributes: Vec<IAttribute>,
    pub table_impl: String,
    pub columns_mod: String,
    pub column_enum: String,
    pub meta_impl: String,
    pub primary_impl: String,
    pub columns_impl: String,
}

trait TableMemberTrait {
    fn to_key(&self) -> Key;
    fn is_primary(&self) -> bool;
    fn is_key(&self) -> bool;
    fn to_column(&self) -> Column;
}

impl Column {
    pub fn id_hex(&self) -> String {
        felt_to_hex_string(&self.id)
    }

    pub fn generate_column_def(&self) -> String {
        make_column_def(&self.id_hex(), &self.name, &self.c_type, &self.attributes)
    }
}

pub trait Columns {
    fn generate_column_defs_span(&self) -> String;
    fn types(&self) -> Vec<String>;
    fn generate_child_defs_span(&self) -> String {}
}

impl Columns for [Column] {
    fn generate_column_defs_span(self: &[Column]) -> String {
        spanify(
            self.iter()
                .map(Column::generate_column_def)
                .collect::<Vec<_>>(),
        )
    }

    fn types(&self) -> Vec<String> {
        self.iter().map(|c| c.c_type.clone()).collect()
    }

    fn generate_child_defs_span(&self) -> String {
        combined_type_child_defs(self.types())
    }
}

impl<'db> TableMemberTrait for Member<'db> {
    fn to_key(&self) -> Key {
        Key {
            name: self.name.clone(),
            c_type: self.ty.clone(),
        }
    }

    fn is_primary(&self) -> bool {
        matches!(
            self.ty.as_str(),
            "felt252"
                | "bool"
                | "u8"
                | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "i8"
                | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "bytes31"
                | "ClassHash"
                | "ContractAddress"
                | "EthAddress"
                | "StorageAddress"
                | "StorageBaseAddress"
        )
    }

    fn is_key(&self) -> bool {
        self.attributes.iter().any(|attr| attr.name == "key")
    }

    fn to_column(&self) -> Column {
        Column {
            id: string_to_keccak_felt(&self.name),
            attributes: self.iattributes(),
            name: self.name.clone(),
            member: self.name.clone(),
            c_type: self.ty.clone(),
        }
    }
}

fn get_key_type<'db>(members: &[Member<'db>]) -> KeyType {
    let keys: Vec<&Member> = members.into_iter().filter(|m| m.is_key()).collect();
    match keys.len() {
        0 => KeyType::None,
        1 => {
            if keys[0].is_primary() {
                KeyType::Primary(keys[0].to_key())
            } else {
                KeyType::Custom(vec![keys[0].to_key()])
            }
        }
        _ => KeyType::Custom(keys.into_iter().map(Member::to_key).collect()),
    }
}

impl Table {
    pub fn new<'db>(
        c_struct: &Struct<'db>,
        table_name: Option<String>,
        impl_name: Option<String>,
        meta_impl: Option<String>,
        primary_impl: Option<String>,
        columns_impl: Option<String>,
    ) -> Self {
        let name = table_name.unwrap_or_else(|| c_struct.name.clone());
        let impl_name = impl_name.unwrap_or(name.clone());
        let id = string_to_keccak_felt(&name);
        let key = get_key_type(&c_struct.members);
        let columns = c_struct.members.iter().map(Member::to_column).collect();

        let attributes = c_struct.iattributes();
        let table_impl = TABLE_IMPL_NAME.replace("{{impl_name}}", &impl_name);
        let columns_mod = COLUMNS_MOD_NAME.replace("{{impl_name}}", &impl_name);
        let column_enum = COLUMN_ENUM_NAME.replace("{{impl_name}}", &impl_name);
        let meta_impl =
            meta_impl.unwrap_or_else(|| META_IMPL_NAME.replace("{{impl_name}}", &impl_name));
        let primary_impl =
            primary_impl.unwrap_or_else(|| PRIMARY_IMPL_NAME.replace("{{impl_name}}", &impl_name));
        let columns_impl =
            columns_impl.unwrap_or_else(|| COLUMNS_IMPL_NAME.replace("{{impl_name}}", &impl_name));
        Table {
            id,
            name,
            key,
            columns,
            attributes,
            table_impl,
            columns_mod,
            column_enum,
            meta_impl,
            primary_impl,
            columns_impl,
        }
    }

    pub fn id_hex(&self) -> String {
        felt_to_hex_string(&self.id)
    }
    pub fn generate_columns_mod(&self) -> String {
        fn make_id(col: &Column) -> String {
            COLUMN_ID_TPL
                .replace("{{member}}", &col.member)
                .replace("{{id}}", &col.id_hex())
        }
        let ids = self
            .columns
            .iter()
            .map(make_id)
            .collect::<Vec<_>>()
            .join("\n");
        COLUMNS_MOD_TPL
            .replace("{{columns_mod}}", &self.columns_mod)
            .replace("{{ids}}", &indent_by(4, &ids))
    }

    pub fn generate_primary_impl(&self) -> String {
        match &self.key {
            _ => {}
        }
    }

    pub fn generate_meta_impl(&self) -> String {
        META_IMPL_TPL
            .replace("{{meta_impl}}", &self.meta_impl)
            .replace("{{table_id}}", &self.id_hex())
            .replace("{{table_name}}", &self.name)
            .replace(
                "{{attributes}}",
                &indent_by(4, &iattributes_to_span(&self.attributes)),
            )
    }

    pub fn generate_columns_impl(&self) -> String {
        COLUMNS_IMPL_TPL
            .replace("{{columns_impl}}", &self.columns_impl)
            .replace(
                "{{column_defs}}",
                &indent_by(4, &generate_column_defs_span(&self.columns)),
            )
            .replace("{{child_defs}}")
    }
}
