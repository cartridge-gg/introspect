use super::{ToTypeDef, type_child_defs};
use crate::introspect::ItemTrait;
use crate::introspect::attribute::make_attributes_string;
use crate::introspect::utils::stack_type_defs;
use crate::{Member, Struct};
use indent::indent_by;

const STRUCT_TYPE_DEF_TPL: &str = include_str!("../../templates/struct_def.cairo");
const MEMBER_TYPE_DEF_TPL: &str = include_str!("../../templates/member_def.cairo");

impl ToTypeDef for Member<'_> {
    fn to_type_def(&self) -> String {
        let attributes_str = make_attributes_string(&self.attributes);
        MEMBER_TYPE_DEF_TPL
            .replace("{{name}}", &self.name)
            .replace("{{attrs_str}}", indent_by(8, attributes_str).as_str())
            .replace("{{member_type}}", &self.ty)
    }
}

impl ToTypeDef for Struct<'_> {
    fn to_type_def(&self) -> String {
        let attributes_str = make_attributes_string(&self.attributes);
        let members_str = stack_type_defs(&self.members);
        STRUCT_TYPE_DEF_TPL
            .replace("{{name}}", &self.name)
            .replace("{{attrs_str}}", indent_by(8, attributes_str).as_str())
            .replace("{{members_str}}", indent_by(8, members_str).as_str())
    }
}

impl<'db> ItemTrait for Struct<'db> {
    const ITEM: &'static str = "Struct";
    fn name(&self) -> &str {
        &self.name
    }
    fn generic_params(&self) -> &Option<Vec<String>> {
        &self.generic_params
    }
    fn child_defs(&self) -> Vec<String> {
        self.members
            .iter()
            .map(|m| type_child_defs(&m.ty))
            .collect()
    }
}
