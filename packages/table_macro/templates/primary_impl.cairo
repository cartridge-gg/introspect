impl {{primary_impl}} of {{i_table_path}}::TablePrimary {
    type Primary = {{ty}};
    fn primary_def() -> {{i_path}}::PrimaryDef {
        {{i_path}}::PrimaryDef {
            name: {{name}}, attributes: {{attributes}}, type_def: {{type_def}},
        }
    }
}
