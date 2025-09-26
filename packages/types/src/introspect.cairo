use core::dict::Felt252Dict;
use starknet::{ClassHash, ContractAddress, EthAddress};
use crate::Ty;

trait Introspect<T> {
    fn introspect() -> Ty;
    fn schemas() -> Array<(felt252, Ty)> {
        Default::default()
    }
    const fn size() -> Option<u32> {
        None
    }
}

pub fn merge_schemas(mut schemas_array: Array<Array<(felt252, Ty)>>) -> Array<(felt252, Ty)> {
    let mut merged = if let Option::Some(first) = schemas_array.pop_front() {
        first
    } else {
        return Default::default();
    };
    let mut added: Felt252Dict<bool> = Default::default();
    loop {
        match schemas_array.pop_front() {
            Option::Some(schemas) => {
                for (id, ty) in schemas {
                    if !added.get(id) {
                        added.insert(id, true);
                        merged.append((id, ty));
                    }
                }
            },
            Option::None => { break; },
        }
    }
    merged
}


pub mod primitive_impl {
    use crate::Ty;
    use super::Introspect;
    pub impl PrimitiveIntrospect<T, const TY: Ty, const SIZE: u32> of Introspect<T> {
        fn introspect() -> Ty {
            TY
        }
        const fn size() -> Option<u32> {
            Some(SIZE)
        }
    }
}

pub mod short_string {
    use crate::Ty;

    pub impl ShortStringIntrospectImpl =
        super::primitive_impl::PrimitiveIntrospect<felt252, Ty::ShortString, 1>;
}


pub impl Felt252Introspect = primitive_impl::PrimitiveIntrospect<felt252, Ty::Felt252, 1>;
pub impl BoolIntrospect = primitive_impl::PrimitiveIntrospect<bool, Ty::Bool, 1>;
pub impl Uint8Introspect = primitive_impl::PrimitiveIntrospect<u8, Ty::Uint8, 1>;
pub impl Uint16Introspect = primitive_impl::PrimitiveIntrospect<u16, Ty::Uint16, 1>;
pub impl Uint32Introspect = primitive_impl::PrimitiveIntrospect<u32, Ty::Uint32, 1>;
pub impl Uint64Introspect = primitive_impl::PrimitiveIntrospect<u64, Ty::Uint64, 1>;
pub impl Uint128Introspect = primitive_impl::PrimitiveIntrospect<u128, Ty::Uint128, 1>;
pub impl Uint256Introspect = primitive_impl::PrimitiveIntrospect<u256, Ty::Uint256, 2>;
pub impl Int8Introspect = primitive_impl::PrimitiveIntrospect<i8, Ty::Int8, 1>;
pub impl Int16Introspect = primitive_impl::PrimitiveIntrospect<i16, Ty::Int16, 1>;
pub impl Int32Introspect = primitive_impl::PrimitiveIntrospect<i32, Ty::Int32, 1>;
pub impl Int64Introspect = primitive_impl::PrimitiveIntrospect<i64, Ty::Int64, 1>;
pub impl Int128Introspect = primitive_impl::PrimitiveIntrospect<i128, Ty::Int128, 1>;
pub impl USizeIntrospect = primitive_impl::PrimitiveIntrospect<usize, Ty::USize, 1>;
pub impl ClassHashIntrospect = primitive_impl::PrimitiveIntrospect<ClassHash, Ty::ClassHash, 1>;
pub impl ContractAddressIntrospect =
    primitive_impl::PrimitiveIntrospect<ContractAddress, Ty::ContractAddress, 2>;
pub impl EthAddressIntrospect = primitive_impl::PrimitiveIntrospect<EthAddress, Ty::EthAddress, 1>;


pub impl Tuple0Introspect = primitive_impl::PrimitiveIntrospect<(), Ty::None, 0>;

pub impl ByteArrayIntrospect of Introspect<ByteArray> {
    fn introspect() -> Ty {
        Ty::ByteArray
    }
}


pub impl TArrayIntrospect<T, impl I: Introspect<T>> of Introspect<Array<T>> {
    fn introspect() -> Ty {
        Ty::Array(BoxTrait::new(I::introspect()))
    }
    fn schemas() -> Array<(felt252, Ty)> {
        I::schemas()
    }
}

pub impl FixedArrayIntrospect<T, const N: u32, impl I: Introspect<T>> of Introspect<[T; N]> {
    fn introspect() -> Ty {
        Ty::FixedArray((BoxTrait::new(I::introspect()), N))
    }
    fn schemas() -> Array<(felt252, Ty)> {
        I::schemas()
    }
    const fn size() -> Option<u32> {
        match I::size() {
            Option::Some(size) => Option::Some(size * N),
            Option::None => Option::None,
        }
    }
}

pub impl Tuple1Introspect<T0, impl I0: Introspect<T0>> of Introspect<(T0,)> {
    fn introspect() -> Ty {
        Ty::Tuple([I0::introspect()].span())
    }
    fn schemas() -> Array<(felt252, Ty)> {
        I0::schemas()
    }
    const fn size() -> Option<u32> {
        I0::size()
    }
}

pub impl Tuple2Introspect<
    T0, T1, impl I0: Introspect<T0>, impl I1: Introspect<T1>,
> of Introspect<(T0, T1)> {
    fn introspect() -> Ty {
        Ty::Tuple([I0::introspect(), I1::introspect()].span())
    }
    fn schemas() -> Array<(felt252, Ty)> {
        let mut schemas1 = I0::schemas();
        schemas1.append_span(I1::schemas().span());
        schemas1
    }
    const fn size() -> Option<u32> {
        match (I0::size(), I1::size()) {
            (Option::Some(size0), Option::Some(size1)) => Option::Some(size0 + size1),
            _ => Option::None,
        }
    }
}

pub impl Tuple3Introspect<
    T0, T1, T2, impl I0: Introspect<T0>, impl I1: Introspect<T1>, impl I2: Introspect<T2>,
> of Introspect<(T0, T1, T2)> {
    fn introspect() -> Ty {
        Ty::Tuple([I0::introspect(), I1::introspect(), I2::introspect()].span())
    }
    fn schemas() -> Array<(felt252, Ty)> {
        let mut schemas1 = I0::schemas();
        schemas1.append_span(I1::schemas().span());
        schemas1.append_span(I2::schemas().span());
        schemas1
    }
    const fn size() -> Option<u32> {
        match (I0::size(), I1::size(), I2::size()) {
            (
                Option::Some(size0), Option::Some(size1), Option::Some(size2),
            ) => Option::Some(size0 + size1 + size2),
            _ => Option::None,
        }
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
    fn introspect() -> Ty {
        Ty::Tuple([I0::introspect(), I1::introspect(), I2::introspect(), I3::introspect()].span())
    }
    fn schemas() -> Array<(felt252, Ty)> {
        let mut schemas1 = I0::schemas();
        schemas1.append_span(I1::schemas().span());
        schemas1.append_span(I2::schemas().span());
        schemas1.append_span(I3::schemas().span());
        schemas1
    }
    const fn size() -> Option<u32> {
        match (I0::size(), I1::size(), I2::size(), I3::size()) {
            (
                Option::Some(size0), Option::Some(size1), Option::Some(size2), Option::Some(size3),
            ) => Option::Some(size0 + size1 + size2 + size3),
            _ => Option::None,
        }
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
    fn introspect() -> Ty {
        Ty::Tuple(
            [
                I0::introspect(), I1::introspect(), I2::introspect(), I3::introspect(),
                I4::introspect(),
            ]
                .span(),
        )
    }
    fn schemas() -> Array<(felt252, Ty)> {
        let mut schemas1 = I0::schemas();
        schemas1.append_span(I1::schemas().span());
        schemas1.append_span(I2::schemas().span());
        schemas1.append_span(I3::schemas().span());
        schemas1.append_span(I4::schemas().span());
        schemas1
    }
    const fn size() -> Option<u32> {
        match (I0::size(), I1::size(), I2::size(), I3::size(), I4::size()) {
            (
                Option::Some(size0),
                Option::Some(size1),
                Option::Some(size2),
                Option::Some(size3),
                Option::Some(size4),
            ) => Option::Some(size0 + size1 + size2 + size3 + size4),
            _ => Option::None,
        }
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
    fn introspect() -> Ty {
        Ty::Tuple(
            [
                I0::introspect(), I1::introspect(), I2::introspect(), I3::introspect(),
                I4::introspect(), I5::introspect(),
            ]
                .span(),
        )
    }
    fn schemas() -> Array<(felt252, Ty)> {
        merge_schemas(
            array![
                I0::schemas(), I1::schemas(), I2::schemas(), I3::schemas(), I4::schemas(),
                I5::schemas(),
            ],
        )
    }
    const fn size() -> Option<u32> {
        match (I0::size(), I1::size(), I2::size(), I3::size(), I4::size(), I5::size()) {
            (
                Option::Some(size0),
                Option::Some(size1),
                Option::Some(size2),
                Option::Some(size3),
                Option::Some(size4),
                Option::Some(size5),
            ) => Option::Some(size0 + size1 + size2 + size3 + size4 + size5),
            _ => Option::None,
        }
    }
}


pub impl OptionIntrospect<T, impl I: Introspect<T>> of Introspect<Option<T>> {
    fn introspect() -> Ty {
        Ty::Option(BoxTrait::new(I::introspect()))
    }
    fn schemas() -> Array<(felt252, Ty)> {
        I::schemas()
    }
}

pub impl ResultIntrospect<
    T, E, impl IT: Introspect<T>, impl IE: Introspect<E>,
> of Introspect<Result<T, E>> {
    fn introspect() -> Ty {
        Ty::Result((BoxTrait::new(IT::introspect()), BoxTrait::new(IE::introspect())))
    }
    fn schemas() -> Array<(felt252, Ty)> {
        merge_schemas(array![IT::schemas(), IE::schemas()])
    }
}

