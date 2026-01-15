use introspect_macros::I_PATH;

use crate::I_TABLE_PATH;

const COLUMN_ID_TPL: &str = "pub const {{member}}: felt252 = {{id}};";
const META_IMPL_TPL: &str = include_str!("../templates/meta_impl.cairo");
const SCHEMA_IMPLS_TPL: &str = include_str!("../templates/schema_impls.cairo");
const TABLE_IMPLS_TPL: &str = include_str!("../templates/table_impls.cairo");

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

pub fn members_impl_name_tpl(impl_name: &str) -> String {
    format!("{impl_name}Members")
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

pub fn schema_impls_tpl(
    struct_name: &str,
    primary_ty: &str,
    primary_name: &str,
    primary_attributes: &str,
    primary_type_def: &str,
    column_defs: &str,
    child_defs: &str,
    member_impls: &str,
    serialize_member_calls: &str,
) -> String {
    SCHEMA_IMPLS_TPL
        .replace("{{i_path}}", I_PATH)
        .replace("{{i_table_path}}", I_TABLE_PATH)
        .replace("{{struct_name}}", struct_name)
        .replace("{{primary_ty}}", primary_ty)
        .replace("{{primary_name}}", primary_name)
        .replace("{{primary_attributes}}", primary_attributes)
        .replace("{{primary_type_def}}", primary_type_def)
        .replace("{{column_defs}}", column_defs)
        .replace("{{child_defs}}", child_defs)
        .replace("{{member_impls}}", member_impls)
        .replace("{{serialize_member_calls}}", serialize_member_calls)
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

pub fn table_impls_tpl(
    impl_name: &str,
    struct_name: &str,
    meta_impl_name: &str,
    members_impl_name: &str,
) -> String {
    TABLE_IMPLS_TPL
        .replace("{{i_table_path}}", I_TABLE_PATH)
        .replace("{{impl_name}}", impl_name)
        .replace("{{struct_name}}", struct_name)
        .replace("{{meta_impl_name}}", meta_impl_name)
        .replace("{{members_impl_name}}", members_impl_name)
}

pub fn column_id_const(name: &str, id: &str) -> String {
    format!("pub const {name}: felt252 = {id};")
}

pub fn table_schema_impl_tpl(table_impl: &str, meta_impl: &str, members_impl: &str) -> String {
    format!("pub impl {table_impl} = {I_TABLE_PATH}::TableSchemaImpl<{meta_impl}, {members_impl}>;")
}

pub fn table_impl_tpl(table_impl: &str, i_table_impl: &str) -> String {
    format!("pub impl {i_table_impl} = {I_TABLE_PATH}::TableImpl<{table_impl}>;")
}

pub fn member_impl_name_tpl(struct_name: &str, member_name: &str) -> String {
    format!("{struct_name}_{member_name}_MemberImpl")
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
