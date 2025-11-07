use indent::indent_all_by;

pub mod attribute;
pub mod enums;
pub mod structs;

pub use attribute::make_attributes_string;

pub fn merge_defs(type_defs: Vec<String>) -> String {
    if type_defs.is_empty() {
        "array![]".to_string()
    } else if type_defs.len() == 1 {
        type_defs[0].clone()
    } else {
        format!(
            "introspect::types::introspect::merge_defs(\n    array![\n{},\n    ],\n)",
            indent_all_by(8, type_defs.join(",\n"))
        )
    }
}

pub trait ToTypeDef {
    fn to_type_def(&self) -> String;
}

pub trait ItemTrait {
    const ITEM: &'static str;
    fn name(&self) -> &str;
    fn generic_params(&self) -> &Option<Vec<String>>;
    fn child_defs(&self) -> Vec<String>;
}

pub fn type_child_defs(ty: &str) -> String {
    format!("introspect::Introspect::<{}>::child_defs()", ty)
}

pub fn pad_nl(s: &str) -> String {
    format!("\n{},\n    ", indent_all_by(4, s).as_str())
}

pub fn nl_non_empty_list(s: String) -> String {
    if s.is_empty() {
        s
    } else {
        format!("\n{},\n", indent_all_by(4, s).as_str())
    }
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
