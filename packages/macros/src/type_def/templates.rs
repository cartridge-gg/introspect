pub fn as_type_def(i_path: &str, variant: &str, type_def: String) -> String {
    format!("{i_path}::{}({})", variant, type_def)
}

pub fn as_unit_type_def(i_path: &str, variant: &str) -> String {
    format!("{i_path}::TypeDef::{}", variant)
}

pub fn as_type_def_boxed(i_path: &str, variant: &str, type_def: String) -> String {
    format!(
        "{i_path}::TypeDef::{}(BoxTrait::new({}))",
        variant, type_def
    )
}

pub fn attribute_data_tpl(i_path: &str, name: &str, data: &str) -> String {
    format!("{i_path}::attribute_data({name}, {data})")
}

pub fn attribute_empty_tpl(i_path: &str, name: &str) -> String {
    format!("{i_path}::attribute_empty({name})")
}

pub fn member_default_def_tpl(i_path: &str, name: &str, attributes: &str, ty: &str) -> String {
    format!("{i_path}::member_default_def::<{ty}>({name}, {attributes})")
}

pub fn member_def_tpl(i_path: &str, name: &str, attributes: &str, type_def: &str) -> String {
    format!("{i_path}::member_def({name}, {attributes}, {type_def})")
}

pub fn struct_def_tpl(i_path: &str, name: &str, attributes: &str, members: &str) -> String {
    format!("{i_path}::struct_def({name}, {attributes}, {members})")
}

pub fn as_struct_type_def_tpl(i_path: &str, name: &str, attributes: &str, members: &str) -> String {
    format!("{i_path}::struct_type_def({name}, {attributes}, {members})",)
}

pub fn variant_default_def_tpl(
    i_path: &str,
    selector: &str,
    name: &str,
    attributes: &str,
    ty: &str,
) -> String {
    format!("{i_path}::variant_default_def::<{ty}>({selector}, {name}, {attributes})")
}

pub fn variant_def_tpl(
    i_path: &str,
    selector: &str,
    name: &str,
    attributes: &str,
    type_def: &str,
) -> String {
    format!("{i_path}::variant_def({selector}, {name}, {attributes}, {type_def})")
}

pub fn variant_unit_def_tpl(i_path: &str, selector: &str, name: &str, attributes: &str) -> String {
    format!("{i_path}::variant_unit_def({selector}, {name}, {attributes})")
}

pub fn enum_def_tpl(i_path: &str, name: &str, attributes: &str, variants: &str) -> String {
    format!("{i_path}::enum_def({name}, {attributes}, {variants})")
}

pub fn enum_type_def_tpl(i_path: &str, name: &str, attributes: &str, variants: &str) -> String {
    format!("{i_path}::enum_type_def({name}, {attributes}, {variants})",)
}

pub fn fixed_array_type_def_tpl(i_path: &str, type_def: &str, size: u32) -> String {
    format!("{i_path}::fixed_array_type_def({type_def}, {size})")
}

pub fn array_type_def_tpl(i_path: &str, type_def: &str) -> String {
    format!("{i_path}::array_type_def({type_def})")
}

pub fn result_type_def_tpl(i_path: &str, ok: &str, err: &str) -> String {
    format!("{i_path}::result_type_def({ok}, {err})")
}

pub fn option_type_def_tpl(i_path: &str, type_def: &str) -> String {
    format!("{i_path}::option_type_def({type_def})")
}

pub fn nullable_type_def_tpl(i_path: &str, type_def: &str) -> String {
    format!("{i_path}::nullable_type_def({type_def})")
}

pub fn felt252_dict_type_def_tpl(i_path: &str, type_def: &str) -> String {
    format!("{i_path}::felt252_dict_type_def({type_def})")
}

pub fn ref_type_def_tpl(i_path: &str, type_id: &str) -> String {
    format!("{i_path}::ref_type_def({type_id})")
}

pub fn collect_child_defs_tpl(i_path: &str, type_str: &str) -> String {
    format!("{i_path}::collect_child_defs::<{}>(ref defs);", type_str)
}

pub fn tuple_def_tpl(i_path: &str, elements: &str) -> String {
    format!("{i_path}::tuple_def({elements})")
}
