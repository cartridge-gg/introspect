use core::fmt::Debug;

#[derive(Debug, Default, Introspect, Fuzzable)]
pub struct TestStruct<T, S> {
    #[key]
    pub value: Span<T>,
    pub value2: (felt252, S),
}
#[derive(Debug, Default, Introspect, Fuzzable)]
pub enum TestEnum<T> {
    #[default]
    Variant1,
    Variant2: TestStruct<T, felt252>,
}


pub impl GenTestStructFuzzableImpl<
    T,
    S,
    +core::fmt::Debug<T>,
    +snforge_std::fuzzable::Fuzzable<T>,
    +core::fmt::Debug<S>,
    +snforge_std::fuzzable::Fuzzable<S>,
> of snforge_std::fuzzable::Fuzzable<TestStruct<T, S>> {
    fn blank() -> TestStruct<T, S> {
        Default::default()
    }

    fn generate() -> TestStruct<T, S> {
        TestStruct {
            value: snforge_std::fuzzable::Fuzzable::generate(),
            value2: snforge_std::fuzzable::Fuzzable::generate(),
        }
    }
}

pub impl GenTestEnumFuzzableImpl<
    T, +core::fmt::Debug<T>, +snforge_std::fuzzable::Fuzzable<T>,
> of snforge_std::fuzzable::Fuzzable<TestEnum<T>> {
    fn blank() -> TestEnum<T> {
        Default::default()
    }

    fn generate() -> TestEnum<T> {
        match snforge_std::fuzzable::generate_arg(0_u32, 1) {
            0 => TestEnum::Variant1,
            1 => TestEnum::Variant2(snforge_std::fuzzable::Fuzzable::generate()),
            _ => Default::default(),
        }
    }
}

#[derive(Copy, Drop, Serde, IntrospectRef, Debug, PartialEq)]
pub struct Foo {
    #[key]
    k1: u8,
    #[key]
    k2: felt252,
    v1: u128,
    v2: u32,
}
#[derive(Copy, Drop, Serde, Debug, Introspect)]
pub struct Foo2 {
    #[key]
    k1: u8,
    #[key]
    k2: felt252,
    v1: u128,
    v2: u32,
}

#[derive(Copy, Drop, Serde, Debug, Introspect)]
pub struct Foo3 {
    #[key]
    k1: u256,
    #[key]
    k2: felt252,
    v1: u128,
    v2: u32,
}

#[derive(Copy, Drop, Serde, Debug, IntrospectRef)]
pub struct AStruct {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

#[derive(Copy, Drop, Serde, Debug, Introspect)]
pub struct Foo4 {
    #[key]
    id: felt252,
    v0: u256,
    v1: felt252,
    v2: u128,
    v3: AStruct,
}
#[derive(Copy, Drop, Serde, Debug, Introspect)]
pub struct FooSchema {
    v0: u256,
    v3: AStruct,
}
// to test the issue https://github.com/dojoengine/dojo/issues/3199
#[derive(Copy, Drop, Serde, Debug, Introspect)]
pub struct ModelWithCommentOnLastFied {
    #[key]
    k1: u8,
    v1: Span<u32> // a comment without a comma
}

#[derive(Copy, Drop, Serde, Debug, Introspect, Default, PartialEq)]
pub enum EnumWithCommentOnLastVariant {
    #[default]
    X: u8,
    Y: Span<u32> // a comment without a comma
}

#[derive(Copy, Drop, Serde, Debug, Introspect, Default, PartialEq)]
pub enum MyEnumLegacy<T> {
    X: Option<u32>,
    Y: (T, u32),
    #[default]
    Z,
}

#[derive(Copy, Drop, Serde, Debug, Introspect, PartialEq)]
pub struct LegacyModel<T> {
    #[key]
    a: u8,
    b: (u8, u32),
    c: Option<u32>,
    d: MyEnumLegacy<T>,
}

#[derive(Copy, Drop, Serde, Debug, Introspect, Default, PartialEq)]
pub enum MyEnum {
    X: Option<u32>,
    Y: (u8, u32),
    #[default]
    Z,
}

#[derive(Copy, Drop, Serde, Debug, Introspect, PartialEq)]
pub struct DojoStoreModel {
    #[key]
    a: u8,
    b: (u8, u32),
    c: Option<u32>,
    d: MyEnum,
}

#[derive(Copy, Drop, Serde, Introspect, Default, Debug, PartialEq)]
pub enum EnumKey {
    #[default]
    KEY_1,
    KEY_2,
    KEY_3,
}

#[derive(Copy, Drop, Debug, Introspect, PartialEq)]
pub struct LegacyModelWithEnumKey<T> {
    #[key]
    k1: u8,
    #[key]
    k2: EnumKey,
    v1: u32,
    v2: Option<u32>,
    v3: MyEnumLegacy<T>,
}

#[derive(Copy, Drop, Serde, Introspect, Debug, PartialEq)]
pub struct LegacyModelSubset<T> {
    v2: Option<u32>,
    v3: MyEnumLegacy<T>,
}

#[derive(Copy, Drop, Debug, Introspect, PartialEq)]
pub struct DojoStoreModelWithEnumKey {
    #[key]
    k1: u8,
    #[key]
    k2: EnumKey,
    v1: u32,
    v2: Option<u32>,
    v3: MyEnum,
}

#[derive(Copy, Drop, Serde, Introspect, Debug, PartialEq)]
pub struct DojoStoreModelSubset {
    v2: Option<u32>,
    v3: MyEnum,
}

// to test with unit types
#[derive(Copy, Drop, Introspect, Debug, Serde, PartialEq, Default)]
pub enum EnumWithUnitType {
    #[default]
    X: u8,
    Y,
    Z: (),
}

#[derive(Copy, Drop, Introspect, Debug, Serde, PartialEq)]
pub struct StructWithUnitType {
    x: (),
}

#[derive(Copy, Drop, Introspect, Debug, Serde, PartialEq)]
pub struct ModelWithUnitType {
    #[key]
    k: u8,
    x: StructWithUnitType,
    y: EnumWithUnitType,
    z: (),
    a: ((), (u8, ())),
}
#[derive(Introspect, Serde, Drop, Default)]
pub struct StructWithTuples {
    x: (u8, u16, u32),
    y: Array<(u128, u128)>,
    z: (u8, (u16, Option<u32>), (), u32),
}

#[derive(Introspect, Serde, Drop, Default)]
pub enum EnumWithTuples {
    #[default]
    A: (u8, u16, u32),
    B: Array<(u128, u128)>,
    C: (u8, (u16, Option<u32>), (), u32),
}
#[derive(Introspect, Serde, Drop, Default)]
pub struct StructPackedWithTuples {
    x: (u8, u16, u32),
    y: (u8, (u16, u32), (), u32),
}

#[derive(Introspect, Serde, Drop, Default)]
pub enum EnumPackedWithTuples {
    #[default]
    A: (u8, (u16, u32), (), u32),
    B: (u8, (u16, u32), (), u32),
}

// To test Option with tuple
#[derive(Introspect, Serde, Drop, Default)]
pub struct StructWithOptionWithTuple {
    #[key]
    k: u8,
    x: Option<(u8, u16)>,
    y: Option<u32>,
}
#[derive(Introspect, Serde, Drop, Default)]
pub struct ModelWithFixedArray {
    #[key]
    // test a comment, here
    k1: u8,
    v1: [u16; 3],
}
#[derive(Introspect, Serde, Drop, Default)]
pub struct AStructWithNone {}

#[derive(Introspect, Serde, Drop, Default)]
pub struct AStructWithOne {
    a: u8,
}

#[derive(Drop, Introspect, starknet::Store)]
pub enum Element {
    #[default]
    None,
    Air: u8,
    Fire: u8,
    Earth: u8,
    Water: u8,
}

#[derive(Drop, Introspect, starknet::Store)]
pub enum Material {
    #[default]
    Cloth,
    Leather,
    Iron,
    Steel,
    Mythril,
    Elemental: Element,
}

#[derive(Drop, Introspect, starknet::Store)]
pub struct ArmourPiece {
    experience: u32,
    wear: u8,
    material: Material,
}

#[derive(Drop, Introspect)]
pub enum WeaponType {
    Sword,
    Axe,
    Bow,
}

#[derive(Drop, Introspect)]
pub struct Weapon {
    name: ByteArray,
    level: u16,
    material: Material,
    weapon_type: WeaponType,
}

#[derive(Drop, Introspect)]
pub struct ArmourSet {
    head: ArmourPiece,
    chest: ArmourPiece,
    legs: ArmourPiece,
    gloves: ArmourPiece,
    boots: ArmourPiece,
}

#[derive(Drop, Introspect)]
pub struct WeaponHit {
    damage: u16,
    element: Element,
}

#[derive(Drop, Introspect)]
pub struct ArmourSetIds {
    head: felt252,
    chest: felt252,
    legs: felt252,
    gloves: felt252,
    boots: felt252,
}

#[derive(Drop, Introspect)]
pub struct WarriorTable {
    name: ByteArray,
    level: u8,
    health: u16,
    weapons: felt252,
    armour: ArmourSetIds,
    gold: u128,
    alive: bool,
}

#[derive(Drop, Introspect)]
pub struct Water {
    depth: u8,
    current_speed: u8,
    fish: bool,
}

#[derive(Drop, Introspect)]
pub struct Mountain {
    height: u32,
    trolls: u8,
}

#[derive(Drop, Introspect)]
pub enum Terrain {
    Grass,
    Water: Water,
    Mountain: Mountain,
    Desert,
}

#[derive(Drop, Introspect)]
pub struct MapPosition {
    x: u8,
    y: u8,
    terrain: Terrain,
    warrior: Option<felt252>,
}

#[derive(Drop, Introspect)]
pub struct Weather {
    wind: u8,
    temperature: i8,
}

