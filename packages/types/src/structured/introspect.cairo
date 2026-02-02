use core::integer::u512;
use core::poseidon::poseidon_hash_span;
use starknet::storage_access::StorageBaseAddress;
use starknet::{ClassHash, ContractAddress, EthAddress, StorageAddress};
use crate::ISerde;
use crate::type_def::selectors;
use super::type_def::MemberDefTrait;
use super::{FixedSizeArrayDef, ResultDef, StructDef, TypeDef};

pub trait TypeDefStructuredRef<T> {
    fn ref_type_def() -> TypeDef;
    fn collect_ref_child_defs(ref defs: ChildDefs);
}

pub trait TypeDefStructured<T> {
    fn type_def() -> TypeDef;
    fn type_id() -> felt252 {
        hash_type_def(@Self::type_def())
    }
    fn collect_children(ref defs: ChildDefs) {}
}

pub type ChildDefs = Array<ChildDef>;

#[derive(Drop)]
pub struct ChildDef {
    pub id: felt252,
    pub type_def: Span<felt252>,
}

// impl ChildDefISerde of ISerde<ChildDef> {
//     fn iserialize(self: @ChildDef, ref output: Array<felt252>) {
//         output.append(*self.id);
//         output.append_span(*self.type_def);
//     }

//     fn ideserialize(ref serialized: Span<felt252>) -> Option<ChildDef> {
//         let id = *serialized.pop_front()?;
//         let type_def = ISerde::ideserialize(ref serialized)?;
//         Some(ChildDef { id, type_def })
//     }
// }

pub impl TypeDefStructuredRefImpl<T, impl IR: TypeDefStructuredRef<T>> of TypeDefStructured<T> {
    fn type_def() -> TypeDef {
        TypeDef::Ref(Self::type_id())
    }
    fn type_id() -> felt252 {
        hash_type_def(@IR::ref_type_def())
    }
    fn collect_children(ref defs: ChildDefs) {
        IR::collect_ref_child_defs(ref defs);
        let type_def_span = IR::ref_type_def().iserialize_inline();
        add_child_def(ref defs, poseidon_hash_span(type_def_span), type_def_span)
    }
}

pub fn hash_type_def(type_def: @TypeDef) -> felt252 {
    hash_type_def_span(type_def.iserialize_inline())
}

pub fn hash_type_def_span(type_def_span: Span<felt252>) -> felt252 {
    poseidon_hash_span(type_def_span)
}

pub fn add_child_def(ref defs: ChildDefs, hash: felt252, type_def_span: Span<felt252>) {
    let mut n = 0;
    let len = defs.len();
    while n != len {
        if *defs[n].id == hash {
            return;
        }
        n += 1;
    }
    defs.append(ChildDef { id: hash, type_def: type_def_span });
}


pub mod impls {
    use super::{TypeDef, TypeDefStructured};
    pub impl Primitive<T, const TY: TypeDef, const ID: felt252> of TypeDefStructured<T> {
        fn type_def() -> TypeDef {
            TY
        }
        fn type_id() -> felt252 {
            ID
        }
    }
    pub impl Empty<T> of TypeDefStructured<T> {
        fn type_def() -> TypeDef {
            TypeDef::None
        }
        fn type_id() -> felt252 {
            0
        }
    }
}
pub mod empty_impl {}

pub impl Felt252Introspect = impls::Primitive<felt252, TypeDef::Felt252, selectors::felt252>;
pub impl Bytes31Introspect = impls::Primitive<bytes31, TypeDef::Bytes31, selectors::bytes31>;
pub impl BoolIntrospect = impls::Primitive<bool, TypeDef::Bool, selectors::bool>;
pub impl U8Introspect = impls::Primitive<u8, TypeDef::U8, selectors::u8>;
pub impl U16Introspect = impls::Primitive<u16, TypeDef::U16, selectors::u16>;
pub impl U32Introspect = impls::Primitive<u32, TypeDef::U32, selectors::u32>;
pub impl U64Introspect = impls::Primitive<u64, TypeDef::U64, selectors::u64>;
pub impl U128Introspect = impls::Primitive<u128, TypeDef::U128, selectors::u128>;
pub impl U256Introspect = impls::Primitive<u256, TypeDef::U256, selectors::u256>;
pub impl U512Introspect = impls::Primitive<u512, TypeDef::U512, selectors::u512>;
pub impl I8Introspect = impls::Primitive<i8, TypeDef::I8, selectors::i8>;
pub impl I16Introspect = impls::Primitive<i16, TypeDef::I16, selectors::i16>;
pub impl I32Introspect = impls::Primitive<i32, TypeDef::I32, selectors::i32>;
pub impl I64Introspect = impls::Primitive<i64, TypeDef::I64, selectors::i64>;
pub impl I128Introspect = impls::Primitive<i128, TypeDef::I128, selectors::i128>;
pub impl ClassHashIntrospect =
    impls::Primitive<ClassHash, TypeDef::ClassHash, selectors::ClassHash>;
pub impl ContractAddressIntrospect =
    impls::Primitive<ContractAddress, TypeDef::ContractAddress, selectors::ContractAddress>;
pub impl EthAddressIntrospect =
    impls::Primitive<EthAddress, TypeDef::EthAddress, selectors::EthAddress>;
pub impl StorageAddressIntrospect =
    impls::Primitive<StorageAddress, TypeDef::StorageAddress, selectors::StorageAddress>;
pub impl StorageBaseAddressIntrospect =
    impls::Primitive<
        StorageBaseAddress, TypeDef::StorageBaseAddress, selectors::StorageBaseAddress,
    >;
pub impl Utf8StringIntrospect =
    impls::Primitive<ByteArray, TypeDef::Utf8String, selectors::Utf8String>;

pub impl Tuple0Introspect = impls::Empty<()>;

pub impl TArrayIntrospect<T, impl I: TypeDefStructured<T>> of TypeDefStructured<Array<T>> {
    fn type_def() -> TypeDef {
        TypeDef::Array(BoxTrait::new(I::type_def()))
    }

    fn collect_children(ref defs: ChildDefs) {
        I::collect_children(ref defs);
    }
}

pub impl TSpanIntrospect<T, impl I: TypeDefStructured<T>> of TypeDefStructured<Span<T>> {
    fn type_def() -> TypeDef {
        TypeDef::Array(BoxTrait::new(I::type_def()))
    }

    fn collect_children(ref defs: ChildDefs) {
        I::collect_children(ref defs);
    }
}

pub impl FixedArrayIntrospect<
    T, const SIZE: u32, impl I: TypeDefStructured<T>,
> of TypeDefStructured<[T; SIZE]> {
    fn type_def() -> TypeDef {
        TypeDef::FixedArray(
            BoxTrait::new(FixedSizeArrayDef { type_def: I::type_def(), size: SIZE }),
        )
    }

    fn collect_children(ref defs: ChildDefs) {
        I::collect_children(ref defs);
    }
}


pub impl BoxIntrospect<T, impl I: TypeDefStructured<T>> of TypeDefStructured<Box<T>> {
    fn type_def() -> TypeDef {
        I::type_def()
    }

    fn collect_children(ref defs: ChildDefs) {
        I::collect_children(ref defs);
    }
}


pub impl Tuple1Introspect<T0, impl I0: TypeDefStructured<T0>> of TypeDefStructured<(T0,)> {
    fn type_def() -> TypeDef {
        TypeDef::Tuple([I0::type_def()].span())
    }
    fn collect_children(ref defs: ChildDefs) {
        I0::collect_children(ref defs);
    }
}

pub impl Tuple2Introspect<
    T0, T1, impl I0: TypeDefStructured<T0>, impl I1: TypeDefStructured<T1>,
> of TypeDefStructured<(T0, T1)> {
    fn type_def() -> TypeDef {
        TypeDef::Tuple([I0::type_def(), I1::type_def()].span())
    }
    fn collect_children(ref defs: ChildDefs) {
        I0::collect_children(ref defs);
        I1::collect_children(ref defs);
    }
}

pub impl Tuple3Introspect<
    T0,
    T1,
    T2,
    impl I0: TypeDefStructured<T0>,
    impl I1: TypeDefStructured<T1>,
    impl I2: TypeDefStructured<T2>,
> of TypeDefStructured<(T0, T1, T2)> {
    fn type_def() -> TypeDef {
        TypeDef::Tuple([I0::type_def(), I1::type_def(), I2::type_def()].span())
    }
    fn collect_children(ref defs: ChildDefs) {
        I0::collect_children(ref defs);
        I1::collect_children(ref defs);
        I2::collect_children(ref defs);
    }
}

pub impl Tuple4Introspect<
    T0,
    T1,
    T2,
    T3,
    impl I0: TypeDefStructured<T0>,
    impl I1: TypeDefStructured<T1>,
    impl I2: TypeDefStructured<T2>,
    impl I3: TypeDefStructured<T3>,
> of TypeDefStructured<(T0, T1, T2, T3)> {
    fn type_def() -> TypeDef {
        TypeDef::Tuple([I0::type_def(), I1::type_def(), I2::type_def(), I3::type_def()].span())
    }
    fn collect_children(ref defs: ChildDefs) {
        I0::collect_children(ref defs);
        I1::collect_children(ref defs);
        I2::collect_children(ref defs);
        I3::collect_children(ref defs);
    }
}


pub impl Tuple5Introspect<
    T0,
    T1,
    T2,
    T3,
    T4,
    impl I0: TypeDefStructured<T0>,
    impl I1: TypeDefStructured<T1>,
    impl I2: TypeDefStructured<T2>,
    impl I3: TypeDefStructured<T3>,
    impl I4: TypeDefStructured<T4>,
> of TypeDefStructured<(T0, T1, T2, T3, T4)> {
    fn type_def() -> TypeDef {
        TypeDef::Tuple(
            [I0::type_def(), I1::type_def(), I2::type_def(), I3::type_def(), I4::type_def()].span(),
        )
    }
    fn collect_children(ref defs: ChildDefs) {
        I0::collect_children(ref defs);
        I1::collect_children(ref defs);
        I2::collect_children(ref defs);
        I3::collect_children(ref defs);
        I4::collect_children(ref defs);
    }
}

pub impl Tuple6Introspect<
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    impl I0: TypeDefStructured<T0>,
    impl I1: TypeDefStructured<T1>,
    impl I2: TypeDefStructured<T2>,
    impl I3: TypeDefStructured<T3>,
    impl I4: TypeDefStructured<T4>,
    impl I5: TypeDefStructured<T5>,
> of TypeDefStructured<(T0, T1, T2, T3, T4, T5)> {
    fn type_def() -> TypeDef {
        TypeDef::Tuple(
            [
                I0::type_def(), I1::type_def(), I2::type_def(), I3::type_def(), I4::type_def(),
                I5::type_def(),
            ]
                .span(),
        )
    }
    fn collect_children(ref defs: ChildDefs) {
        I0::collect_children(ref defs);
        I1::collect_children(ref defs);
        I2::collect_children(ref defs);
        I3::collect_children(ref defs);
        I4::collect_children(ref defs);
        I5::collect_children(ref defs);
    }
}

pub impl OptionTIntrospect<T, impl I: TypeDefStructured<T>> of TypeDefStructured<Option<T>> {
    fn type_def() -> TypeDef {
        TypeDef::Option(BoxTrait::new(I::type_def()))
    }

    fn collect_children(ref defs: ChildDefs) {
        I::collect_children(ref defs);
    }
}

pub impl ResultTEIntrospect<
    T, E, impl IT: TypeDefStructured<T>, impl IE: TypeDefStructured<E>,
> of TypeDefStructured<Result<T, E>> {
    fn type_def() -> TypeDef {
        TypeDef::Result(BoxTrait::new(ResultDef { ok: IT::type_def(), err: IE::type_def() }))
    }
    fn collect_children(ref defs: ChildDefs) {
        IT::collect_children(ref defs);
        IE::collect_children(ref defs);
    }
}

pub impl CallIntrospect of TypeDefStructured<starknet::account::Call> {
    fn type_def() -> TypeDef {
        TypeDef::Struct(
            StructDef {
                name: "Call",
                attributes: [].span(),
                members: [
                    MemberDefTrait::new::<ContractAddress>("to", [].span()),
                    MemberDefTrait::new::<felt252>("selector", [].span()),
                    MemberDefTrait::new::<Span<felt252>>("calldata", [].span()),
                ]
                    .span(),
            },
        )
    }
}

pub impl BlockInfoIntrospect of TypeDefStructured<starknet::BlockInfo> {
    fn type_def() -> TypeDef {
        TypeDef::Struct(
            StructDef {
                name: "BlockInfo",
                attributes: [].span(),
                members: [
                    MemberDefTrait::new::<felt252>("block_hash", [].span()),
                    MemberDefTrait::new::<u64>("block_number", [].span()),
                    MemberDefTrait::new::<u64>("block_timestamp", [].span()),
                    MemberDefTrait::new::<ContractAddress>("sequencer_address", [].span()),
                ]
                    .span(),
            },
        )
    }
}

pub impl ResourceBoundsIntrospect of TypeDefStructured<starknet::ResourcesBounds> {
    fn type_def() -> TypeDef {
        TypeDef::Struct(
            StructDef {
                name: "ResourceBounds",
                attributes: [].span(),
                members: [
                    MemberDefTrait::new::<felt252>("resource", [].span()),
                    MemberDefTrait::new::<u64>("max_amount", [].span()),
                    MemberDefTrait::new::<u128>("max_price_per_unit", [].span()),
                ]
                    .span(),
            },
        )
    }
}


pub impl TxInfoV2Introspect of TypeDefStructured<starknet::TxInfo> {
    fn type_def() -> TypeDef {
        TypeDef::Struct(
            StructDef {
                name: "TxInfo",
                attributes: [].span(),
                members: [
                    MemberDefTrait::new::<felt252>("version", [].span()),
                    MemberDefTrait::new::<ContractAddress>("account_contract_address", [].span()),
                    MemberDefTrait::new::<u128>("max_fee", [].span()),
                    MemberDefTrait::new::<Span<felt252>>("signature", [].span()),
                    MemberDefTrait::new::<felt252>("transaction_hash", [].span()),
                    MemberDefTrait::new::<felt252>("chain_id", [].span()),
                    MemberDefTrait::new::<felt252>("nonce", [].span()),
                    MemberDefTrait::new::<
                        Span<starknet::ResourcesBounds>,
                    >("resource_bounds", [].span()),
                    MemberDefTrait::new::<u128>("tip", [].span()),
                    MemberDefTrait::new::<Span<felt252>>("paymaster_data", [].span()),
                    MemberDefTrait::new::<u32>("nonce_data_availability_mode", [].span()),
                    MemberDefTrait::new::<u32>("fee_data_availability_mode", [].span()),
                    MemberDefTrait::new::<Span<felt252>>("account_deployment_data", [].span()),
                ]
                    .span(),
            },
        )
    }
}
