#[derive(Introspect, Schema, ISerde)]
struct TestStruct<T, S> {
    #[key]
    #[default]
    pub value: Span<T>,
    pub value2: (felt252, S),
}

#[derive(Introspect, ISerde)]
enum TestEnum {
    Variant1,
    Variant2: TestStruct<felt252, felt252>,
}

#[derive(Copy, Drop, Serde, ISerde, IntrospectRef, Schema, Debug, PartialEq)]
struct Foo {
    #[key]
    k1: u8,
    #[key]
    k2: felt252,
    v1: u128,
    v2: u32,
}
#[derive(Copy, Drop, Serde, ISerde, Debug, Introspect, Schema)]
struct Foo2 {
    #[key]
    k1: u8,
    #[key]
    k2: felt252,
    v1: u128,
    v2: u32,
}

#[derive(Copy, Drop, Serde, ISerde, Debug, Introspect, Schema)]
struct Foo3 {
    #[key]
    k1: u256,
    #[key]
    k2: felt252,
    v1: u128,
    v2: u32,
}

#[derive(Copy, Drop, Serde, ISerde, Debug, IntrospectRef, Schema)]
struct AStruct {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

#[derive(Copy, Drop, Serde, ISerde, Debug, Introspect, Schema)]
struct Foo4 {
    #[key]
    id: felt252,
    v0: u256,
    v1: felt252,
    v2: u128,
    v3: AStruct,
}
#[derive(Copy, Drop, Serde, ISerde, Debug, Introspect, Schema)]
struct FooSchema {
    v0: u256,
    v3: AStruct,
}
// to test the issue https://github.com/dojoengine/dojo/issues/3199
#[derive(Copy, Drop, Serde, ISerde, Debug, Introspect, Schema)]
struct ModelWithCommentOnLastFied {
    #[key]
    k1: u8,
    v1: Span<u32> // a comment without a comma
}

#[derive(Copy, Drop, Serde, ISerde, Debug, Introspect, Default, PartialEq)]
enum EnumWithCommentOnLastVariant {
    #[default]
    X: u8,
    Y: Span<u32> // a comment without a comma
}

#[derive(Copy, Drop, Serde, ISerde, Debug, Introspect, Default, PartialEq)]
enum MyEnumLegacy<T> {
    X: Option<u32>,
    Y: (T, u32),
    #[default]
    Z,
}

fn something<T, +Serde<T>, +Drop<T>>(blah: MyEnumLegacy<T>) {
    let mut output: Array<felt252> = Default::default();
    match blah {
        MyEnumLegacy::<T>::X(value) => value.serialize(ref output),
        MyEnumLegacy::<T>::Y(value) => value.serialize(ref output),
        MyEnumLegacy::<T>::Z(value) => value.serialize(ref output),
    };
}

#[derive(Copy, Drop, Serde, ISerde, Debug, Introspect, Schema, PartialEq)]
struct LegacyModel<T> {
    #[key]
    a: u8,
    b: (u8, u32),
    c: Option<u32>,
    d: MyEnumLegacy<T>,
}

#[derive(Copy, Drop, Serde, ISerde, Debug, Introspect, Default, PartialEq)]
enum MyEnum {
    X: Option<u32>,
    Y: (u8, u32),
    #[default]
    Z,
}

#[derive(Copy, Drop, Serde, ISerde, Debug, Introspect, Schema, PartialEq)]
struct DojoStoreModel {
    #[key]
    a: u8,
    b: (u8, u32),
    c: Option<u32>,
    d: MyEnum,
}

#[derive(Copy, Drop, Serde, ISerde, Introspect, Default, Debug, PartialEq)]
enum EnumKey {
    #[default]
    KEY_1,
    KEY_2,
    KEY_3,
}

#[derive(Copy, Drop, Debug, Introspect, Schema, PartialEq)]
struct LegacyModelWithEnumKey<T> {
    #[key]
    k1: u8,
    #[key]
    k2: EnumKey,
    v1: u32,
    v2: Option<u32>,
    v3: MyEnumLegacy<T>,
}

#[derive(Copy, Drop, Serde, ISerde, Introspect, Schema, Debug, PartialEq)]
struct LegacyModelSubset<T> {
    v2: Option<u32>,
    v3: MyEnumLegacy<T>,
}

#[derive(Copy, Drop, Debug, PartialEq)]
struct DojoStoreModelWithEnumKey {
    #[key]
    k1: u8,
    #[key]
    k2: EnumKey,
    v1: u32,
    v2: Option<u32>,
    v3: MyEnum,
}

#[derive(Copy, Drop, Serde, ISerde, Introspect, Schema, Debug, PartialEq)]
struct DojoStoreModelSubset {
    v2: Option<u32>,
    v3: MyEnum,
}

// to test with unit types
#[derive(Copy, Drop, Introspect, Debug, Serde, ISerde, PartialEq, Default)]
enum EnumWithUnitType {
    #[default]
    X: u8,
    Y,
    Z: (),
}

#[derive(Copy, Drop, Introspect, Schema, Debug, Serde, ISerde, PartialEq)]
struct StructWithUnitType {
    x: (),
}

#[derive(Copy, Drop, Serde, ISerde, Debug, PartialEq)]
struct ModelWithUnitType {
    #[key]
    k: u8,
    x: StructWithUnitType,
    y: EnumWithUnitType,
    z: (),
    a: ((), (u8, ())),
}
#[derive(Introspect, Schema, Serde, ISerde, Drop, Default)]
struct StructWithTuples {
    x: (u8, u16, u32),
    y: Array<(u128, u128)>,
    z: (u8, (u16, Option<u32>), (), u32),
}

#[derive(Introspect, Serde, ISerde, Drop, Default)]
enum EnumWithTuples {
    #[default]
    A: (u8, u16, u32),
    B: Array<(u128, u128)>,
    C: (u8, (u16, Option<u32>), (), u32),
}
#[derive(Introspect, Schema, Serde, ISerde, Drop, Default)]
struct StructPackedWithTuples {
    x: (u8, u16, u32),
    y: (u8, (u16, u32), (), u32),
}

#[derive(Introspect, Serde, ISerde, Drop, Default)]
enum EnumPackedWithTuples {
    #[default]
    A: (u8, (u16, u32), (), u32),
    B: (u8, (u16, u32), (), u32),
}

// To test Option with tuple
#[derive(Introspect, Schema, Serde, ISerde, Drop, Default)]
struct StructWithOptionWithTuple {
    #[key]
    k: u8,
    x: Option<(u8, u16)>,
    y: Option<u32>,
}
#[derive(Introspect, Schema, Serde, ISerde, Drop, Default)]
struct ModelWithFixedArray {
    #[key]
    // test a comment, here
    k1: u8,
    v1: [u16; 3],
}
#[derive(Introspect, Schema, Serde, ISerde, Drop, Default)]
struct AStructWithNone {}

#[derive(Introspect, Schema, Serde, ISerde, Drop, Default)]
struct AStructWithOne {
    a: u8,
}

#[derive(Drop, Introspect, starknet::Store)]
enum Element {
    #[default]
    None,
    Air: u8,
    Fire: u8,
    Earth: u8,
    Water: u8,
}

#[derive(Drop, Introspect, starknet::Store)]
enum Material {
    #[default]
    Cloth,
    Leather,
    Iron,
    Steel,
    Mythril,
    Elemental: Element,
}

#[derive(Drop, Introspect, starknet::Store)]
struct ArmourPiece {
    experience: u32,
    wear: u8,
    material: Material,
}


#[derive(Drop, Introspect)]
enum WeaponType {
    Sword,
    Axe,
    Bow,
}


#[derive(Drop, Schema)]
struct Weapon {
    name: ByteArray,
    level: u16,
    material: Material,
    weapon_type: WeaponType,
}


#[derive(Drop, Introspect)]
struct ArmourSet {
    head: ArmourPiece,
    chest: ArmourPiece,
    legs: ArmourPiece,
    gloves: ArmourPiece,
    boots: ArmourPiece,
}

#[derive(Drop, Introspect)]
struct WeaponHit {
    damage: u16,
    element: Element,
}

#[derive(Drop, Introspect)]
struct ArmourSetIds {
    head: felt252,
    chest: felt252,
    legs: felt252,
    gloves: felt252,
    boots: felt252,
}


#[derive(Drop, Schema)]
struct WarriorTable {
    name: ByteArray,
    level: u8,
    health: u16,
    weapons: felt252,
    armour: ArmourSetIds,
    gold: u128,
    alive: bool,
}

#[derive(Drop, Introspect)]
struct Water {
    depth: u8,
    current_speed: u8,
    fish: bool,
}

#[derive(Drop, Introspect)]
struct Mountain {
    height: u32,
    trolls: u8,
}

#[derive(Drop, Introspect)]
enum Terrain {
    Grass,
    Water: Water,
    Mountain: Mountain,
    Desert,
}

#[derive(Drop, Schema)]
struct MapPosition {
    x: u8,
    y: u8,
    terrain: Terrain,
    warrior: Option<felt252>,
}

#[derive(Drop, Introspect)]
struct Weather {
    wind: u8,
    temperature: i8,
}

#[starknet::contract]
mod a_contract {
    use starknet::storage::Map;
    use crate::ArmourPiece;
    use super::{MapPosition, Warrior};

    #[storage]
    struct Storage {
        map: Map<(u8, u8), MapPosition>,
        warriors: Map<felt252, Warrior>,
        armours: Map<felt252, ArmourPiece>,
    }
}
