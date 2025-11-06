use crate::introspect::attribute::make_attributes_string;
use crate::introspect::utils::stack_type_defs;
use crate::introspect::{ItemTrait, ToTypeDef, type_child_defs};
use crate::{Enum, Variant};
use indent::indent_by;

const VARIANT_TYPE_DEF_TPL: &str = include_str!("../../templates/variant_def.cairo");
const ENUM_TYPE_DEF_TPL: &str = include_str!("../../templates/enum_def.cairo");

impl ToTypeDef for Variant<'_> {
    fn to_type_def(&self) -> String {
        let attributes_str = make_attributes_string(&self.attributes);
        let ty_str = match &self.ty {
            Some(ty) => ty.clone(),
            None => "introspect::types::TypeDef::None".to_string(),
        };
        VARIANT_TYPE_DEF_TPL
            .replace("{{name}}", &self.name)
            .replace("{{attrs_str}}", indent_by(8, attributes_str).as_str())
            .replace("{{variant_type}}", &ty_str)
    }
}

impl ToTypeDef for Enum<'_> {
    fn to_type_def(&self) -> String {
        let variants_str = stack_type_defs(&self.variants);
        let attributes_str = make_attributes_string(&self.attributes);
        ENUM_TYPE_DEF_TPL
            .replace("{{name}}", &self.name)
            .replace("{{attrs_str}}", indent_by(8, attributes_str).as_str())
            .replace("{{variants_str}}", indent_by(8, variants_str).as_str())
    }
}

impl<'db> ItemTrait for Enum<'db> {
    const ITEM: &'static str = "Enum";
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
