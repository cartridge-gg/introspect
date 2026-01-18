use crate::IItem;
use crate::introspect::IntrospectImpl;

impl IntrospectImpl for IItem {
    fn to_introspect_impl<const IS_REF: bool>(&self, i_path: &str) -> String {
        match self {
            IItem::Struct(struct_item) => struct_item.to_introspect_impl::<IS_REF>(i_path),
            IItem::Enum(enum_item) => enum_item.to_introspect_impl::<IS_REF>(i_path),
        }
    }
}
