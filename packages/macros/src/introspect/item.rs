use crate::IItem;
use crate::introspect::IntrospectImpl;

impl IntrospectImpl for IItem {
    fn to_introspect_impl(&self) -> String {
        match self {
            IItem::Struct(struct_item) => struct_item.to_introspect_impl(),
            IItem::Enum(enum_item) => enum_item.to_introspect_impl(),
        }
    }
    fn to_introspect_ref_impl(&self) -> String {
        match self {
            IItem::Struct(struct_item) => struct_item.to_introspect_ref_impl(),
            IItem::Enum(enum_item) => enum_item.to_introspect_ref_impl(),
        }
    }
}
