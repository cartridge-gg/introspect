impl {{primary_impl}} of {{i_table_path}}::TablePrimary {
    type Primary = {{ty}};
    fn primary_def() -> {{i_path}}::PrimaryDef {
        {{i_path}}::primary_default_def::<Self::Primary>({{name}}, {{attributes}})
    }
}
