use crate::items::{
    ItemTrait, ToTypeDef, make_attributes_string, stack_type_defs, type_child_defs,
};
use crate::{Enum, Variant};
use indent::indent_by;

const VARIANT_TYPE_DEF_TPL: &str = include_str!("../../templates/variant_def.cairo");
const ENUM_TYPE_DEF_TPL: &str = include_str!("../../templates/enum_def.cairo");

impl ToTypeDef for Variant<'_> {
    fn to_type_def(&self) -> String {
        let attributes_str = make_attributes_string(&self.iattributes());
        let ty_str = match &self.ty {
            Some(ty) => format!("introspect::Introspect::<{ty}>::type_def()"),
            None => "introspect::TypeDef::None".to_string(),
        };
        VARIANT_TYPE_DEF_TPL
            .replace("{{selector}}", &self.selector)
            .replace("{{name}}", &self.name)
            .replace("{{attributes_str}}", indent_by(8, attributes_str).as_str())
            .replace("{{type_def}}", &ty_str)
    }
}

impl ToTypeDef for Enum<'_> {
    fn to_type_def(&self) -> String {
        let variants_str = stack_type_defs(&self.variants);
        let attributes_str = make_attributes_string(&self.iattributes());
        ENUM_TYPE_DEF_TPL
            .replace("{{name}}", &self.name)
            .replace("{{attributes_str}}", indent_by(8, attributes_str).as_str())
            .replace("{{variants_str}}", indent_by(4, variants_str).as_str())
    }
}

impl<'db> ItemTrait for Enum<'db> {
    fn kind(&self) -> &str {
        "Enum"
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn generic_params(&self) -> &Option<Vec<String>> {
        &self.generic_params
    }
    fn child_defs(&self) -> Vec<String> {
        self.variants
            .iter()
            .filter_map(|v| match &v.ty {
                Some(ty) => Some(type_child_defs(ty)),
                None => None,
            })
            .collect()
    }
}
