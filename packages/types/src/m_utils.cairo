use crate::PrimaryTypeDef;
pub use crate::serde::iserialize_keyed_type;
pub use crate::{
    Attribute, EnumDef, FixedArrayDef, ISerde, Introspect, MemberDef, PrimaryDef, PrimaryTrait,
    ResultDef, StructDef, TypeDef, VariantDef,
};

#[inline(always)]
pub fn member_def(name: ByteArray, attributes: Span<Attribute>, type_def: TypeDef) -> MemberDef {
    MemberDef { name, type_def, attributes }
}

#[inline]
pub fn member_default_def<T, impl I: Introspect<T>>(
    name: ByteArray, attributes: Span<Attribute>,
) -> MemberDef {
    MemberDef { name, type_def: I::type_def(), attributes }
}

#[inline(always)]
pub fn struct_def(
    name: ByteArray, attributes: Span<Attribute>, members: Span<MemberDef>,
) -> StructDef {
    StructDef { name, attributes, members }
}

#[inline(always)]
pub fn struct_type_def(
    name: ByteArray, attributes: Span<Attribute>, members: Span<MemberDef>,
) -> TypeDef {
    TypeDef::Struct(StructDef { name, attributes, members })
}

#[inline(always)]
pub fn variant_def(
    selector: felt252, name: ByteArray, attributes: Span<Attribute>, type_def: TypeDef,
) -> VariantDef {
    VariantDef { selector, name, attributes, type_def }
}

#[inline]
pub fn variant_default_def<T, +Introspect<T>>(
    selector: felt252, name: ByteArray, attributes: Span<Attribute>,
) -> VariantDef {
    VariantDef { selector, name, attributes, type_def: Introspect::<T>::type_def() }
}

#[inline(always)]
pub fn variant_unit_def(
    selector: felt252, name: ByteArray, attributes: Span<Attribute>,
) -> VariantDef {
    VariantDef { selector, name, attributes, type_def: TypeDef::None }
}

#[inline(always)]
pub fn enum_def(
    name: ByteArray, attributes: Span<Attribute>, variants: Span<VariantDef>,
) -> EnumDef {
    EnumDef { name, attributes, variants }
}

#[inline(always)]
pub fn enum_type_def(
    name: ByteArray, attributes: Span<Attribute>, variants: Span<VariantDef>,
) -> TypeDef {
    TypeDef::Enum(EnumDef { name, attributes, variants })
}

#[inline(always)]
pub fn fixed_array_def(type_def: TypeDef, size: u32) -> FixedArrayDef {
    FixedArrayDef { type_def, size }
}

#[inline(always)]
pub fn fixed_array_type_def(type_def: TypeDef, size: u32) -> TypeDef {
    TypeDef::FixedArray(BoxTrait::new(FixedArrayDef { type_def, size }))
}

#[inline(always)]
pub fn result_def(ok: TypeDef, err: TypeDef) -> ResultDef {
    ResultDef { ok, err }
}

#[inline(always)]
pub fn result_type_def(ok: TypeDef, err: TypeDef) -> TypeDef {
    TypeDef::Result(BoxTrait::new(ResultDef { ok, err }))
}

#[inline(always)]
pub fn boxed_type_def(type_def: TypeDef) -> Box<TypeDef> {
    BoxTrait::new(type_def)
}

#[inline(always)]
pub fn primary_def(
    name: ByteArray, attributes: Span<Attribute>, type_def: PrimaryTypeDef,
) -> PrimaryDef {
    PrimaryDef { name, attributes, type_def }
}

#[inline]
pub fn primary_default_def<T, impl P: PrimaryTrait<T>>(
    name: ByteArray, attributes: Span<Attribute>,
) -> PrimaryDef {
    PrimaryDef { name, attributes, type_def: P::to_type_def() }
}

#[inline]
pub fn primary_type_def<T, impl P: PrimaryTrait<T>>() -> PrimaryTypeDef {
    P::to_type_def()
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
pub fn child_defs<T, impl I: Introspect<T>>() -> Array<(felt252, TypeDef)> {
    I::child_defs()
}
