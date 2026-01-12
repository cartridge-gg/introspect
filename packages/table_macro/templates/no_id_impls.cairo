
pub impl {{impl_name}}Table = {{i_table_path}}::TableImpl<{{struct_name}}, {{meta_impl_name}}, {{primary_impl_name}}, {{columns_impl_name}}>;

pub impl I{{impl_name}}Table = {{i_table_path}}::ITableImpl<{{impl_name}}Table>;

{{member_impls}}

impl {{impl_name}}RecordId = {{i_table_path}}::RecordIdFelt252Impl<{{impl_name}}Table>;

impl {{impl_name}}RecordValuesSpan of introspect_table::RecordValuesSpanTrait<{{struct_name}}, {{impl_name}}Table> {
    fn serialize_values(self: @{{struct_name}}, ref data: Array<felt252>) {
        {{serialize_member_calls}}
    }
}