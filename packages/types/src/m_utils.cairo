pub use crate::serde::iserialize_keyed_type;
pub use crate::{ChildDefs, EnumDef, ISerde, MemberDef, ResultDef, StructDef, TypeDef, VariantDef};

#[inline]
pub fn iserialize<T, impl I: ISerde<T>>(value: @T, ref output: Array<felt252>) {
    I::iserialize(value, ref output);
}

#[inline]
pub fn ideserialize<T, impl I: ISerde<T>>(ref serialized: Span<felt252>) -> Option<T> {
    I::ideserialize(ref serialized)
}
// #[inline]
// pub fn collect_child_defs<T, impl I: IntrospectT<T>>(ref defs: ChildDefs) {
//     I::collect_child_defs(ref defs);
// }


