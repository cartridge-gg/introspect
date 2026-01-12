const TABLE_IMPL_NAME: &str = "{{impl_name}}Table";
const COLUMNS_MOD_NAME: &str = "{{impl_name}}Columns";
const COLUMN_ENUM_NAME: &str = "{{impl_name}}Column";
const META_IMPL_NAME: &str = "{{impl_name}}TableMeta";
const PRIMARY_IMPL_NAME: &str = "{{impl_name}}TablePrimary";
const COLUMNS_IMPL_NAME: &str = "{{impl_name}}TableColumns";
const COLUMNS_MOD_TPL: &str = include_str!("../templates/columns_mod.cairo");
const COLUMN_ID_TPL: &str = "pub const {{member}}: felt252 = {{id}};";
const META_IMPL_TPL: &str = include_str!("../templates/meta_impl.cairo");
const PRIMARY_IMPL_TPL: &str = include_str!("../templates/primary_impl.cairo");
const COLUMNS_IMPL_TPL: &str = include_str!("../templates/columns_impl.cairo");

pub fn table_meta_impl_name(impl_name: &str) -> String {
    format!("{impl_name}TableMeta")
}

pub fn table_primary_impl_name(impl_name: &str) -> String {
    format!("{impl_name}TablePrimary")
}

pub fn table_columns_impl_name(impl_name: &str) -> String {
    format!("{impl_name}TableColumns")
}

pub fn table_meta_tpl(
    i_path: &str,
    i_table_path: &str,
    impl_name: &str,
    id: &str,
    name: &str,
    attributes: &str,
) -> String {
    META_IMPL_TPL
        .replace("{{i_path}}", i_path)
        .replace("{{i_table_path}}", i_table_path)
        .replace("{{meta_impl}}", impl_name)
        .replace("{{table_id}}", id)
        .replace("{{table_name}}", name)
        .replace("{{attributes}}", attributes)
}
