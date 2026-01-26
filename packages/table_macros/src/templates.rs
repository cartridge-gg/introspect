const SCHEMA_IMPLS_TPL: &str = include_str!("../templates/structure_impls.cairo");
const TABLE_IMPL_TPL: &str = include_str!("../templates/table_impl.cairo");

const RECORD_PRIMARY_IMPL_TPL: &str = include_str!("../templates/record_primary_impl.cairo");
const RECORD_KEY_IMPL_TPL: &str = include_str!("../templates/record_key_impls.cairo");
const RECORD_SINGLE_KEY_IMPL_TPL: &str = include_str!("../templates/record_single_key_impls.cairo");

const COLUMN_SET_ITEM_TPL: &str = include_str!("../templates/column_set_item_impl.cairo");
const COLUMN_SET_VALUE_TPL: &str = include_str!("../templates/column_set_value_impl.cairo");

pub fn table_impl_name_tpl(impl_name: &str) -> String {
    format!("{impl_name}Table")
}

pub fn interface_impl_name_tpl(impl_name: &str) -> String {
    format!("{impl_name}Table")
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

pub fn column_mod_name_tpl(column_mod_name: &str, member: &str) -> String {
    format!("{column_mod_name}::{member}")
}

pub fn structure_impls_tpl(
    i_path: &str,
    i_table_path: &str,
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
        .replace("{{i_table_path}}", i_table_path)
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

pub fn column_id_const(name: &str, member_impl_name: &str) -> String {
    format!("pub const {name}: felt252 = super::{member_impl_name}::ID;")
}

pub fn append_table_attribute_tpl(attribute: &str) -> String {
    format!("ArrayTrait::append(ref attributes, {attribute});")
}

pub fn table_impl_tpl(
    i_path: &str,
    i_table_path: &str,
    table_impl_name: &str,
    struct_impl_name: &str,
    table_id: &str,
    table_name: &str,
    table_attributes: &str,
) -> String {
    TABLE_IMPL_TPL
        .replace("{{i_path}}", i_path)
        .replace("{{i_table_path}}", i_table_path)
        .replace("{{table_impl_name}}", table_impl_name)
        .replace("{{struct_impl_name}}", struct_impl_name)
        .replace("{{table_id}}", table_id)
        .replace("{{table_name}}", table_name)
        .replace("{{table_attributes}}", table_attributes)
}

pub fn member_impl_name_tpl(struct_name: &str, member_name: &str) -> String {
    format!("{struct_name}_{member_name}_member")
}

pub fn serialize_struct_member_call_tpl(member_impl_name: &str, member_name: &str) -> String {
    format!("{member_impl_name}::serialize_member(self.{member_name}, ref data);")
}

pub fn serialize_member_call_tpl(member_impl_name: &str, member_name: &str) -> String {
    format!("{member_impl_name}::serialize_member({member_name}, ref data);")
}

pub fn member_impl_tpl(
    i_table_path: &str,
    member_impl_name: &str,
    struct_impl_name: &str,
    type_str: &str,
    id: &str,
) -> String {
    format!(
        "pub impl {member_impl_name} = {i_table_path}::TableMemberImpl<{struct_impl_name}, {type_str}, {id}>;"
    )
}

pub fn record_primary_impl_tpl(
    i_table_path: &str,
    struct_name: &str,
    struct_impl_name: &str,
    primary_member_name: &str,
) -> String {
    RECORD_PRIMARY_IMPL_TPL
        .replace("{{i_table_path}}", i_table_path)
        .replace("{{struct_name}}", struct_name)
        .replace("{{struct_impl_name}}", struct_impl_name)
        .replace("{{primary_member_name}}", primary_member_name)
}

pub fn snappable_key_tpl(i_table_path: &str, n: usize, ty: &str) -> String {
    format!("+{i_table_path}::Snapable<@K{n}, {ty}>")
}

pub fn keyed_impls_tpl(
    i_table_path: &str,
    struct_name: &str,
    struct_impl_name: &str,
    key_types: &str,
    snapped_key_types: &str,
    serialize_calls: &str,
    key_members: &str,
    self_key_members: &str,
) -> String {
    RECORD_KEY_IMPL_TPL
        .replace("{{i_table_path}}", i_table_path)
        .replace("{{struct_name}}", struct_name)
        .replace("{{struct_impl_name}}", struct_impl_name)
        .replace("{{key_types}}", key_types)
        .replace("{{snapped_key_types}}", snapped_key_types)
        .replace("{{serialize_calls}}", serialize_calls)
        .replace("{{key_members}}", key_members)
        .replace("{{self_key_members}}", self_key_members)
}

pub fn single_key_impls_tpl(
    i_table_path: &str,
    struct_name: &str,
    struct_impl_name: &str,
    ty: &str,
    key_member: &str,
    member_impl_name: &str,
) -> String {
    RECORD_SINGLE_KEY_IMPL_TPL
        .replace("{{i_table_path}}", i_table_path)
        .replace("{{struct_name}}", struct_name)
        .replace("{{struct_impl_name}}", struct_impl_name)
        .replace("{{ty}}", ty)
        .replace("{{key_member}}", key_member)
        .replace("{{member_impl_name}}", member_impl_name)
}

pub fn column_set_impl_name_tpl(struct_name: &str) -> String {
    format!("__{struct_name}ColumnSet")
}

pub fn column_set_member_impl_tpl(
    i_table_path: &str,
    member_impl_name: &str,
    id: &str,
    ty: &str,
) -> String {
    format!(
        "impl {member_impl_name}: {i_table_path}::Member<Table, Table::Record, {id}>[Type: {ty}]"
    )
}

pub fn column_set_value_impl_tpl(
    i_table_path: &str,
    struct_name: &str,
    impl_name: &str,
    size: &str,
    member_impls: &str,
    column_ids: &str,
    serialize_member_calls: &str,
) -> String {
    COLUMN_SET_VALUE_TPL
        .replace("{{i_table_path}}", i_table_path)
        .replace("{{struct_name}}", struct_name)
        .replace("{{impl_name}}", impl_name)
        .replace("{{size}}", size)
        .replace("{{column_ids}}", column_ids)
        .replace("{{member_impls}}", member_impls)
        .replace("{{serialize_member_calls}}", serialize_member_calls)
}

pub fn column_set_item_impl_tpl(
    i_table_path: &str,
    struct_name: &str,
    impl_name: &str,
    size: &str,
    member_impls: &str,
    column_ids: &str,
    self_keys: &str,
    snapped_key: &str,
    serialize_member_calls: &str,
) -> String {
    COLUMN_SET_ITEM_TPL
        .replace("{{i_table_path}}", i_table_path)
        .replace("{{struct_name}}", struct_name)
        .replace("{{impl_name}}", impl_name)
        .replace("{{size}}", size)
        .replace("{{member_impls}}", member_impls)
        .replace("{{column_ids}}", column_ids)
        .replace("{{snapped_key}}", snapped_key)
        .replace("{{serialize_member_calls}}", serialize_member_calls)
        .replace("{{self_keys}}", self_keys)
}
