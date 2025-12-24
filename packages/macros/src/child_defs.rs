use indent::indent_all_by;

use crate::utils::is_of_base_types;

pub fn type_child_defs(ty: &str) -> String {
    format!("introspect::Introspect::<{}>::child_defs()", ty)
}

const MERGE_DEFS_TPL: &str = "introspect::merge_defs(array![\n{{child_defs}},\n],\n)";

pub fn combined_type_child_defs(type_defs: Vec<String>) -> String {
    let type_defs = type_defs
        .into_iter()
        .filter(|t| !is_of_base_types(t))
        .map(|t| type_child_defs(&t))
        .collect::<Vec<_>>();
    if type_defs.is_empty() {
        "array![]".to_string()
    } else if type_defs.len() == 1 {
        type_defs[0].clone()
    } else {
        MERGE_DEFS_TPL.replace("{{child_defs}}", &indent_all_by(8, type_defs.join(",\n")))
    }
}
