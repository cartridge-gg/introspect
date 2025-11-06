use crate::introspect::ToTypeDef;
use indent::indent_all_by;

pub fn nl_non_empty_list(s: String) -> String {
    if s.is_empty() { s } else { format!("\n{s}\n") }
}

pub fn stack_type_defs<T: ToTypeDef>(type_defs: &[T]) -> String {
    nl_non_empty_list(
        type_defs
            .iter()
            .map(ToTypeDef::to_type_def)
            .collect::<Vec<_>>()
            .join(",\n"),
    )
}

pub fn merge_defs(type_defs: Vec<String>) -> String {
    if type_defs.is_empty() {
        "[].span()".to_string()
    } else if type_defs.len() == 1 {
        type_defs[0].clone()
    } else {
        format!(
            "introspect::types::introspect::merge_defs(\n    array![\n{}\n    ],\n)",
            indent_all_by(8, type_defs.join(",\n"))
        )
    }
}

pub fn make_params(generic_params: &Option<Vec<String>>, traits: &[&str]) -> String {
    match generic_params {
        Some(params) => {
            let mut items = params.clone();
            params
                .iter()
                .for_each(|p| traits.iter().for_each(|t| items.push(format!("+{t}<{p}>"))));
            format!("<{}>", items.join(", "))
        }
        None => Default::default(),
    }
}

pub fn make_introspect_params(generic_params: &Option<Vec<String>>) -> String {
    match generic_params {
        Some(params) => format!(
            "{}",
            params
                .iter()
                .map(|p| format!(", +introspect::Introspect::<{p}>"))
                .collect::<Vec<String>>()
                .join(", ")
        ),
        None => Default::default(),
    }
}
