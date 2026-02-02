use core::integer::u512;
use core::nullable::null;
use introspect_types::{ChildDefs, EnumDef, ISerde, ISerdeByteArray, StructDef, TypeDef, structured};
use starknet::{ClassHash, ContractAddress, EthAddress};

struct MyStruct {
    a: (felt252, u128, Option<felt252>),
    b: u8,
}

struct MyOtherStruct {
    a: (felt252, u128, Option<felt252>),
    b: u8,
}

enum MyEnum {
    Variant1,
    Variant2: (felt252, u128, Option<felt252>),
}


fn make_my_struct_def() -> structured::TypeDef {
    structured::TypeDef::Struct(
        structured::StructDef {
            name: "MyStruct",
            attributes: [].span(),
            members: [
                structured::MemberDef {
                    name: "a",
                    attributes: [].span(),
                    type_def: structured::TypeDef::Tuple(
                        [
                            structured::TypeDef::Felt252, structured::TypeDef::U128,
                            structured::TypeDef::Option(
                                BoxTrait::new(structured::TypeDef::Felt252),
                            ),
                        ]
                            .span(),
                    ),
                },
                structured::MemberDef {
                    name: "b", attributes: [].span(), type_def: structured::TypeDef::U8,
                },
            ]
                .span(),
        },
    )
}

mod MyStructMember {
    use super::*;
    pub impl a of introspect_types::MemberDef<2> {
        const META_DATA: [felt252; 2] = ['a'.partial_terminator(1), 0];
        type Type = (felt252, u128, Option<felt252>);
    }

    pub impl b of introspect_types::MemberDef<2> {
        const META_DATA: [felt252; 2] = ['b'.partial_terminator(1), 0];
        type Type = u8;
    }
}

impl MyStructDef of StructDef<MyStruct, 1, 0> {
    const NAME: [felt252; 1] = ['MyStruct'.partial_terminator(8)];
    const ATTRIBUTES_COUNT: u32 = 0;
    const ATTRIBUTES: [felt252; 0] = [];
    const MEMBERS_COUNT: u32 = 2;
    const MEMBERS_SIZE: u32 = MyStructMember::a::SIZE() + MyStructMember::b::SIZE();
    const REF: bool = false;
    fn serialize_members(ref output: Array<felt252>) {
        MyStructMember::a::serialize(ref output);
        MyStructMember::b::serialize(ref output);
    }
    fn collect_member_children(ref children: ChildDefs) {
        MyStructMember::a::collect_children(ref children);
        MyStructMember::b::collect_children(ref children);
    }
    fn serialize_members_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
        MyStructMember::a::serialize_with_children(ref type_def, ref children);
        MyStructMember::b::serialize_with_children(ref type_def, ref children);
    }
}

impl MyOtherStructDef of StructDef<MyOtherStruct, 1, 0> {
    const NAME: [felt252; 1] = ['MyOtherStruct'.partial_terminator(13)];
    const ATTRIBUTES_COUNT: u32 = 0;
    const ATTRIBUTES: [felt252; 0] = [];
    const MEMBERS_COUNT: u32 = 2;
    const MEMBERS_SIZE: u32 = MyStructMember::a::SIZE() + MyStructMember::b::SIZE();
    const REF: bool = true;
    fn serialize_members(ref output: Array<felt252>) {
        MyStructMember::a::serialize(ref output);
        MyStructMember::b::serialize(ref output);
    }
    fn collect_member_children(ref children: ChildDefs) {
        MyStructMember::a::collect_children(ref children);
        MyStructMember::b::collect_children(ref children);
    }
    fn serialize_members_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
        MyStructMember::a::serialize_with_children(ref type_def, ref children);
        MyStructMember::b::serialize_with_children(ref type_def, ref children);
    }
}

mod MyEnumVariant {
    pub use introspect_types::ISerdeByteArray;
    pub impl Variant1 of introspect_types::VariantDef<2> {
        const SELECTOR: felt252 = selector!("Variant1");
        const META_DATA: [felt252; 2] = ['Variant1'.partial_terminator(8), 0];
        type Type = ();
    }
    pub impl Variant2 of introspect_types::VariantDef<2> {
        const SELECTOR: felt252 = selector!("Variant2");
        const META_DATA: [felt252; 2] = ['Variant2'.partial_terminator(8), 0];
        type Type = (felt252, u128, Option<felt252>);
    }
}

impl MyEnumDef of EnumDef<MyEnum, 1, 0> {
    const NAME: [felt252; 1] = ['MyEnum'.partial_terminator(6)];
    const ATTRIBUTES_COUNT: u32 = 0;
    const ATTRIBUTES: [felt252; 0] = [];
    const VARIANTS_COUNT: u32 = 2;
    const VARIANTS_SIZE: u32 = MyEnumVariant::Variant1::SIZE() + MyEnumVariant::Variant2::SIZE();
    const REF: bool = false;
    fn serialize_variants(ref output: Array<felt252>) {
        MyEnumVariant::Variant1::serialize(ref output);
        MyEnumVariant::Variant2::serialize(ref output);
    }
    fn collect_variant_children(ref children: ChildDefs) {
        MyEnumVariant::Variant1::collect_children(ref children);
        MyEnumVariant::Variant2::collect_children(ref children);
    }
    fn serialize_variants_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
        MyEnumVariant::Variant1::serialize_with_children(ref type_def, ref children);
        MyEnumVariant::Variant2::serialize_with_children(ref type_def, ref children);
    }
}


fn test_type_def_trait<T, const REF: bool, impl TD: TypeDef<T, REF>>(expected: ByteArray) {
    let mut output: Array<felt252> = Default::default();
    TD::serialize(ref output);
    // assert_eq!(output.len(), TypeDef::SIZE);
    let mut data = output.span();
    println!("---------------------------------------");
    println!("{:?}", expected);
    match ISerde::<structured::TypeDef>::ideserialize(ref data) {
        Some(type_def) => { println!("{:?}", type_def); },
        None => {
            println!("{:?}", output);
            panic!("Failed to deserialize type def");
        },
    }
}

#[test]
fn type_def_tests() {
    test_type_def_trait::<felt252>("felt252");
    test_type_def_trait::<bool>("bool");
    test_type_def_trait::<u8>("u8");
    test_type_def_trait::<u16>("u16");
    test_type_def_trait::<u32>("u32");
    test_type_def_trait::<u64>("u64");
    test_type_def_trait::<u128>("u128");
    test_type_def_trait::<u256>("u256");
    test_type_def_trait::<u512>("u512");
    test_type_def_trait::<i8>("i8");
    test_type_def_trait::<i16>("i16");
    test_type_def_trait::<i32>("i32");
    test_type_def_trait::<i64>("i64");
    test_type_def_trait::<i128>("i128");

    test_type_def_trait::<bytes31>("bytes31");
    test_type_def_trait::<ByteArray>("ByteArray");

    test_type_def_trait::<ContractAddress>("ContractAddress");
    test_type_def_trait::<ClassHash>("ClassHash");
    test_type_def_trait::<EthAddress>("EthAddress");
    test_type_def_trait::<Option<felt252>>("Option<felt252>");
    test_type_def_trait::<Result<felt252, Span<felt252>>>("Result<felt252, Span<felt252>>");
    test_type_def_trait::<Nullable<felt252>>("Nullable<felt252>");
    test_type_def_trait::<Box<felt252>>("Box<felt252>");
    test_type_def_trait::<Nullable<felt252>>("Nullable<felt252>");
    test_type_def_trait::<Array<felt252>>("Array<felt252>");
    test_type_def_trait::<Span<felt252>>("Span<felt252>");
    test_type_def_trait::<[felt252; 0]>("[felt252; 0]");
    test_type_def_trait::<[felt252; 1]>("[felt252; 1]");
    test_type_def_trait::<[felt252; 2]>("[felt252; 2]");
    test_type_def_trait::<[felt252; 3]>("[felt252; 3]");
    test_type_def_trait::<[felt252; 16]>("[felt252; 16]");
    test_type_def_trait::<()>("()");
    test_type_def_trait::<(felt252,)>("(felt252,)");
    test_type_def_trait::<(felt252, u8)>("(felt252, u8)");
    test_type_def_trait::<(felt252, u8, u16)>("(felt252, u8, u16)");
    test_type_def_trait::<(felt252, u8, u16, ByteArray)>("(felt252, u8, u16, ByteArray)");
    test_type_def_trait::<MyStruct>("MyStruct");
    test_type_def_trait::<MyOtherStruct, false>("MyOtherStruct");
    test_type_def_trait::<MyEnum>("MyEnum");
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


