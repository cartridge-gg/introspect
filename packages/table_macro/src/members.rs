use introspect_macros::Struct;

const TABLE_MEMBER_IMPL_TPL: &str = "impl {{table_name}}_{{member_name}}_MemberImpl = introspect_table::iserde_table_member::Impl<{{table_name}}Table, {{table_name}}Columns::{{member_name}}, {{type}}>";

fn make_table_member(strict: &Struct, n: usize) -> String {
    let member = &strict.members[n];
}
