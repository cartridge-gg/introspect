use core::dict::Felt252Dict;
use core::poseidon::poseidon_hash_span;
use starknet::{ClassHash, ContractAddress, EthAddress};
use crate::{FixedArrayDef, ResultDef, TypeDef};


pub trait Introspect<T> {
    fn type_def() -> TypeDef;
    fn child_defs() -> Array<(felt252, TypeDef)> {
        Default::default()
    }
    fn hash() -> felt252 {
        let mut serialized: Array<felt252> = Default::default();
        Serde::<TypeDef>::serialize(@Self::type_def(), ref serialized);
        poseidon_hash_span(serialized.span())
    }
}

pub fn merge_defs(
    mut schemas_array: Array<Array<(felt252, TypeDef)>>,
) -> Array<(felt252, TypeDef)> {
    let mut merged = if let Option::Some(first) = schemas_array.pop_front() {
        first
    } else {
        return Default::default();
    };
    let mut added: Felt252Dict<bool> = Default::default();
    loop {
        match schemas_array.pop_front() {
            Option::Some(child_defs) => {
                for (id, def) in child_defs {
                    if !added.get(id) {
                        added.insert(id, true);
                        merged.append((id, def));
                    }
                }
            },
            Option::None => { break; },
        }
    }
    merged
}


pub mod primitive_impl {
    use crate::TypeDef;
    use super::Introspect;
    pub impl PrimitiveIntrospect<T, const TY: TypeDef> of Introspect<T> {
        fn type_def() -> TypeDef {
            TY
        }
    }
}

pub impl Felt252Introspect = primitive_impl::PrimitiveIntrospect<felt252, TypeDef::Felt252>;
pub impl BoolIntrospect = primitive_impl::PrimitiveIntrospect<bool, TypeDef::Bool>;
pub impl U8Introspect = primitive_impl::PrimitiveIntrospect<u8, TypeDef::U8>;
pub impl U16Introspect = primitive_impl::PrimitiveIntrospect<u16, TypeDef::U16>;
pub impl U32Introspect = primitive_impl::PrimitiveIntrospect<u32, TypeDef::U32>;
pub impl U64Introspect = primitive_impl::PrimitiveIntrospect<u64, TypeDef::U64>;
pub impl U128Introspect = primitive_impl::PrimitiveIntrospect<u128, TypeDef::U128>;
pub impl U256Introspect = primitive_impl::PrimitiveIntrospect<u256, TypeDef::U256>;
pub impl I8Introspect = primitive_impl::PrimitiveIntrospect<i8, TypeDef::I8>;
pub impl I16Introspect = primitive_impl::PrimitiveIntrospect<i16, TypeDef::I16>;
pub impl I32Introspect = primitive_impl::PrimitiveIntrospect<i32, TypeDef::I32>;
pub impl I64Introspect = primitive_impl::PrimitiveIntrospect<i64, TypeDef::I64>;
pub impl I128Introspect = primitive_impl::PrimitiveIntrospect<i128, TypeDef::I128>;
pub impl ClassHashIntrospect = primitive_impl::PrimitiveIntrospect<ClassHash, TypeDef::ClassHash>;
pub impl ContractAddressIntrospect =
    primitive_impl::PrimitiveIntrospect<ContractAddress, TypeDef::ContractAddress>;
pub impl EthAddressIntrospect =
    primitive_impl::PrimitiveIntrospect<EthAddress, TypeDef::EthAddress>;


pub impl Tuple0Introspect = primitive_impl::PrimitiveIntrospect<(), TypeDef::None>;

pub impl ByteArrayIntrospect of Introspect<ByteArray> {
    fn type_def() -> TypeDef {
        TypeDef::ByteArray
    }
}


pub impl TArrayIntrospect<T, impl I: Introspect<T>> of Introspect<Array<T>> {
    fn type_def() -> TypeDef {
        TypeDef::Array(BoxTrait::new(I::type_def()))
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        I::child_defs()
    }
}

pub impl TSpanIntrospect<T, impl I: Introspect<T>> of Introspect<Span<T>> {
    fn type_def() -> TypeDef {
        TypeDef::Array(BoxTrait::new(I::type_def()))
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        I::child_defs()
    }
}

pub impl FixedArrayIntrospect<T, const SIZE: u32, impl I: Introspect<T>> of Introspect<[T; SIZE]> {
    fn type_def() -> TypeDef {
        TypeDef::FixedArray(BoxTrait::new(FixedArrayDef { type_def: I::type_def(), size: SIZE }))
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        I::child_defs()
    }
}

pub impl Tuple1Introspect<T0, impl I0: Introspect<T0>> of Introspect<(T0,)> {
    fn type_def() -> TypeDef {
        TypeDef::Tuple([I0::type_def()].span())
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        I0::child_defs()
    }
}

pub impl Tuple2Introspect<
    T0, T1, impl I0: Introspect<T0>, impl I1: Introspect<T1>,
> of Introspect<(T0, T1)> {
    fn type_def() -> TypeDef {
        TypeDef::Tuple([I0::type_def(), I1::type_def()].span())
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        merge_defs(array![I0::child_defs(), I1::child_defs()])
    }
}

pub impl Tuple3Introspect<
    T0, T1, T2, impl I0: Introspect<T0>, impl I1: Introspect<T1>, impl I2: Introspect<T2>,
> of Introspect<(T0, T1, T2)> {
    fn type_def() -> TypeDef {
        TypeDef::Tuple([I0::type_def(), I1::type_def(), I2::type_def()].span())
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        merge_defs(array![I0::child_defs(), I1::child_defs(), I2::child_defs()])
    }
}

pub impl Tuple4Introspect<
    T0,
    T1,
    T2,
    T3,
    impl I0: Introspect<T0>,
    impl I1: Introspect<T1>,
    impl I2: Introspect<T2>,
    impl I3: Introspect<T3>,
> of Introspect<(T0, T1, T2, T3)> {
    fn type_def() -> TypeDef {
        TypeDef::Tuple([I0::type_def(), I1::type_def(), I2::type_def(), I3::type_def()].span())
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        merge_defs(array![I0::child_defs(), I1::child_defs(), I2::child_defs(), I3::child_defs()])
    }
}


pub impl Tuple5Introspect<
    T0,
    T1,
    T2,
    T3,
    T4,
    impl I0: Introspect<T0>,
    impl I1: Introspect<T1>,
    impl I2: Introspect<T2>,
    impl I3: Introspect<T3>,
    impl I4: Introspect<T4>,
> of Introspect<(T0, T1, T2, T3, T4)> {
    fn type_def() -> TypeDef {
        TypeDef::Tuple(
            [I0::type_def(), I1::type_def(), I2::type_def(), I3::type_def(), I4::type_def()].span(),
        )
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        merge_defs(
            array![
                I0::child_defs(), I1::child_defs(), I2::child_defs(), I3::child_defs(),
                I4::child_defs(),
            ],
        )
    }
}

pub impl Tuple6Introspect<
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    impl I0: Introspect<T0>,
    impl I1: Introspect<T1>,
    impl I2: Introspect<T2>,
    impl I3: Introspect<T3>,
    impl I4: Introspect<T4>,
    impl I5: Introspect<T5>,
> of Introspect<(T0, T1, T2, T3, T4, T5)> {
    fn type_def() -> TypeDef {
        TypeDef::Tuple(
            [
                I0::type_def(), I1::type_def(), I2::type_def(), I3::type_def(), I4::type_def(),
                I5::type_def(),
            ]
                .span(),
        )
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        merge_defs(
            array![
                I0::child_defs(), I1::child_defs(), I2::child_defs(), I3::child_defs(),
                I4::child_defs(), I5::child_defs(),
            ],
        )
    }
}


pub impl OptionIntrospect<T, impl I: Introspect<T>> of Introspect<Option<T>> {
    fn type_def() -> TypeDef {
        TypeDef::Option(BoxTrait::new(I::type_def()))
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        I::child_defs()
    }
}

pub impl ResultIntrospect<
    T, E, impl IT: Introspect<T>, impl IE: Introspect<E>,
> of Introspect<Result<T, E>> {
    fn type_def() -> TypeDef {
        TypeDef::Result(BoxTrait::new(ResultDef { ok: IT::type_def(), err: IE::type_def() }))
    }
    fn child_defs() -> Array<(felt252, TypeDef)> {
        merge_defs(array![IT::child_defs(), IE::child_defs()])
    }
}

