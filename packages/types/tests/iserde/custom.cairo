use introspect_tests::introspect::{
    AStruct, AStructWithNone, AStructWithOne, ArmourPiece, ArmourSet, ArmourSetIds, DojoStoreModel,
    DojoStoreModelSubset, DojoStoreModelWithEnumKey, Element, EnumKey, EnumPackedWithTuples,
    EnumWithCommentOnLastVariant, EnumWithTuples, EnumWithUnitType, Foo, Foo2, Foo3, Foo4,
    FooSchema, LegacyModel, LegacyModelSubset, LegacyModelWithEnumKey, MapPosition, Material,
    ModelWithCommentOnLastFied, ModelWithFixedArray, ModelWithUnitType, Mountain, MyEnum,
    MyEnumLegacy, StructPackedWithTuples, StructWithOptionWithTuple, StructWithTuples,
    StructWithUnitType, Terrain, TestEnum, TestStruct, WarriorTable, Water, Weapon, WeaponHit,
    WeaponType, Weather,
};


fn test_introspect_iserde<>() {
    let instance = T::default();
    let mut serialized = instance.iserialize_inline();
    assert_eq!(
        ISerde::ideserialize(ref serialized).unwrap(),
        instance,
        "Deserialized doesnt match",
    );
}