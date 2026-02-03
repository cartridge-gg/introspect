use core::integer::u512;
use core::nullable::null;
use introspect_types::{ChildDef, ChildDefs, ISerde, TypeDef, structured};
use starknet::{ClassHash, ContractAddress, EthAddress};


fn make_my_struct_def() -> structured::TypeDef {
    structured::TypeDef::Struct(
        structured::StructDef {
            name: "MyStruct",
            attributes: [].span(),
            members: [
                structured::MemberDef {
                    name: "a",
                    attributes: [].span(),
                    type_def: structured::TypeDef::Array(
                        structured::TypeDef::Array(structured::TypeDef::Felt252.into()).into(),
                    ),
                },
                structured::MemberDef {
                    name: "b",
                    attributes: [].span(),
                    type_def: structured::TypeDef::Option(structured::TypeDef::Felt252.into()),
                },
                structured::MemberDef {
                    name: "c", attributes: [].span(), type_def: structured::TypeDef::Utf8String,
                },
                structured::MemberDef {
                    name: "d",
                    attributes: [].span(),
                    type_def: structured::TypeDef::Tuple(
                        [
                            structured::TypeDef::U8, structured::TypeDef::U16,
                            structured::TypeDef::U32,
                        ]
                            .span(),
                    ),
                },
            ]
                .span(),
        },
    )
}

#[derive(Drop, Table)]
struct MyStruct {
    a: Array<Span<felt252>>,
    b: Option<felt252>,
    c: ByteArray,
    id: (u8, u16, u32),
    e: (u8, u16, u32),
    f: (u8, u16, u32),
}

impl MyStructAsRef of introspect_types::m_utils::DefaultToRef<MyStruct>;
impl Tuple_U8U16U32_AsRef of introspect_types::m_utils::DefaultToRef<(u8, u16, u32)>;


#[derive(Drop)]
enum MyEnum {
    Variant1,
    Variant2: u8,
    Variant3: (felt252, felt252, Option<felt252>),
}


impl MyStruct_a_Def =
    introspect_types::m_utils::FieldDef<
        Array<Span<felt252>>,
        _,
        { [0x0301000000000000000000000000000000000000000000000000000000000061, 0] },
    >;
impl MyStruct_b_Def =
    introspect_types::m_utils::FieldDef<
        Option<felt252>,
        _,
        { [0x0301000000000000000000000000000000000000000000000000000000000062, 0] },
    >;
impl MyStruct_c_Def =
    introspect_types::m_utils::FieldDef<
        ByteArray, _, { [0x0301000000000000000000000000000000000000000000000000000000000063, 0] },
    >;
impl MyStruct_d_Def =
    introspect_types::m_utils::FieldDef<
        (u8, u16, u32),
        _,
        { [0x0301000000000000000000000000000000000000000000000000000000000064, 0] },
        introspect_types::m_utils::AsInline,
    >;
pub impl MyStructTypeDef of introspect_types::m_utils::CompoundDef<MyStruct, 1, 0> {
    const DEF_SELECTOR: felt252 = 'struct';
    const NAME: [felt252; 1] = [0x0308000000000000000000000000000000000000000000004d79537472756374];
    const ATTRIBUTES_COUNT: u32 = 0;
    const ATTRIBUTES: [felt252; 0] = [];
    const FIELDS_COUNT: u32 = 4;
    fn serialize_fields(ref output: Array<felt252>) {
        MyStruct_a_Def::serialize(ref output);
        MyStruct_b_Def::serialize(ref output);
        MyStruct_c_Def::serialize(ref output);
        MyStruct_d_Def::serialize(ref output);
    }
    fn collect_field_children(ref children: introspect_types::m_utils::ChildDefs) {
        MyStruct_a_Def::collect_children(ref children);
        MyStruct_b_Def::collect_children(ref children);
        MyStruct_c_Def::collect_children(ref children);
        MyStruct_d_Def::collect_children(ref children);
    }
    fn serialize_fields_with_children(
        ref type_def: Array<felt252>, ref children: introspect_types::m_utils::ChildDefs,
    ) {
        MyStruct_a_Def::serialize_with_children(ref type_def, ref children);
        MyStruct_b_Def::serialize_with_children(ref type_def, ref children);
        MyStruct_c_Def::serialize_with_children(ref type_def, ref children);
        MyStruct_d_Def::serialize_with_children(ref type_def, ref children);
    }
}

impl MyEnum_Variant1_Def =
    introspect_types::m_utils::FieldDef<
        (),
        _,
        {
            [
                0x0156f5ce477021d6111ecb5192d2b252a0bbdb2d3ee7d2a0aaceb5a2077ee46a,
                0x03080000000000000000000000000000000000000000000056617269616e7431, 0,
            ]
        },
    >;
impl MyEnum_Variant2_Def =
    introspect_types::m_utils::FieldDef<
        u8,
        _,
        {
            [
                0x00661273ab485bcba4e353e1e878530864b9c295d01d1d297fa4626f24604c0a,
                0x03080000000000000000000000000000000000000000000056617269616e7432, 0,
            ]
        },
    >;
impl MyEnum_Variant3_Def =
    introspect_types::m_utils::FieldDef<
        (felt252, felt252, Option<felt252>),
        _,
        {
            [
                0x02f4bf2d068042c60eec062e9b09dd41231cba010b7cf12fabf3ba39c9de8dd4,
                0x03080000000000000000000000000000000000000000000056617269616e7433, 0,
            ]
        },
    >;
pub impl MyEnumTypeDef of introspect_types::m_utils::CompoundDef<MyEnum, 1, 0> {
    const DEF_SELECTOR: felt252 = 'enum';
    const NAME: [felt252; 1] = [0x03060000000000000000000000000000000000000000000000004d79456e756d];
    const ATTRIBUTES_COUNT: u32 = 0;
    const ATTRIBUTES: [felt252; 0] = [];
    const FIELDS_COUNT: u32 = 3;
    fn serialize_fields(ref output: Array<felt252>) {
        MyEnum_Variant1_Def::serialize(ref output);
        MyEnum_Variant2_Def::serialize(ref output);
        MyEnum_Variant3_Def::serialize(ref output);
    }
    fn collect_field_children(ref children: introspect_types::m_utils::ChildDefs) {
        MyEnum_Variant1_Def::collect_children(ref children);
        MyEnum_Variant2_Def::collect_children(ref children);
        MyEnum_Variant3_Def::collect_children(ref children);
    }
    fn serialize_fields_with_children(
        ref type_def: Array<felt252>, ref children: introspect_types::m_utils::ChildDefs,
    ) {
        MyEnum_Variant1_Def::serialize_with_children(ref type_def, ref children);
        MyEnum_Variant2_Def::serialize_with_children(ref type_def, ref children);
        MyEnum_Variant3_Def::serialize_with_children(ref type_def, ref children);
    }
}


fn test_type_def_trait<T, impl TD: TypeDef<T>>(expected: ByteArray) {
    let mut output: Array<felt252> = Default::default();
    let mut children = Default::default();
    TD::serialize_with_children(ref output, ref children);
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
    if children.len() > 0 {
        for ChildDef { id, type_def: mut data } in children {
            println!("Child ID: {:?}", id);
            match ISerde::<structured::TypeDef>::ideserialize(ref data) {
                Some(type_def) => { println!("{:?}", type_def); },
                None => {
                    println!("{:?}", output);
                    panic!("Failed to deserialize type def");
                },
            }
        }
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

#[inline]
pub fn serialise_column_with_meta<
    const ID: felt252, const SIZE: u32, const DATA: [felt252; SIZE], T, impl TD: TypeDef<T>,
>(
    ref type_def: Array<felt252>, ref children: ChildDefs,
) {
    type_def.append_span(DATA.span());
    TD::serialize_with_children(ref type_def, ref children);
}

#[test]
fn test() {
    let mut type_def: Array<felt252> = Default::default();
    let mut children: ChildDefs = Default::default();
    const DATA: [felt252; 4] = [1, 2, 3, 69];
    serialise_column_with_meta::<123, _, { [1, 2, 3] }, felt252>(ref type_def, ref children);
    println!("{type_def:?}");
    println!("{DATA:?}")
}
