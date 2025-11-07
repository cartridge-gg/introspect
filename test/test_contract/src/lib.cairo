#[derive(Introspect, Schema)]
struct TestStruct<T, S> {
    #[key]
    #[default]
    pub value: Span<T>,
    pub value2: (felt252, S),
}


// #[derive(Introspect)]
// enum TestEnum {
//     Variant1,
//     Variant2: TestStruct<felt252, felt252>,
// }
#[derive(Copy, Drop, Serde, IntrospectRef, Schema, Debug, PartialEq)]
struct Foo {
    #[key]
    k1: u8,
    #[key]
    k2: felt252,
    v1: u128,
    v2: u32,
}
// #[derive(Copy, Drop, Serde, Debug, Introspect, Schema)]
// struct Foo2 {
//     #[key]
//     k1: u8,
//     #[key]
//     k2: felt252,
//     v1: u128,
//     v2: u32,
// }

// #[derive(Copy, Drop, Serde, Debug, Introspect, Schema)]
// struct Foo3 {
//     #[key]
//     k1: u256,
//     #[key]
//     k2: felt252,
//     v1: u128,
//     v2: u32,
// }

// #[derive(Copy, Drop, Serde, Debug, IntrospectRef, Schema)]
// struct AStruct {
//     a: u8,
//     b: u8,
//     c: u8,
//     d: u8,
// }

// #[derive(Copy, Drop, Serde, Debug, Introspect, Schema)]
// struct Foo4 {
//     #[key]
//     id: felt252,
//     v0: u256,
//     v1: felt252,
//     v2: u128,
//     v3: AStruct,
// }
// #[derive(Copy, Drop, Serde, Debug, Introspect, Schema)]
// struct FooSchema {
//     v0: u256,
//     v3: AStruct,
// }
// // to test the issue https://github.com/dojoengine/dojo/issues/3199
// #[derive(Copy, Drop, Serde, Debug, Introspect, Schema)]
// struct ModelWithCommentOnLastFied {
//     #[key]
//     k1: u8,
//     v1: Span<u32> // a comment without a comma
// }

// #[derive(Copy, Drop, Serde, Debug, Introspect, Default, PartialEq)]
// enum EnumWithCommentOnLastVariant {
//     #[default]
//     X: u8,
//     Y: Span<u32> // a comment without a comma
// }
#[introspect(variants_as_names)]
#[derive(Copy, Drop, Debug, Introspect, Default, PartialEq)]
enum MyEnumLegacy {
    X: Option<u32>,
    Y: (u8, u32),
    #[default]
    Z,
}


// #[derive(Copy, Drop, Serde, Debug, Introspect, Schema, PartialEq)]
// struct LegacyModel {
//     #[key]
//     a: u8,
//     b: (u8, u32),
//     c: Option<u32>,
//     d: MyEnumLegacy,
// }

// #[derive(Copy, Drop, Serde, Debug, Introspect, Default, PartialEq)]
// enum MyEnum {
//     X: Option<u32>,
//     Y: (u8, u32),
//     #[default]
//     Z,
// }

// #[derive(Copy, Drop, Serde, Debug, Introspect, Schema, PartialEq)]
// struct DojoStoreModel {
//     #[key]
//     a: u8,
//     b: (u8, u32),
//     c: Option<u32>,
//     d: MyEnum,
// }

// #[derive(Copy, Drop, Serde, Introspect, Default, Debug, PartialEq)]
// enum EnumKey {
//     #[default]
//     KEY_1,
//     KEY_2,
//     KEY_3,
// }

// #[derive(Copy, Drop, Debug, Introspect, Schema, PartialEq)]
// struct LegacyModelWithEnumKey {
//     #[key]
//     k1: u8,
//     #[key]
//     k2: EnumKey,
//     v1: u32,
//     v2: Option<u32>,
//     v3: MyEnumLegacy,
// }

// #[derive(Copy, Drop, Serde, Introspect, Schema, Debug, PartialEq)]
// struct LegacyModelSubset {
//     v2: Option<u32>,
//     v3: MyEnumLegacy,
// }

// #[derive(Copy, Drop, Debug, PartialEq)]
// struct DojoStoreModelWithEnumKey {
//     #[key]
//     k1: u8,
//     #[key]
//     k2: EnumKey,
//     v1: u32,
//     v2: Option<u32>,
//     v3: MyEnum,
// }

// #[derive(Copy, Drop, Serde, Introspect, Schema, Debug, PartialEq)]
// struct DojoStoreModelSubset {
//     v2: Option<u32>,
//     v3: MyEnum,
// }

// // to test with unit types
// #[derive(Copy, Drop, Introspect, Debug, Serde, PartialEq, Default)]
// enum EnumWithUnitType {
//     #[default]
//     X: u8,
//     Y,
//     Z: (),
// }

// #[derive(Copy, Drop, Introspect, Schema, Debug, Serde, PartialEq)]
// struct StructWithUnitType {
//     x: (),
// }

// #[derive(Copy, Drop, Serde, Debug, PartialEq)]
// struct ModelWithUnitType {
//     #[key]
//     k: u8,
//     x: StructWithUnitType,
//     y: EnumWithUnitType,
//     z: (),
//     a: ((), (u8, ())),
// }
// #[derive(Introspect, Schema, Serde, Drop, Default)]
// struct StructWithTuples {
//     x: (u8, u16, u32),
//     y: Array<(u128, u128)>,
//     z: (u8, (u16, Option<u32>), (), u32),
// }

#[derive(Introspect, Serde, Drop, Default)]
enum EnumWithTuples {
    #[default]
    A: (u8, u16, u32),
    B: Array<(u128, u128)>,
    C: (u8, (u16, Option<u32>), (), u32),
}
// #[derive(Introspect, Schema, Serde, Drop, Default)]
// struct StructPackedWithTuples {
//     x: (u8, u16, u32),
//     y: (u8, (u16, u32), (), u32),
// }

// #[derive(Introspect, Serde, Drop, Default)]
// enum EnumPackedWithTuples {
//     #[default]
//     A: (u8, (u16, u32), (), u32),
//     B: (u8, (u16, u32), (), u32),
// }

// // To test Option with tuple
// #[derive(Introspect, Schema, Serde, Drop, Default)]
// struct StructWithOptionWithTuple {
//     #[key]
//     k: u8,
//     x: Option<(u8, u16)>,
//     y: Option<u32>,
// }
// #[derive(Introspect, Schema, Serde, Drop, Default)]
// struct ModelWithFixedArray {
//     #[key]
//     // test a comment, here
//     k1: u8,
//     v1: [u16; 3],
// }
// #[derive(Introspect, Schema, Serde, Drop, Default)]
// struct AStructWithNone {}

// #[derive(Introspect, Schema, Serde, Drop, Default)]
// struct AStructWithOne {
//     a: u8,
// }

pub impl MyEnumLegacyIntrospectImpl of introspect::Introspect<MyEnumLegacy> {
    fn type_def() -> introspect::types::TypeDef {
        introspect::types::TypeDef::Enum(
            introspect::types::EnumDef {
                name: "MyEnumLegacy",
                attrs: [].span(),
                variants: [
                    introspect::types::VariantDef {
                        selector: 0,
                        name: "X",
                        attrs: [].span(),
                        type_def: introspect::Introspect::<Option<u32>>::type_def(),
                    },
                    introspect::types::VariantDef {
                        selector: 1,
                        name: "Y",
                        attrs: [].span(),
                        type_def: introspect::Introspect::<(u8, u32)>::type_def(),
                    },
                    introspect::types::VariantDef {
                        selector: 2,
                        name: "Z",
                        attrs: [].span(),
                        type_def: introspect::TypeDef::None,
                    },
                ]
                    .span(),
            },
        )
    }

    fn child_defs() -> Array<(felt252, introspect::types::TypeDef)> {
        introspect::types::introspect::merge_defs(
            array![
                introspect::Introspect::<Option<u32>>::child_defs(),
                introspect::Introspect::<(u8, u32)>::child_defs(),
            ],
        )
    }
}
pub impl EnumWithTuplesIntrospectImpl of introspect::Introspect<EnumWithTuples> {
    fn type_def() -> introspect::types::TypeDef {
        introspect::types::TypeDef::Enum(
            introspect::types::EnumDef {
                name: "EnumWithTuples",
                attrs: [].span(),
                variants: [
                    introspect::types::VariantDef {
                        selector: 0,
                        name: "A",
                        attrs: [].span(),
                        type_def: introspect::Introspect::<(u8, u16, u32)>::type_def(),
                    },
                    introspect::types::VariantDef {
                        selector: 1,
                        name: "B",
                        attrs: [].span(),
                        type_def: introspect::Introspect::<Array<(u128, u128)>>::type_def(),
                    },
                    introspect::types::VariantDef {
                        selector: 2,
                        name: "C",
                        attrs: [].span(),
                        type_def: introspect::Introspect::<
                            (u8, (u16, Option<u32>), (), u32),
                        >::type_def(),
                    },
                ]
                    .span(),
            },
        )
    }

    fn child_defs() -> Array<(felt252, introspect::types::TypeDef)> {
        introspect::types::introspect::merge_defs(
            array![
                introspect::Introspect::<(u8, u16, u32)>::child_defs(),
                introspect::Introspect::<Array<(u128, u128)>>::child_defs(),
                introspect::Introspect::<(u8, (u16, Option<u32>), (), u32)>::child_defs(),
            ],
        )
    }
}
