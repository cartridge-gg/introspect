use crate::attribute::{Attribute, IAttribute};
use crate::items::{ItemTrait, make_attributes_string, merge_defs, nl_non_empty_list};
use crate::{Member, Struct};
use indent::indent_by;

const SCHEMA_IMPL_TPL: &str = include_str!("../../templates/schema_impl.cairo");
const COLUMN_TYPE_DEF_TPL: &str = include_str!("../../templates/column_def.cairo");

pub fn make_column_def(id: &str, name: &str, type_def: &str, attributes: &[IAttribute]) -> String {
    let attributes_str = make_attributes_string(attributes);
    COLUMN_TYPE_DEF_TPL
        .replace("{{id}}", id)
        .replace("{{name}}", name)
        .replace("{{attributes_str}}", &indent_by(8, attributes_str))
        .replace("{{type_def}}", type_def)
}

pub fn to_column_def<'db>(member: &Member<'_>) -> String {
    make_column_def(
        &format!("'{}'", &member.name),
        &member.name,
        &member.ty,
        member.iattributes().as_slice(),
    )
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
            .replace("{{full_name}}", &self.full_name())
            .replace(
                "{{impl_params}}",
                &self.generics_with_traits(&["introspect::Introspect"]),
            )
            .replace("{{column_defs}}", &indent_by(8, column_defs_str))
            .replace(
                "{{child_defs}}",
                &indent_by(8, merge_defs(self.child_defs())),
            )
    }
}
