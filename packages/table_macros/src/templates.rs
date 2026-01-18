const COLUMN_ID_TPL: &str = "pub const {{member}}: felt252 = {{id}};";
const SCHEMA_IMPLS_TPL: &str = include_str!("../templates/structure_impls.cairo");
const TABLE_IMPL_TPL: &str = include_str!("../templates/table_impl.cairo");

const RECORD_ID_IMPL_TPL: &str = include_str!("../templates/record_id_impl.cairo");
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

pub fn struct_impl_name_tpl(impl_name: &str) -> String {
    format!("{impl_name}Structure")
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

pub fn structure_impls_tpl(
    i_path: &str,
    struct_name: &str,
    struct_impl_name: &str,
    primary_ty: &str,
    primary_name: &str,
    primary_attributes: &str,
    primary_type_def: &str,
    columns_mod_name: &str,
    column_id_consts: &str,
    column_defs: &str,
    child_defs: &str,
    member_impls: &str,
    serialize_member_calls: &str,
) -> String {
    SCHEMA_IMPLS_TPL
        .replace("{{i_path}}", i_path)
        .replace("{{struct_name}}", struct_name)
        .replace("{{struct_impl_name}}", struct_impl_name)
        .replace("{{primary_ty}}", primary_ty)
        .replace("{{primary_name}}", primary_name)
        .replace("{{primary_attributes}}", primary_attributes)
        .replace("{{primary_type_def}}", primary_type_def)
        .replace("{{columns_mod_name}}", columns_mod_name)
        .replace("{{column_consts}}", column_id_consts)
        .replace("{{column_defs}}", column_defs)
        .replace("{{child_defs}}", child_defs)
        .replace("{{member_impls}}", member_impls)
        .replace("{{serialize_member_calls}}", serialize_member_calls)
}

pub fn column_id_const(name: &str, id: &str) -> String {
    format!("pub const {name}: felt252 = {id};")
}

pub fn table_impl_tpl(
    i_path: &str,
    i_table_impl: &str,
    struct_impl_name: &str,
    meta_impl_name: &str,
) -> String {
    TABLE_IMPL_TPL
        .replace("{{i_path}}", i_path)
        .replace("{{i_table_impl}}", i_table_impl)
        .replace("{{struct_impl_name}}", struct_impl_name)
        .replace("{{meta_impl_name}}", meta_impl_name)
}

pub fn member_impl_name_tpl(struct_name: &str, member_name: &str) -> String {
    format!("{struct_name}_{member_name}_MemberImpl")
}

pub fn serialize_member_call_tpl(member_impl_name: &str, member_name: &str) -> String {
    format!("{member_impl_name}::serialize_member(self.{member_name}, ref data);")
}

pub fn member_impl_tpl(
    i_path: &str,
    member_impl_name: &str,
    struct_impl_name: &str,
    columns_mod: &str,
    member_name: &str,
    type_str: &str,
) -> String {
    format!(
        "pub impl {member_impl_name} = {i_path}::TableMemberImpl<{struct_impl_name}, {columns_mod}::{member_name}, {type_str}>;"
    )
}

pub fn record_id_impl_tpl(
    i_path: &str,
    struct_name: &str,
    struct_impl_name: &str,
    primary_member: &str,
) -> String {
    RECORD_ID_IMPL_TPL
        .replace("{{i_path}}", i_path)
        .replace("{{struct_name}}", struct_name)
        .replace("{{struct_impl_name}}", struct_impl_name)
        .replace("{{primary_member}}", primary_member)
}
