use indent::indent_by;

use crate::attribute::{IAttribute, iattributes_to_span};

const COLUMN_TYPE_DEF_TPL: &str = include_str!("../templates/column_def.cairo");

pub fn make_column_def(id: &str, name: &str, type_def: &str, attributes: &[IAttribute]) -> String {
    let attributes_str = iattributes_to_span(attributes);
    COLUMN_TYPE_DEF_TPL
        .replace("{{id}}", id)
        .replace("{{name}}", name)
        .replace("{{attributes_str}}", &indent_by(8, attributes_str))
        .replace("{{type_def}}", type_def)
}
