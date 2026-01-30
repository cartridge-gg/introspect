use core::integer::u512;
use core::nullable::null;
use introspect_types::introspect::{ChildDefs, Member, MemberDef, Struct, StructDef, TypeDefTrait};
use introspect_types::serde::partial_terminator;
use introspect_types::type_def::selectors;
// use introspect_types::introspect::{Felt252Def, Member, Struct, TypeDefTrait, U8Def};
// use introspect_types::serde::partial_terminator;
use introspect_types::{ISerde};
use starknet::{ClassHash, ContractAddress, EthAddress};

impl MyStructMember_a of MemberDef<felt252, 1, 0> {
    const NAME: [felt252; 1] = ['a' + partial_terminator::<1>()];
    const ATTRIBUTES: [felt252; 0] = [];
    const ATTRIBUTES_COUNT: u32 = 0; // name size + type size + attributes count + attributes
    const SIZE: u32 = 1 + 1 + 0 + TypeDefTrait::<felt252>::SIZE;
}

impl MyStructMember_b of MemberDef<u8, 1, 0> {
    const NAME: [felt252; 1] = ['b' + partial_terminator::<1>()];
    const ATTRIBUTES: [felt252; 0] = [];
    const ATTRIBUTES_COUNT: u32 = 0; // name size + type size + attributes count + attributes
    const SIZE: u32 = 1 + 1 + 0 + TypeDefTrait::<u8>::SIZE;
}


impl MyStructDef of StructDef<MyStruct, 1, 0> {
    const NAME: [felt252; 1] = ['MyStruct' + partial_terminator::<1>()];
    const ATTRIBUTES: [felt252; 0] = [];
    const ATTRIBUTES_COUNT: u32 = 0;
    const MEMBERS_COUNT: u32 = 2;
    const MEMBERS_SIZE: u32 = MyStructMember_a::SIZE + MyStructMember_b::SIZE;
    fn serialize_members(ref output: Array<felt252>) {
        MyStructMember_a::serialize_member(ref output);
        MyStructMember_b::serialize_member(ref output);
    }
    fn collect_child_defs(ref defs: ChildDefs) {}
}

struct MyStruct {
    a: felt252,
    b: u8,
}

#[test]
fn type_def() {
    let mut output: Array<felt252> = Default::default();

    TypeDefTrait::<felt252>::serialize_type_def(ref output);
    TypeDefTrait::<bool>::serialize_type_def(ref output);
    TypeDefTrait::<u8>::serialize_type_def(ref output);
    TypeDefTrait::<u16>::serialize_type_def(ref output);
    TypeDefTrait::<u32>::serialize_type_def(ref output);
    TypeDefTrait::<u64>::serialize_type_def(ref output);
    TypeDefTrait::<u128>::serialize_type_def(ref output);
    TypeDefTrait::<u256>::serialize_type_def(ref output);
    TypeDefTrait::<u512>::serialize_type_def(ref output);
    TypeDefTrait::<i8>::serialize_type_def(ref output);
    TypeDefTrait::<i16>::serialize_type_def(ref output);
    TypeDefTrait::<i32>::serialize_type_def(ref output);
    TypeDefTrait::<i64>::serialize_type_def(ref output);
    TypeDefTrait::<i128>::serialize_type_def(ref output);

    TypeDefTrait::<bytes31>::serialize_type_def(ref output);
    TypeDefTrait::<ByteArray>::serialize_type_def(ref output);

    TypeDefTrait::<ContractAddress>::serialize_type_def(ref output);
    TypeDefTrait::<ClassHash>::serialize_type_def(ref output);
    TypeDefTrait::<EthAddress>::serialize_type_def(ref output);
    TypeDefTrait::<Option<felt252>>::serialize_type_def(ref output);
    TypeDefTrait::<Result<felt252, Span<felt252>>>::serialize_type_def(ref output);
    TypeDefTrait::<Nullable<felt252>>::serialize_type_def(ref output);
    TypeDefTrait::<Box<felt252>>::serialize_type_def(ref output);
    TypeDefTrait::<Nullable<felt252>>::serialize_type_def(ref output);
    TypeDefTrait::<Array<felt252>>::serialize_type_def(ref output);
    TypeDefTrait::<Span<felt252>>::serialize_type_def(ref output);
    TypeDefTrait::<[felt252; 0]>::serialize_type_def(ref output);
    TypeDefTrait::<[felt252; 1]>::serialize_type_def(ref output);
    TypeDefTrait::<[felt252; 2]>::serialize_type_def(ref output);
    TypeDefTrait::<[felt252; 3]>::serialize_type_def(ref output);
    TypeDefTrait::<[felt252; 16]>::serialize_type_def(ref output);
    TypeDefTrait::<()>::serialize_type_def(ref output);
    TypeDefTrait::<(felt252,)>::serialize_type_def(ref output);
    TypeDefTrait::<(felt252, u8)>::serialize_type_def(ref output);
    TypeDefTrait::<(felt252, u8, u16)>::serialize_type_def(ref output);
    TypeDefTrait::<(felt252, u8, u16, ByteArray)>::serialize_type_def(ref output);
    TypeDefTrait::<MyStruct>::serialize_type_def(ref output);
}

#[test]
fn iserde() {
    let mut output: Array<felt252> = Default::default();

    let val_u512 = u512 { limb0: 1, limb1: 0, limb2: 0, limb3: 0 };

    let byte_array: ByteArray = "hello world";
    let bytes_31: bytes31 = 1.try_into().unwrap();

    let contract_address: ContractAddress = 1_felt252.try_into().unwrap();
    let class_hash: ClassHash = 1_felt252.try_into().unwrap();
    let eth_address: EthAddress = 1_felt252.try_into().unwrap();
    let box = BoxTrait::new(1_felt252);
    let is_null: Nullable<felt252> = null();
    let not_null: Nullable<felt252> = NullableTrait::new(1_felt252);

    let option_some: Option<felt252> = Some(1_felt252);
    let option_none: Option<felt252> = None;
    let result_ok: Result<felt252, Span<felt252>> = Ok(1_felt252);
    let result_err: Result<felt252, Span<felt252>> = Err([1; 20].span());

    let array = array![1_felt252, 2_felt252, 3_felt252];
    let span = [1_felt252, 2_felt252, 3_felt252].span();

    let empty_fixed_array: [felt252; 0] = [];
    let fixed_array_1: [felt252; 1] = [1_felt252];
    let fixed_array_2: [felt252; 2] = [1_felt252, 2_felt252];
    let fixed_array_3: [felt252; 3] = [1_felt252, 2_felt252, 3_felt252];
    let fixed_array_16: [felt252; 16] = [1_felt252; 16];

    let tuple_0: () = ();
    let tuple_1: (felt252,) = (1_felt252,);
    let tuple_2: (felt252, u8) = (1_felt252, 2_u8);
    let tuple_3: (felt252, u8, u16) = (1_felt252, 2_u8, 3_u16);

    1_felt252.iserialize(ref output);

    true.iserialize(ref output);
    false.iserialize(ref output);

    1_u8.iserialize(ref output);
    1_u16.iserialize(ref output);
    1_u32.iserialize(ref output);
    1_u64.iserialize(ref output);
    1_u128.iserialize(ref output);
    1_u256.iserialize(ref output);
    val_u512.iserialize(ref output);
    1_i8.iserialize(ref output);
    1_i16.iserialize(ref output);
    1_i32.iserialize(ref output);
    1_i64.iserialize(ref output);
    1_i128.iserialize(ref output);

    bytes_31.iserialize(ref output);
    byte_array.iserialize(ref output);

    contract_address.iserialize(ref output);
    class_hash.iserialize(ref output);
    eth_address.iserialize(ref output);

    option_some.iserialize(ref output);
    option_none.iserialize(ref output);
    result_ok.iserialize(ref output);
    result_err.iserialize(ref output);
    is_null.iserialize(ref output);
    not_null.iserialize(ref output);

    box.iserialize(ref output);
    array.iserialize(ref output);
    span.iserialize(ref output);

    empty_fixed_array.iserialize(ref output);
    fixed_array_1.iserialize(ref output);
    fixed_array_2.iserialize(ref output);
    fixed_array_3.iserialize(ref output);
    fixed_array_16.iserialize(ref output);
    tuple_0.iserialize(ref output);
    tuple_1.iserialize(ref output);
    tuple_2.iserialize(ref output);
    tuple_3.iserialize(ref output);

    let mut data = output.span();

    ISerde::ideserialize(ref data) == Some(1);

    ISerde::ideserialize(ref data) == Some(true);
    ISerde::ideserialize(ref data) == Some(false);

    ISerde::ideserialize(ref data) == Some(1_u8);
    ISerde::ideserialize(ref data) == Some(1_u16);
    ISerde::ideserialize(ref data) == Some(1_u32);
    ISerde::ideserialize(ref data) == Some(1_u64);
    ISerde::ideserialize(ref data) == Some(1_u128);
    ISerde::ideserialize(ref data) == Some(1_u256);
    ISerde::ideserialize(ref data) == Some(val_u512);
    ISerde::ideserialize(ref data) == Some(1_i8);
    ISerde::ideserialize(ref data) == Some(1_i16);
    ISerde::ideserialize(ref data) == Some(1_i32);
    ISerde::ideserialize(ref data) == Some(1_i64);
    ISerde::ideserialize(ref data) == Some(1_i128);

    ISerde::ideserialize(ref data) == Some(bytes_31);
    ISerde::ideserialize(ref data) == Some(byte_array);

    ISerde::ideserialize(ref data) == Some(contract_address);
    ISerde::ideserialize(ref data) == Some(class_hash);
    ISerde::ideserialize(ref data) == Some(eth_address);

    ISerde::ideserialize(ref data) == Some(option_some);
    ISerde::ideserialize(ref data) == Some(option_none);
    ISerde::ideserialize(ref data) == Some(result_ok);
    ISerde::ideserialize(ref data) == Some(result_err);

    let _: Option<Nullable<felt252>> = ISerde::ideserialize(ref data);
    let _: Option<Nullable<felt252>> = ISerde::ideserialize(ref data);

    let _: Option<Box<felt252>> = ISerde::ideserialize(ref data);
    ISerde::ideserialize(ref data) == Some(array);
    ISerde::ideserialize(ref data) == Some(span);

    ISerde::ideserialize(ref data) == Some(empty_fixed_array);
    ISerde::ideserialize(ref data) == Some(fixed_array_1);
    ISerde::ideserialize(ref data) == Some(fixed_array_2);
    ISerde::ideserialize(ref data) == Some(fixed_array_3);
    ISerde::ideserialize(ref data) == Some(fixed_array_16);
    ISerde::ideserialize(ref data) == Some(tuple_0);
    ISerde::ideserialize(ref data) == Some(tuple_1);
    ISerde::ideserialize(ref data) == Some(tuple_2);
    ISerde::ideserialize(ref data) == Some(tuple_3);
}
// trait ThatHasAConst {
//     const VALUE: felt252;
// }

// impl ThatHasAConstImpl of ThatHasAConst {
//     const VALUE: felt252 = 42;
// }

// impl TestConstTrait of ThatHasAConst {
//     const VALUE: felt252 = ThatHasAConstImpl::VALUE + 1;
// }


