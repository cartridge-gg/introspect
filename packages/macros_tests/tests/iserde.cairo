#[derive(Drop, ISerde, TypeDef)]
struct MyStruct<T> {
    a: T,
    b: Option<felt252>,
}


#[derive(Drop, ISerde, TypeDef)]
enum MyEnum<T> {
    Variant1,
    Variant2: u8,
    Variant3: (T, felt252, Option<T>),
}

pub impl MyStructTypeDef<
    T, +introspect::m_utils::TypeDef<T, false>,
> of introspect::m_utils::StructDef<MyStruct<T>, 1, 0> {
    const NAME: [felt252; 1] = [0x0308000000000000000000000000000000000000000000004d79537472756374];
    const ATTRIBUTES_COUNT: u32 = 0;
    const ATTRIBUTES: [felt252; 0] = [];
    const MEMBERS_COUNT: u32 = 2;
    const REF: bool = false;
    fn serialize_members(ref output: Array<felt252>) {
        MyStructMember::a::<T>::serialize(ref output);
        MyStructMember::b::<T>::serialize(ref output);
    }
    fn collect_member_children(ref children: introspect::m_utils::ChildDefs) {
        MyStructMember::a::<T>::collect_children(ref children);
        MyStructMember::b::<T>::collect_children(ref children);
    }
    fn serialize_members_with_children(
        ref type_def: Array<felt252>, ref children: introspect::m_utils::ChildDefs,
    ) {
        MyStructMember::a::<T>::serialize_with_children(ref type_def, ref children);
        MyStructMember::b::<T>::serialize_with_children(ref type_def, ref children);
    }
}
mod MyStructMember {
    pub impl a<T, +introspect::m_utils::TypeDef<T, false>> of introspect::m_utils::MemberDef<2> {
        const META_DATA: [felt252; 2] = [
            0x0301000000000000000000000000000000000000000000000000000000000061, 0,
        ];
        type Type = T;
    }
    pub impl b<T, +introspect::m_utils::TypeDef<T, false>> of introspect::m_utils::MemberDef<2> {
        const META_DATA: [felt252; 2] = [
            0x0301000000000000000000000000000000000000000000000000000000000062, 0,
        ];
        type Type = Option<felt252>;
    }
}

pub impl MyEnumTypeDef<
    T, +introspect::m_utils::TypeDef<T, false>,
> of introspect::m_utils::EnumDef<MyEnum<T>, 1, 0> {
    const NAME: [felt252; 1] = [0x03060000000000000000000000000000000000000000000000004d79456e756d];
    const ATTRIBUTES_COUNT: u32 = 0;
    const ATTRIBUTES: [felt252; 0] = [];
    const VARIANTS_COUNT: u32 = 3;
    const REF: bool = false;
    fn serialize_variants(ref output: Array<felt252>) {
        MyEnumVariant::Variant1::<T>::serialize(ref output);
        MyEnumVariant::Variant2::<T>::serialize(ref output);
        MyEnumVariant::Variant3::<T>::serialize(ref output);
    }
    fn collect_variant_children(ref children: introspect::m_utils::ChildDefs) {
        MyEnumVariant::Variant1::<T>::collect_children(ref children);
        MyEnumVariant::Variant2::<T>::collect_children(ref children);
        MyEnumVariant::Variant3::<T>::collect_children(ref children);
    }
    fn serialize_variants_with_children(
        ref type_def: Array<felt252>, ref children: introspect::m_utils::ChildDefs,
    ) {
        MyEnumVariant::Variant1::<T>::serialize_with_children(ref type_def, ref children);
        MyEnumVariant::Variant2::<T>::serialize_with_children(ref type_def, ref children);
        MyEnumVariant::Variant3::<T>::serialize_with_children(ref type_def, ref children);
    }
}
mod MyEnumVariant {
    pub impl Variant1 of introspect::m_utils::VariantDef<2> {
        const SELECTOR: felt252 =
            0x0156f5ce477021d6111ecb5192d2b252a0bbdb2d3ee7d2a0aaceb5a2077ee46a;
        const META_DATA: [felt252; 2] = [
            0x03080000000000000000000000000000000000000000000056617269616e7431, 0,
        ];
        type Type = ();
    }
    pub impl Variant2 of introspect::m_utils::VariantDef<2> {
        const SELECTOR: felt252 =
            0x00661273ab485bcba4e353e1e878530864b9c295d01d1d297fa4626f24604c0a;
        const META_DATA: [felt252; 2] = [
            0x03080000000000000000000000000000000000000000000056617269616e7432, 0,
        ];
        type Type = u8;
    }
    pub impl Variant3 of introspect::m_utils::VariantDef<2> {
        const SELECTOR: felt252 =
            0x02f4bf2d068042c60eec062e9b09dd41231cba010b7cf12fabf3ba39c9de8dd4;
        const META_DATA: [felt252; 2] = [
            0x03080000000000000000000000000000000000000000000056617269616e7433, 0,
        ];
        type Type = (T, felt252, Option<T>);
    }
}
