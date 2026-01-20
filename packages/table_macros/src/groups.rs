pub struct ColumnGroup {
    pub name: String,
    pub generic_names: Vec<String>,
    pub columns: Vec<Column>,
    pub group_id: Felt,
}

impl ColumnGroup {
    pub fn column_group_impl(
        &self,
        i_table_path: &str,
        table_impl_name: &str,
        columns_mod_name: &str,
    ) -> String {
    }
}
