pub use crate::serde::{
    ISerde, add_checked, add_size_hint, iserialize_keyed_type, match_size_hint, match_size_hints,
    size_hint_add_checked,
};
pub use crate::type_def::custom::{
    AsInline, AsRef, ByteArrayDef, ByteArrayEncoded, Bytes31Def, Bytes31Encoded,
};
pub use crate::type_def::{
    ChildDefs, CompoundDef, DefaultToRef, FieldDef, ResultDef, TypeDef, Utf8StringDef,
};

#[inline]
pub const fn size_hint<T, impl I: ISerde<T>>() -> Option<u32> {
    I::SIZE_HINT
}

#[inline]
pub fn iserialize<T, impl I: ISerde<T>>(value: @T, ref output: Array<felt252>) {
    I::iserialize(value, ref output);
}

#[inline]
pub fn ideserialize<T, impl I: ISerde<T>>(ref serialized: Span<felt252>) -> Option<T> {
    I::ideserialize(ref serialized)
}

#[inline]
pub fn iserialized_size<T, impl I: ISerde<T>>(value: @T) -> u32 {
    I::iserialized_size(value)
}
// #[inline]
// pub fn collect_child_defs<T, impl I: IntrospectT<T>>(ref defs: ChildDefs) {
//     I::collect_child_defs(ref defs);
// }


