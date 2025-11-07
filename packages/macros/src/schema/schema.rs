use crate::params::make_params;
use crate::type_def::{ItemTrait, make_attributes_string, merge_defs, nl_non_empty_list};
use crate::{Member, Struct};
use indent::indent_by;

const SCHEMA_IMPL_TPL: &str = include_str!("../../templates/schema_impl.cairo");
const COLUMN_TYPE_DEF_TPL: &str = include_str!("../../templates/column_def.cairo");

pub fn to_column_def<'db>(member: &Member<'_>) -> String {
    let attributes_str = make_attributes_string(&member.attributes);
    COLUMN_TYPE_DEF_TPL
        .replace("{{id}}", format!("'{}'", &member.name).as_str())
        .replace("{{name}}", &member.name)
        .replace("{{attrs_str}}", indent_by(8, attributes_str).as_str())
        .replace("{{type_def}}", &member.ty)
}

pub trait ToSchemaImpl {
    fn to_schema_impl(&mut self) -> String;
}

impl<'db> ToSchemaImpl for Struct<'db> {
    fn to_schema_impl(&mut self) -> String {
        let column_defs_str = nl_non_empty_list(
            self.members
                .iter()
                .map(to_column_def)
                .collect::<Vec<_>>()
                .join(",\n"),
        );
        SCHEMA_IMPL_TPL
            .replace("{{name}}", self.name())
            .replace(
                "{{impl_params}}",
                make_params(self.generic_params(), &["introspect::Introspect"], false).as_str(),
            )
            .replace(
                "{{params}}",
                make_params(self.generic_params(), &[], false).as_str(),
            )
            .replace("{{column_defs}}", indent_by(8, column_defs_str).as_str())
            .replace(
                "{{child_defs}}",
                indent_by(8, merge_defs(self.child_defs())).as_str(),
            )
    }
}
