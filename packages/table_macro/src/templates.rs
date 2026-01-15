use introspect_macros::I_PATH;

use crate::I_TABLE_PATH;

const TABLE_IMPL_NAME: &str = "{{impl_name}}Table";
const I_TABLE_IMPL_NAME: &str = "I{{impl_name}}Table";
const COLUMNS_MOD_NAME: &str = "{{impl_name}}Columns";
const COLUMN_ENUM_NAME: &str = "{{impl_name}}Column";
const META_IMPL_NAME: &str = "{{impl_name}}TableMeta";
const PRIMARY_IMPL_NAME: &str = "{{impl_name}}TablePrimary";
const COLUMNS_IMPL_NAME: &str = "{{impl_name}}TableColumns";
const NO_ID_IMPLS_TPL: &str = include_str!("../templates/no_id_impls.cairo");
const COLUMN_ID_TPL: &str = "pub const {{member}}: felt252 = {{id}};";
const META_IMPL_TPL: &str = include_str!("../templates/meta_impl.cairo");
const PRIMARY_IMPL_TPL: &str = include_str!("../templates/primary_impl.cairo");
const COLUMNS_IMPL_TPL: &str = include_str!("../templates/columns_impl.cairo");

const REQUIRED_IMPLS_TPL: &str = include_str!("../templates/required_imps.cairo");
const REQUIRED__NO_ID_IMPLS_TPL: &str = include_str!("../templates/required_imps_no_id.cairo");
// const RECORD_KEY_IMPL_TPL: &str = include_str!("../templates/record_key_impl.cairo");

pub fn table_impl_name_tpl(impl_name: &str) -> String {
    format!("{impl_name}Table")
}

pub fn i_table_impl_name_tpl(impl_name: &str) -> String {
    format!("I{impl_name}Table")
}

pub fn table_meta_impl_name_tpl(impl_name: &str) -> String {
    format!("{impl_name}TableMeta")
}

pub fn table_primary_impl_name_tpl(impl_name: &str) -> String {
    format!("{impl_name}TablePrimary")
}

pub fn table_columns_impl_name_tpl(impl_name: &str) -> String {
    format!("{impl_name}TableColumns")
}

pub fn columns_mod_name_tpl(impl_name: &str) -> String {
    format!("{impl_name}Columns")
}

pub fn column_enum_name_tpl(impl_name: &str) -> String {
    format!("{impl_name}Column")
}

pub fn table_meta_tpl(meta_impl: &str, id: &str, name: &str, attributes: &str) -> String {
    META_IMPL_TPL
        .replace("{{i_path}}", I_PATH)
        .replace("{{i_table_path}}", I_TABLE_PATH)
        .replace("{{meta_impl}}", meta_impl)
        .replace("{{table_id}}", id)
        .replace("{{table_name}}", name)
        .replace("{{attributes}}", attributes)
}

pub fn primary_impl_tpl(
    i_path: &str,
    primary_impl: &str,
    name: &str,
    attributes: &str,
    ty: &str,
) -> String {
    PRIMARY_IMPL_TPL
        .replace("{{i_path}}", i_path)
        .replace("{{primary_impl}}", primary_impl)
        .replace("{{name}}", name)
        .replace("{{attributes}}", attributes)
        .replace("{{ty}}", ty)
}

pub fn primary_default_impl_tpl(i_path: &str, primary_impl: &str) -> String {
    format!("impl {primary_impl} = {i_path}::table_primary::Default;\n")
}

pub fn columns_mod_tpl(mod_name: &str, columns_ids: &str) -> String {
    format!("pub mod {mod_name}{{\n{columns_ids}\n}}")
}

pub fn column_id_const(column_member: &str, id: &str) -> String {
    format!("pub const {column_member}: felt252 = {id};")
}

pub fn columns_impl_tpl(columns_impl: &str, column_defs: &str, child_defs: &str) -> String {
    COLUMNS_IMPL_TPL
        .replace("{{i_path}}", I_PATH)
        .replace("{{i_table_path}}", I_TABLE_PATH)
        .replace("{{columns_impl}}", columns_impl)
        .replace("{{column_defs}}", column_defs)
        .replace("{{child_defs}}", child_defs)
}

pub fn table_schema_impl_tpl(
    table_impl: &str,
    meta_impl: &str,
    primary_impl: &str,
    columns_impl: &str,
) -> String {
    format!(
        "pub impl {table_impl} = {I_TABLE_PATH}::TableSchemaImpl<{meta_impl}, {primary_impl}, {columns_impl}>;"
    )
}

pub fn table_impl_tpl(table_impl: &str, i_table_impl: &str) -> String {
    format!("pub impl {i_table_impl} = {I_TABLE_PATH}::TableImpl<{table_impl}>;")
}

pub fn member_impl_name_tpl(impl_name: &str, member_name: &str) -> String {
    format!("{impl_name}_{member_name}_MemberImpl")
}

pub fn serialize_member_call_tpl(member_impl_name: &str, member_name: &str) -> String {
    format!("{member_impl_name}::<T>::serialize_member(self.{member_name}, ref data);")
}

pub fn member_impl_tpl(
    member_impl: &str,
    struct_name: &str,
    columns_mod: &str,
    member_name: &str,
    type_str: &str,
) -> String {
    format!(
        "pub impl {member_impl}<impl T: introspect_table::TableSchema[Record: {struct_name}]> = {I_TABLE_PATH}::TableMemberImpl<T, {columns_mod}::{member_name}, {type_str}>;\n"
    )
}

// pub fn record_key_impl_tpl(
//     i_path: &str,
//     impl_name: &str,
//     struct_name: &str,
//     key_types_ss: &str,
//     key_type: &str,
//     key_expr: &str,
// ) -> String {
//     RECORD_KEY_IMPL_TPL
//         .replace("{{i_path}}", i_path)
//         .replace("{{impl_name}}", impl_name)
//         .replace("{{struct_name}}", struct_name)
//         .replace("{{key_types_ss}}", key_types_ss)
//         .replace("{{key_type}}", key_type)
//         .replace("{{key_expr}}", key_expr)
// }

pub fn record_id_felt252_impl_tpl(impl_name: &str, table_impl: &str) -> String {
    format!("impl {impl_name}RecordId = {I_TABLE_PATH}::RecordIdFelt252Impl<{table_impl}>;")
}

pub fn required_no_id_impls_tpl(
    impl_name: &str,
    struct_name: &str,
    meta_impl_name: &str,
    primary_impl_name: &str,
    columns_impl_name: &str,
    member_impl: &str,
    serialize_member_calls: &str,
) -> String {
    NO_ID_IMPLS_TPL
        .replace("{{i_table_path}}", I_TABLE_PATH)
        .replace("{{impl_name}}", impl_name)
        .replace("{{struct_name}}", struct_name)
        .replace("{{meta_impl_name}}", meta_impl_name)
        .replace("{{primary_impl_name}}", primary_impl_name)
        .replace("{{columns_impl_name}}", columns_impl_name)
        .replace("{{member_impl}}", member_impl)
        .replace("{{serialize_member_calls}}", serialize_member_calls)
}
