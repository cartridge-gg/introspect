use cgg_utils::{CollectionSplit, IsTuple};
use core::integer::u512;
use core::poseidon::poseidon_hash_span;
use starknet::storage_access::StorageBaseAddress;
use starknet::{ClassHash, ContractAddress, EthAddress, StorageAddress};
use crate::type_def::selectors;


#[derive(Drop)]
pub struct ChildDef {
    pub id: felt252,
    pub type_def: Span<felt252>,
}

pub type ChildDefs = Array<ChildDef>;

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


#[derive(PartialEq, Debug, Drop)]
pub enum KnownType {
    Primitive: felt252,
    Ref: felt252,
    None,
}

pub fn is_known_type_def(type_def_span: Span<felt252>) -> KnownType {
    match type_def_span.len() {
        0 => panic!("Type Def cannot be empty"),
        1 => KnownType::Primitive(*type_def_span[0]),
        2 => match *type_def_span[0] {
            'ref' => KnownType::Ref(*type_def_span[1]),
            _ => KnownType::None,
        },
        _ => KnownType::None,
    }
}

pub fn hash_type_def_span(type_def_span: Span<felt252>) -> felt252 {
    match is_known_type_def(type_def_span) {
        KnownType::Primitive(id) | KnownType::Ref(id) => id,
        KnownType::None => poseidon_hash_span(type_def_span),
    }
}

pub trait TypeDefTrait<T, const REF: bool> {
    const SIZE: u32;
    fn serialize_type_def(ref output: Array<felt252>);
    fn collect_child_defs(ref defs: ChildDefs) {}
    fn serialize_defs(
        ref output: Array<felt252>, ref defs: ChildDefs,
    ) {
        Self::serialize_type_def(ref output);
    }
    fn serialized_type_def() -> Span<
        felt252,
    > {
        let mut output: Array<felt252> = Default::default();
        Self::serialize_type_def(ref output);
        output.span()
    }
}

pub enum AsReference {
    Default,
    True,
    False,
}

impl TypeTraitRef<T, impl TD: TypeDefTrait<T, true>> of TypeDefTrait<T, false> {
    const SIZE: u32 = 2;
    fn serialize_type_def(ref output: Array<felt252>) {
        let type_def = TD::serialized_type_def();
        match is_known_type_def(type_def) {
            KnownType::Primitive(id) => { output.append(id); },
            KnownType::Ref(id) => {
                output.append(selectors::Ref);
                output.append(id);
            },
            KnownType::None => {
                output.append(selectors::Ref);
                output.append(hash_type_def_span(type_def));
            },
        }
    }
    fn collect_child_defs(ref defs: ChildDefs) {
        Self::collect_child_defs(ref defs);
        let type_def = TD::serialized_type_def();
        if is_known_type_def(type_def) == KnownType::None {
            let id = hash_type_def_span(type_def);
            add_child_def(ref defs, id, type_def);
        }
    }
    fn serialize_defs(ref output: Array<felt252>, ref defs: ChildDefs) {
        let type_def = TD::serialized_type_def();

        match is_known_type_def(type_def) {
            KnownType::Primitive(id) => { output.append(id); },
            KnownType::Ref(id) => {
                output.append(selectors::Ref);
                output.append(id);
            },
            KnownType::None => {
                output.append(selectors::Ref);
                let id = hash_type_def_span(type_def);
                output.append(id);
                defs.append(ChildDef { id, type_def });
            },
        }
    }
}

pub mod impls {
    use super::ChildDefs;
    pub impl Primary<T, const ID: felt252> of super::TypeDefTrait<T, false> {
        const SIZE: u32 = 1;
        fn serialize_type_def(ref output: Array<felt252>) {
            output.append(ID);
        }
    }

    pub impl Nested<
        const SELECTOR: felt252, Wrapper, Inner, impl TD: super::TypeDefTrait<Inner, false>,
    > of super::TypeDefTrait<Wrapper, false> {
        const SIZE: u32 = TD::SIZE + 1;
        fn serialize_type_def(ref output: Array<felt252>) {
            output.append(SELECTOR);
            TD::serialize_type_def(ref output);
        }
        fn collect_child_defs(ref defs: ChildDefs) {
            TD::collect_child_defs(ref defs);
        }
        fn serialize_defs(ref output: Array<felt252>, ref defs: ChildDefs) {
            TD::serialize_defs(ref output, ref defs);
        }
    }
}
pub impl VoidDef = impls::Primary<(), selectors::None>;
pub impl Felt252Def = impls::Primary<felt252, selectors::felt252>;
pub impl BoolDef = impls::Primary<bool, selectors::bool>;
pub impl U8Def = impls::Primary<u8, selectors::u8>;
pub impl U16Def = impls::Primary<u16, selectors::u16>;
pub impl U32Def = impls::Primary<u32, selectors::u32>;
pub impl U64Def = impls::Primary<u64, selectors::u64>;
pub impl U128Def = impls::Primary<u128, selectors::u128>;
pub impl U256Def = impls::Primary<u256, selectors::u256>;
pub impl U512Def = impls::Primary<u512, selectors::u512>;
pub impl I8Def = impls::Primary<i8, selectors::i8>;
pub impl I16Def = impls::Primary<i16, selectors::i16>;
pub impl I32Def = impls::Primary<i32, selectors::i32>;
pub impl I64Def = impls::Primary<i64, selectors::i64>;
pub impl I128Def = impls::Primary<i128, selectors::i128>;
pub impl Bytes31Def = impls::Primary<bytes31, selectors::bytes31>;
pub impl ClassHashDef = impls::Primary<ClassHash, selectors::ClassHash>;
pub impl ContractAddressDef = impls::Primary<ContractAddress, selectors::ContractAddress>;
pub impl EthAddressDef = impls::Primary<EthAddress, selectors::EthAddress>;
pub impl StorageAddressDef = impls::Primary<StorageAddress, selectors::StorageAddress>;
pub impl StorageBaseAddressDef = impls::Primary<StorageBaseAddress, selectors::StorageBaseAddress>;
pub impl Utf8StringDef = impls::Primary<ByteArray, selectors::Utf8String>;

impl FixedArrayDef<
    T, const SIZE: u32, impl TD: TypeDefTrait<T, false>,
> of TypeDefTrait<[T; SIZE], false> {
    const SIZE: u32 = TD::SIZE + 2;
    fn serialize_type_def(ref output: Array<felt252>) {
        output.append(selectors::FixedArray);
        TD::serialize_type_def(ref output);
        output.append(SIZE.into());
    }
    fn collect_child_defs(ref defs: ChildDefs) {
        TD::collect_child_defs(ref defs);
    }
}
impl ArrayDef<T, impl TD: TypeDefTrait<T, false>> =
    impls::Nested<selectors::Array, Array<T>, T, TD>;
impl SpanDef<T, impl TD: TypeDefTrait<T, false>> = impls::Nested<selectors::Array, Span<T>, T, TD>;
impl OptionDef<T, impl TD: TypeDefTrait<T, false>> =
    impls::Nested<selectors::Option, Option<T>, T, TD>;
impl NullableDef<T, impl TD: TypeDefTrait<T, false>> =
    impls::Nested<selectors::Nullable, Nullable<T>, T, TD>;


impl BoxDef<T, impl TD: TypeDefTrait<T, false>> of TypeDefTrait<Box<T>, false> {
    const SIZE: u32 = TD::SIZE;
    fn serialize_type_def(ref output: Array<felt252>) {
        TD::serialize_type_def(ref output);
    }
    fn collect_child_defs(ref defs: ChildDefs) {
        TD::collect_child_defs(ref defs);
    }
}

impl ResultDef<
    T, E, impl TD: TypeDefTrait<T, false>, impl ED: TypeDefTrait<E, false>,
> of TypeDefTrait<Result<T, E>, false> {
    const SIZE: u32 = TD::SIZE + ED::SIZE + 1;
    fn serialize_type_def(ref output: Array<felt252>) {
        output.append(selectors::Result);
        TD::serialize_type_def(ref output);
        ED::serialize_type_def(ref output);
    }
    fn collect_child_defs(ref defs: ChildDefs) {
        TD::collect_child_defs(ref defs);
        ED::collect_child_defs(ref defs);
    }
}


// pub fn append_member<
//     const NAME_SIZE: u32,
//     const ATTRIBUTES_SIZE: u32,
//     impl MD: MemberDef<NAME_SIZE, ATTRIBUTES_SIZE>,
//     impl NameToSpan: ToSpanTrait<[felt252; NAME_SIZE], felt252>,
//     impl AttrsToSpan: ToSpanTrait<[felt252; ATTRIBUTES_SIZE], felt252>,
//     impl TypeDef: TypeDefTrait<MD::Type, false>,
// >(
//     ref output: Array<felt252>,
// ) {
//     output.append_span(NameToSpan::span(@MD::NAME));
//     output.append(MD::ATTRIBUTES_COUNT.into());
//     output.append_span(AttrsToSpan::span(@MD::ATTRIBUTES));
//     TypeDef::serialize_type_def(ref output);
// }
// pub fn append_member_as_ref<
//     const SIZE: u32,
//     const DATA: [felt252; SIZE],
//     T,
//     impl ToSpan: ToSpanTrait<[felt252; SIZE], felt252>,
//     impl TypeDef: TypeDefTrait<T, true>,
// >(
//     ref output: Array<felt252>,
// ) {
//     output.append_span(ToSpan::span(@DATA));
//     TypeDef::serialize_type_def(ref output);
// }

// pub trait Member<T> {
//     fn serialize_member(ref output: Array<felt252>);
// }

// TODO: Fix once compiler supports fixed array sizes as size 0
// pub trait MemberDef<const NAME_SIZE: u32, const ATTRIBUTES_SIZE: u32> {
//     const NAME_SIZE: u32;
//     const NAME: [felt252; NAME_SIZE];
//     const ATTRIBUTES_COUNT: u32;
//     const ATTRIBUTES_SIZE: u32;
//     const ATTRIBUTES: [felt252; ATTRIBUTES_SIZE];
//     type Type;
//     // const fn SIZE<
//     //     impl TD: TypeDefTrait<Self::Type, false>,
//     // >() -> u32 {
//     //     Self::NAME_SIZE + Self::ATTRIBUTES_SIZE + 2 + TypeDefTrait::<Self::Type>::SIZE
//     // }
//     fn serialize_member<impl TD: TypeDefTrait<Self::Type, false>>(
//         ref output: Array<felt252>,
//     ) {
//         output.append_span(Self::NAME.span());
//         output.append(Self::ATTRIBUTES_COUNT.into());
//         // output.append_span(Self::ATTRIBUTES.span());
//         TD::serialize_type_def(ref output);
//     }
//     fn collect_member_child_defs<impl TD: TypeDefTrait<Self::Type, false>>(
//         ref defs: ChildDefs,
//     ) {
//         TD::collect_child_defs(ref defs);
//     }
// }

pub trait MemberDef<const META_SIZE: u32> {
    const META_SIZE: u32;
    const META_DATA: [felt252; META_SIZE];
    type Type;
    const fn SIZE<
        impl TD: TypeDefTrait<Self::Type, false>,
    >() -> u32 {
        Self::META_SIZE + TypeDefTrait::<Self::Type>::SIZE
    }
    fn serialize_member<impl TD: TypeDefTrait<Self::Type, false>>(
        ref output: Array<felt252>,
    ) {
        output.append_span(Self::META_DATA.span());
        TD::serialize_type_def(ref output);
    }
    fn collect_member_child_defs<impl TD: TypeDefTrait<Self::Type, false>>(
        ref defs: ChildDefs,
    ) {
        TD::collect_child_defs(ref defs);
    }
    fn serialize_member_defs<impl TD: TypeDefTrait<Self::Type, false>>(
        ref output: Array<felt252>, ref defs: ChildDefs,
    ) {
        output.append_span(Self::META_DATA.span());
        TD::serialize_defs(ref output, ref defs);
    }
}

pub trait StructDef<T, const NAME_SIZE: u32, const ATTRIBUTES_SIZE: u32> {
    const NAME: [felt252; NAME_SIZE];
    const ATTRIBUTES_COUNT: u32;
    const ATTRIBUTES: [felt252; ATTRIBUTES_SIZE];
    const MEMBERS_COUNT: u32;
    const MEMBERS_SIZE: u32;
    const REF: bool;
    fn serialize_members(ref output: Array<felt252>);
    fn collect_child_defs(ref defs: ChildDefs) {}
    fn serialize_defs(ref output: Array<felt252>, ref defs: ChildDefs);
}


pub trait Struct<T> {
    const SIZE: u32;
    fn serialize_struct(ref output: Array<felt252>);
    fn serialize_members(ref output: Array<felt252>);
    fn collect_child_defs(ref defs: ChildDefs);
}

// pub trait Member<T> {
//     const SIZE: u32;
//     fn serialize_member(ref output: Array<felt252>);
// }

// impl StructImpl<
//     T,
//     const NAME_SIZE: u32,
//     const ATTRIBUTES_SIZE: u32,
//     impl S: StructDef<T, NAME_SIZE, ATTRIBUTES_SIZE>,
//     impl NameToSpan: ToSpanTrait<[felt252; NAME_SIZE], felt252>,
//     impl AttrsToSpan: ToSpanTrait<[felt252; ATTRIBUTES_SIZE], felt252>,
// > of Struct<T> {
//     const SIZE: u32 = S::MEMBERS_SIZE + NAME_SIZE + ATTRIBUTES_SIZE + 2;
//     fn serialize_struct(ref output: Array<felt252>) {
//         output.append(selectors::Struct);
//         output.append_span(NameToSpan::span(@S::NAME));
//         output.append(S::ATTRIBUTES_COUNT.into());
//         output.append_span(AttrsToSpan::span(@S::ATTRIBUTES));
//         output.append(S::MEMBERS_COUNT.into());
//         Self::serialize_members(ref output);
//     }
//     fn serialize_members(ref output: Array<felt252>) {
//         S::serialize_members(ref output);
//     }
//     fn collect_child_defs(ref defs: ChildDefs) {
//         S::collect_child_defs(ref defs);
//     }
// }

impl StructDefImpl<
    T,
    const NAME_SIZE: u32,
    const ATTRIBUTES_SIZE: u32,
    impl S: StructDef<T, NAME_SIZE, ATTRIBUTES_SIZE>,
    impl NameToSpan: ToSpanTrait<[felt252; NAME_SIZE], felt252>,
    impl AttrsToSpan: ToSpanTrait<[felt252; ATTRIBUTES_SIZE], felt252>,
> of TypeDefTrait<T, S::REF> {
    const SIZE: u32 = S::MEMBERS_SIZE + NAME_SIZE + ATTRIBUTES_SIZE + 2;
    fn serialize_type_def(ref output: Array<felt252>) {
        output.append(selectors::Struct);
        output.append_span(NameToSpan::span(@S::NAME));
        output.append(S::ATTRIBUTES_COUNT.into());
        output.append_span(AttrsToSpan::span(@S::ATTRIBUTES));
        output.append(S::MEMBERS_COUNT.into());
        S::serialize_members(ref output);
    }
    fn collect_child_defs(ref defs: ChildDefs) {
        S::collect_child_defs(ref defs);
    }
    fn serialize_defs(ref output: Array<felt252>, ref defs: ChildDefs) {
        output.append(selectors::Struct);
        output.append_span(NameToSpan::span(@S::NAME));
        output.append(S::ATTRIBUTES_COUNT.into());
        output.append_span(AttrsToSpan::span(@S::ATTRIBUTES));
        output.append(S::MEMBERS_COUNT.into());
        S::serialize_defs(ref output, ref defs);
    }
}

// impl StructDefImpl<T, impl S: Struct<T>> of TypeDefTrait<T, false> {
//     const SIZE: u32 = S::SIZE;
//     fn serialize_type_def(ref output: Array<felt252>) {
//         S::serialize_struct(ref output);
//     }
//     fn collect_child_defs(ref defs: ChildDefs) {
//         S::collect_child_defs(ref defs);
//     }
// }

pub trait Variant<const META_SIZE: u32> {
    const SELECTOR: felt252;
    const META_SIZE: u32;
    const META_DATA: [felt252; META_SIZE];
    type Type;
    const fn SIZE<
        impl TD: TypeDefTrait<Self::Type, false>,
    >() -> u32 {
        1 + Self::META_SIZE + TypeDefTrait::<Self::Type>::SIZE
    }
    fn serialize_member<impl TD: TypeDefTrait<Self::Type, false>>(
        ref output: Array<felt252>,
    ) {
        output.append(Self::SELECTOR);
        output.append_span(Self::META_DATA.span());
        TD::serialize_type_def(ref output);
    }
    fn collect_member_child_defs<impl TD: TypeDefTrait<Self::Type, false>>(
        ref defs: ChildDefs,
    ) {
        TD::collect_child_defs(ref defs);
    }
    fn serialize_member_defs<impl TD: TypeDefTrait<Self::Type, false>>(
        ref output: Array<felt252>, ref defs: ChildDefs,
    ) {
        output.append(Self::SELECTOR);
        output.append_span(Self::META_DATA.span());
        TD::serialize_defs(ref output, ref defs);
    }
}

// pub trait Variant {
//     impl TypeDef: TypeDefTrait;
//     const SELECTOR: felt252;
//     const NAME_SIZE: u32;
//     const NAME: [felt252; Self::NAME_SIZE];
//     const ATTRIBUTES_SIZE: u32;
//     const ATTRIBUTES_COUNT: u32;
//     const ATTRIBUTES: [felt252; Self::ATTRIBUTES_SIZE];
//     const HASH: felt252;
//     fn serialize_variant(
//         ref output: Array<felt252>,
//     ) {
//         Self::serialize_variant_selector(ref output);
//         Self::serialize_variant_name(ref output);
//         Self::serialize_variant_attributes(ref output);
//         Self::serialize_variant_type_def(ref output);
//     }
//     fn serialize_variant_selector(ref output: Array<felt252>) {
//         output.append(Self::SELECTOR);
//     }
//     fn serialize_variant_name<impl NameToSpan: ToSpanTrait<[felt252; Self::NAME_SIZE], felt252>>(
//         ref output: Array<felt252>,
//     ) {
//         output.append_span(NameToSpan::span(@Self::NAME));
//     }
//     fn serialize_variant_attributes<
//         impl AttrsToSpan: ToSpanTrait<[felt252; Self::ATTRIBUTES_SIZE], felt252>,
//     >(
//         ref output: Array<felt252>,
//     ) {
//         output.append(Self::ATTRIBUTES_COUNT.into());
//         output.append_span(AttrsToSpan::span(@Self::ATTRIBUTES));
//     }
//     fn serialize_variant_type_def(
//         ref output: Array<felt252>,
//     ) {
//         Self::TypeDef::serialize_type_def(ref output);
//     }
// }

pub trait Enum<T> {
    const NAME_SIZE: u32;
    const NAME: [felt252; Self::NAME_SIZE];
    const ATTRIBUTES_SIZE: u32;
    const ATTRIBUTES_COUNT: u32;
    const ATTRIBUTES: [felt252; Self::ATTRIBUTES_SIZE];
    const VARIANTS_COUNT: u32;
    const SIZE: u32;
    fn serialize_enum(
        ref output: Array<felt252>,
    ) {
        output.append(selectors::Enum);
        Self::serialize_enum_name(ref output);
        Self::serialize_enum_attributes(ref output);
        output.append(Self::VARIANTS_COUNT.into());
        Self::serialize_variants(ref output);
    }
    fn serialize_enum_name<impl NameToSpan: ToSpanTrait<[felt252; Self::NAME_SIZE], felt252>>(
        ref output: Array<felt252>,
    ) {
        output.append_span(NameToSpan::span(@Self::NAME));
    }
    fn serialize_enum_attributes<
        impl AttrsToSpan: ToSpanTrait<[felt252; Self::ATTRIBUTES_SIZE], felt252>,
    >(
        ref output: Array<felt252>,
    ) {
        output.append(Self::ATTRIBUTES_COUNT.into());
        output.append_span(AttrsToSpan::span(@Self::ATTRIBUTES));
    }
    fn serialize_variants(ref output: Array<felt252>);
    fn collect_child_defs(ref defs: ChildDefs);
}


impl EnumDefImpl<T, impl E: Enum<T>> of TypeDefTrait<T, false> {
    const SIZE: u32 = E::SIZE;
    fn serialize_type_def(ref output: Array<felt252>) {
        E::serialize_enum(ref output);
    }
    fn collect_child_defs(ref defs: ChildDefs) {
        E::collect_child_defs(ref defs);
    }
}
impl TupleDefImpl<T, impl TD: TupleDefTrait<T>, impl Tuple: IsTuple<T>> of TypeDefTrait<T, false> {
    const SIZE: u32 = TD::SIZE + 2;
    fn serialize_type_def(ref output: Array<felt252>) {
        output.append(selectors::Tuple);
        output.append(Tuple::SIZE.into());
        TD::serialize_tuple_def(ref output);
    }
    fn collect_child_defs(ref defs: ChildDefs) {
        TD::collect_tuple_child_defs(ref defs);
    }
    fn serialize_defs(ref output: Array<felt252>, ref defs: ChildDefs) {
        TD::serialize_tuple_def(ref output);
    }
}
trait TupleDefTrait<T> {
    const SIZE: u32;
    fn serialize_tuple_def(ref output: Array<felt252>);
    fn collect_tuple_child_defs(ref defs: ChildDefs);
    fn serialize_defs(ref output: Array<felt252>, ref defs: ChildDefs);
}
impl TupleSize1Impl<T, impl HD: TypeDefTrait<T, false>> of TupleDefTrait<(T,)> {
    const SIZE: u32 = HD::SIZE;
    fn serialize_tuple_def(ref output: Array<felt252>) {
        HD::serialize_type_def(ref output);
    }
    fn collect_tuple_child_defs(ref defs: ChildDefs) {
        HD::collect_child_defs(ref defs);
    }
    fn serialize_defs(ref output: Array<felt252>, ref defs: ChildDefs) {
        HD::serialize_defs(ref output, ref defs);
    }
}
impl NextTupleDefImpl<
    T,
    impl CS: CollectionSplit<T>,
    impl HD: TypeDefTrait<CS::Head, false>,
    impl RD: TupleDefTrait<CS::Rest>,
> of TupleDefTrait<T> {
    const SIZE: u32 = HD::SIZE + RD::SIZE;
    fn serialize_tuple_def(ref output: Array<felt252>) {
        HD::serialize_type_def(ref output);
        RD::serialize_tuple_def(ref output);
    }
    fn collect_tuple_child_defs(ref defs: ChildDefs) {
        HD::collect_child_defs(ref defs);
        RD::collect_tuple_child_defs(ref defs);
    }
    fn serialize_defs(ref output: Array<felt252>, ref defs: ChildDefs) {
        HD::serialize_defs(ref output, ref defs);
        RD::serialize_defs(ref output, ref defs);
    }
}
// pub trait TIntrospect<T> {
//     fn type_def() -> TypeDef;
//     fn serialize_type_def(ref output: Array<felt252>);
//     fn type_id() -> felt252 {
//         hash_type_def(@Self::type_def())
//     }
//     fn collect_child_defs(ref defs: ChildDefs) {}
// }

// pub trait Introspect {
//     type Type;
//     const DEF_SIZE: u32;
//     const TYPE_DEF: [felt252; Self::DEF_SIZE];
//     const ID: felt252;
//     fn type_def() -> TypeDef;
//     fn serialize_type_def(ref output: Array<felt252>);
//     fn type_id() -> felt252 {
//         hash_type_def(@Self::type_def())
//     }
//     fn collect_child_defs(ref defs: ChildDefs) {}
// }

// impl ISpecTImpl<T, impl I: IntrospectT<T>> of Introspect {
//     type Type = T;
//     fn type_def() -> TypeDef {
//         I::type_def()
//     }
//     fn serialize_type_def(ref output: Array<felt252>) {
//         I::serialize_type_def(ref output);
//     }
//     fn type_id() -> felt252 {
//         I::type_id()
//     }
//     fn collect_child_defs(ref defs: ChildDefs) {
//         I::collect_child_defs(ref defs);
//     }
// }

// #[generate_trait]
// pub impl ISpecEnumImpl of ISpecEnum {
//     fn append_enum_meta(ref self: Array<felt252>, name: ByteArray, attributes: Span<Attribute>) {
//         self.append(selectors::Enum);
//         name.iserialize(ref self);
//         attributes.iserialize(ref self);
//     }
//     fn append_variant_meta<impl I: Introspect>(
//         ref self: Array<felt252>, selector: felt252, name: ByteArray,
//     ) {
//         self.append(selector);
//         name.iserialize(ref self);
//     }
//     fn append_variant<impl I: Introspect>(
//         ref self: Array<felt252>, selector: felt252, name: ByteArray, attributes:
//         Span<Attribute>,
//     ) {
//         self.append(selector);
//         name.iserialize(ref self);
//         attributes.iserialize(ref self);
//         I::serialize_type_def(ref self);
//     }
// }

// #[generate_trait]
// pub impl ISpecStructImpl of ISpecStruct {
//     fn append_struct_meta(ref self: Array<felt252>, name: ByteArray, attributes: Span<Attribute>)
//     {
//         self.append(selectors::Struct);
//         name.iserialize(ref self);
//         attributes.iserialize(ref self);
//     }
//     fn serialize_bare_member<impl I: Introspect>(ref self: Array<felt252>, name: ByteArray) {
//         name.iserialize(ref self);
//         self.append(0);
//         I::serialize_type_def(ref self);
//     }
// }

// // impl ChildDefISerde of ISerde<ChildDef> {
// //     fn iserialize(self: @ChildDef, ref output: Array<felt252>) {
// //         output.append(*self.id);
// //         output.append_span(*self.type_def);
// //     }

// //     fn ideserialize(ref serialized: Span<felt252>) -> Option<ChildDef> {
// //         let id = *serialized.pop_front()?;
// //         let type_def = ISerde::ideserialize(ref serialized)?;
// //         Some(ChildDef { id, type_def })
// //     }
// // }

// pub impl ISpecRefImpl<T, impl IR: IntrospectRef<T>> of IntrospectT<T> {
//     fn type_def() -> TypeDef {
//         TypeDef::Ref(Self::type_id())
//     }
//     fn serialize_type_def(ref output: Array<felt252>) {
//         output.append(selectors::Ref);
//         output.append(Self::type_id());
//     }
//     fn type_id() -> felt252 {
//         hash_type_def(@IR::ref_type_def())
//     }
//     fn collect_child_defs(ref defs: ChildDefs) {
//         IR::collect_ref_child_defs(ref defs);
//         let type_def_span = IR::ref_type_def().iserialize_inline();
//         add_child_def(ref defs, poseidon_hash_span(type_def_span), type_def_span)
//     }
// }

// pub fn hash_type_def(type_def: @TypeDef) -> felt252 {
//     hash_type_def_span(type_def.iserialize_inline())
// }

// pub fn hash_type_def_span(type_def_span: Span<felt252>) -> felt252 {
//     poseidon_hash_span(type_def_span)
// }

// pub mod primitive_impl {
//     use crate::TypeDef;
//     use super::ISpecT;
//     pub impl PrimitiveSize1ISpec<T, const TY: TypeDef, const ID: felt252> of ISpecT<T>
//     {
//         const SIZE: Option<u32> = Some(1);
//         fn type_def() -> TypeDef {
//             TY
//         }
//         fn serialize_type_def(ref output: Array<felt252>) {
//             output.append(ID);
//         }
//         fn type_id() -> felt252 {
//             ID
//         }
//     }
//     pub impl PrimitiveISpec<
//         T, const TY: TypeDef, const ID: felt252, const SIZE: u32,
//     > of ISpecT<T> {
//         const SIZE: Option<u32> = Some(SIZE);
//         fn type_def() -> TypeDef {
//             TY
//         }
//         fn serialize_type_def(ref output: Array<felt252>) {
//             output.append(ID);
//         }
//         fn type_id() -> felt252 {
//             ID
//         }
//     }
// }
// pub mod empty_impl {
//     use crate::TypeDef;
//     use super::ISpecT;
//     pub impl EmptyISpec<T> of ISpecT<T> {
//         const SIZE: Option<u32> = Some(0);
//         fn type_def() -> TypeDef {
//             TypeDef::None
//         }
//         fn serialize_type_def(ref output: Array<felt252>) {
//             output.append(0);
//         }
//         fn type_id() -> felt252 {
//             0
//         }
//     }
// }

// pub impl EmptyISpec<T> of Introspect {
//     type Type = T;
//     fn type_def() -> TypeDef {
//         TypeDef::None
//     }
//     fn serialize_type_def(ref output: Array<felt252>) {
//         output.append(0);
//     }
//     fn type_id() -> felt252 {
//         0
//     }
// }

// pub impl PrimitiveSize1ISpec<T, const TY: TypeDef, const ID: felt252> of Introspect {
//     type Type = T;
//     fn type_def() -> TypeDef {
//         TY
//     }
//     fn serialize_type_def(ref output: Array<felt252>) {
//         output.append(ID);
//     }
//     fn type_id() -> felt252 {
//         ID
//     }
// }

// pub impl PrimitiveISpec<T, const TY: TypeDef, const ID: felt252, const SIZE: u32> of Introspect {
//     type Type = T;
//     fn type_def() -> TypeDef {
//         TY
//     }
//     fn serialize_type_def(ref output: Array<felt252>) {
//         output.append(ID);
//     }
//     fn type_id() -> felt252 {
//         ID
//     }
// }

// pub impl Felt252ISpec = PrimitiveSize1ISpec<felt252, TypeDef::Felt252, selectors::felt252>;
// pub impl Bytes31ISpec = PrimitiveSize1ISpec<bytes31, TypeDef::Bytes31, selectors::bytes31>;
// pub impl BoolISpec = PrimitiveSize1ISpec<bool, TypeDef::Bool, selectors::bool>;
// pub impl U8ISpec = PrimitiveSize1ISpec<u8, TypeDef::U8, selectors::u8>;
// pub impl U16ISpec = PrimitiveSize1ISpec<u16, TypeDef::U16, selectors::u16>;
// pub impl U32ISpec = PrimitiveSize1ISpec<u32, TypeDef::U32, selectors::u32>;
// pub impl U64ISpec = PrimitiveSize1ISpec<u64, TypeDef::U64, selectors::u64>;
// pub impl U128ISpec = PrimitiveSize1ISpec<u128, TypeDef::U128, selectors::u128>;
// pub impl U256ISpec = PrimitiveISpec<u256, TypeDef::U256, selectors::u256, 2>;
// pub impl U512ISpec = PrimitiveISpec<u512, TypeDef::U512, selectors::u512, 4>;
// pub impl I8ISpec = PrimitiveSize1ISpec<i8, TypeDef::I8, selectors::i8>;
// pub impl I16ISpec = PrimitiveSize1ISpec<i16, TypeDef::I16, selectors::i16>;
// pub impl I32ISpec = PrimitiveSize1ISpec<i32, TypeDef::I32, selectors::i32>;
// pub impl I64ISpec = PrimitiveSize1ISpec<i64, TypeDef::I64, selectors::i64>;
// pub impl I128ISpec = PrimitiveSize1ISpec<i128, TypeDef::I128, selectors::i128>;
// pub impl ClassHashISpec = PrimitiveSize1ISpec<ClassHash, TypeDef::ClassHash,
// selectors::ClassHash>;
// pub impl ContractAddressISpec =
//     PrimitiveSize1ISpec<ContractAddress, TypeDef::ContractAddress, selectors::ContractAddress>;
// pub impl EthAddressISpec =
//     PrimitiveSize1ISpec<EthAddress, TypeDef::EthAddress, selectors::EthAddress>;
// pub impl StorageAddressISpec =
//     PrimitiveSize1ISpec<StorageAddress, TypeDef::StorageAddress, selectors::StorageAddress>;
// pub impl StorageBaseAddressISpec =
//     PrimitiveSize1ISpec<
//         StorageBaseAddress, TypeDef::StorageBaseAddress, selectors::StorageBaseAddress,
//     >;
// pub impl Utf8StringISpec =
//     PrimitiveSize1ISpec<ByteArray, TypeDef::Utf8String, selectors::Utf8String>;

// pub impl Tuple0ISpec = EmptyISpec<()>;

// pub impl ArrayISpec<T, impl I: Introspect> of Introspect {
//     type Type = Array<T>;
//     fn type_def() -> TypeDef {
//         TypeDef::Array(BoxTrait::new(I::type_def()))
//     }

//     fn serialize_type_def(ref output: Array<felt252>) {
//         output.append(selectors::Array);
//         I::serialize_type_def(ref output);
//     }

//     fn collect_child_defs(ref defs: ChildDefs) {
//         I::collect_child_defs(ref defs);
//     }
// }

// pub impl SpanISpec<T, impl I: Introspect> of Introspect {
//     type Type = Span<T>;
//     fn type_def() -> TypeDef {
//         TypeDef::Array(BoxTrait::new(I::type_def()))
//     }

//     fn serialize_type_def(ref output: Array<felt252>) {
//         output.append(selectors::Array);
//         I::serialize_type_def(ref output);
//     }

//     fn collect_child_defs(ref defs: ChildDefs) {
//         I::collect_child_defs(ref defs);
//     }
// }

// pub impl FixedArrayISpec<T, const SIZE: u32, impl I: Introspect> of Introspect {
//     type Type = [T; SIZE];
//     fn type_def() -> TypeDef {
//         TypeDef::FixedArray(BoxTrait::new(FixedArrayDef { type_def: I::type_def(), size: SIZE }))
//     }

//     fn serialize_type_def(ref output: Array<felt252>) {
//         output.append(selectors::FixedArray);
//         I::serialize_type_def(ref output);
//         output.append(SIZE.into());
//     }

//     fn collect_child_defs(ref defs: ChildDefs) {
//         I::collect_child_defs(ref defs);
//     }
// }

// pub impl BoxISpec<T, impl I: Introspect> of Introspect {
//     type Type = Box<T>;
//     fn type_def() -> TypeDef {
//         I::type_def()
//     }
//     fn serialize_type_def(ref output: Array<felt252>) {
//         I::serialize_type_def(ref output);
//     }
//     fn collect_child_defs(ref defs: ChildDefs) {
//         I::collect_child_defs(ref defs);
//     }
// }

// pub impl Tuple1ISpec<T0, impl I0: Introspect> of Introspect {
//     fn type_def() -> TypeDef {
//         TypeDef::Tuple([I0::type_def()].span())
//     }
//     fn collect_child_defs(ref defs: ChildDefs) {
//         I0::collect_child_defs(ref defs);
//     }
// }

// pub impl Tuple2ISpec<
//     T0, T1, impl I0: IntrospectT<T0>, impl I1: IntrospectT<T1>,
// > of IntrospectT<(T0, T1)> {
//     fn type_def() -> TypeDef {
//         TypeDef::Tuple([I0::type_def(), I1::type_def()].span())
//     }
//     fn collect_child_defs(ref defs: ChildDefs) {
//         I0::collect_child_defs(ref defs);
//         I1::collect_child_defs(ref defs);
//     }
// }

// pub impl Tuple3ISpec<
//     T0, T1, T2, impl I0: IntrospectT<T0>, impl I1: IntrospectT<T1>, impl I2: IntrospectT<T2>,
// > of IntrospectT<(T0, T1, T2)> {
//     fn type_def() -> TypeDef {
//         TypeDef::Tuple([I0::type_def(), I1::type_def(), I2::type_def()].span())
//     }
//     fn collect_child_defs(ref defs: ChildDefs) {
//         I0::collect_child_defs(ref defs);
//         I1::collect_child_defs(ref defs);
//         I2::collect_child_defs(ref defs);
//     }
// }

// pub impl Tuple4ISpec<
//     T0,
//     T1,
//     T2,
//     T3,
//     impl I0: IntrospectT<T0>,
//     impl I1: IntrospectT<T1>,
//     impl I2: IntrospectT<T2>,
//     impl I3: IntrospectT<T3>,
// > of IntrospectT<(T0, T1, T2, T3)> {
//     fn type_def() -> TypeDef {
//         TypeDef::Tuple([I0::type_def(), I1::type_def(), I2::type_def(), I3::type_def()].span())
//     }
//     fn collect_child_defs(ref defs: ChildDefs) {
//         I0::collect_child_defs(ref defs);
//         I1::collect_child_defs(ref defs);
//         I2::collect_child_defs(ref defs);
//         I3::collect_child_defs(ref defs);
//     }
// }

// pub impl Tuple5ISpec<
//     T0,
//     T1,
//     T2,
//     T3,
//     T4,
//     impl I0: IntrospectT<T0>,
//     impl I1: IntrospectT<T1>,
//     impl I2: IntrospectT<T2>,
//     impl I3: IntrospectT<T3>,
//     impl I4: IntrospectT<T4>,
// > of IntrospectT<(T0, T1, T2, T3, T4)> {
//     fn type_def() -> TypeDef {
//         TypeDef::Tuple(
//             [I0::type_def(), I1::type_def(), I2::type_def(), I3::type_def(),
//             I4::type_def()].span(),
//         )
//     }
//     fn collect_child_defs(ref defs: ChildDefs) {
//         I0::collect_child_defs(ref defs);
//         I1::collect_child_defs(ref defs);
//         I2::collect_child_defs(ref defs);
//         I3::collect_child_defs(ref defs);
//         I4::collect_child_defs(ref defs);
//     }
// }

// pub impl Tuple6ISpec<
//     T0,
//     T1,
//     T2,
//     T3,
//     T4,
//     T5,
//     impl I0: IntrospectT<T0>,
//     impl I1: IntrospectT<T1>,
//     impl I2: IntrospectT<T2>,
//     impl I3: IntrospectT<T3>,
//     impl I4: IntrospectT<T4>,
//     impl I5: IntrospectT<T5>,
// > of IntrospectT<(T0, T1, T2, T3, T4, T5)> {
//     fn type_def() -> TypeDef {
//         TypeDef::Tuple(
//             [
//                 I0::type_def(), I1::type_def(), I2::type_def(), I3::type_def(), I4::type_def(),
//                 I5::type_def(),
//             ]
//                 .span(),
//         )
//     }
//     fn collect_child_defs(ref defs: ChildDefs) {
//         I0::collect_child_defs(ref defs);
//         I1::collect_child_defs(ref defs);
//         I2::collect_child_defs(ref defs);
//         I3::collect_child_defs(ref defs);
//         I4::collect_child_defs(ref defs);
//         I5::collect_child_defs(ref defs);
//     }
// }

// pub impl OptionTISpec<T, impl I: IntrospectT<T>> of IntrospectT<Option<T>> {
//     const SIZE: Option<u32> = I::SIZE.add_size(1);
//     fn type_def() -> TypeDef {
//         TypeDef::Option(BoxTrait::new(I::type_def()))
//     }
//     fn serialize_type_def(ref output: Array<felt252>) {
//         output.append(selectors::Option);
//         I::serialize_type_def(ref output);
//     }

//     fn collect_child_defs(ref defs: ChildDefs) {
//         I::collect_child_defs(ref defs);
//     }
// }

// pub impl ResultTEISpec<
//     T, E, impl IT: IntrospectT<T>, impl IE: IntrospectT<E>,
// > of IntrospectT<Result<T, E>> {
//     const SIZE: Option<u32> = IT::SIZE.add_size_option(IE::SIZE).add_size(1);
//     fn type_def() -> TypeDef {
//         TypeDef::Result(BoxTrait::new(ResultDef { ok: IT::type_def(), err: IE::type_def() }))
//     }
//     fn serialize_type_def(ref output: Array<felt252>) {
//         output.append(selectors::Result);
//         IT::serialize_type_def(ref output);
//         IE::serialize_type_def(ref output);
//     }
//     fn collect_child_defs(ref defs: ChildDefs) {
//         IT::collect_child_defs(ref defs);
//         IE::collect_child_defs(ref defs);
//     }
// }

// pub impl CallISpec of IntrospectT<starknet::account::Call> {
//     const SIZE: Option<u32> = None;
//     fn type_def() -> TypeDef {
//         TypeDef::Struct(
//             StructDef {
//                 name: "Call",
//                 attributes: [].span(),
//                 members: [
//                     MemberDefTrait::new::<ContractAddress>("to", [].span()),
//                     MemberDefTrait::new::<felt252>("selector", [].span()),
//                     MemberDefTrait::new::<Span<felt252>>("calldata", [].span()),
//                 ]
//                     .span(),
//             },
//         )
//     }

//     fn serialize_type_def(ref output: Array<felt252>) {
//         output.append_struct_meta("Call", [].span());
//         output.serialize_bare_member::<ISpecContractAddress>("to", [].span());
//         output.serialize_member::<felt252>("selector", [].span());
//         output.serialize_member::<Span<felt252>>("calldata", [].span());
//     }
// }

// pub impl BlockInfoISpec of IntrospectT<starknet::BlockInfo> {
//     fn type_def() -> TypeDef {
//         TypeDef::Struct(
//             StructDef {
//                 name: "BlockInfo",
//                 attributes: [].span(),
//                 members: [
//                     MemberDefTrait::new::<felt252>("block_hash", [].span()),
//                     MemberDefTrait::new::<u64>("block_number", [].span()),
//                     MemberDefTrait::new::<u64>("block_timestamp", [].span()),
//                     MemberDefTrait::new::<ContractAddress>("sequencer_address", [].span()),
//                 ]
//                     .span(),
//             },
//         )
//     }
// }

// pub impl ResourceBoundsISpec of IntrospectT<starknet::ResourcesBounds> {
//     fn type_def() -> TypeDef {
//         TypeDef::Struct(
//             StructDef {
//                 name: "ResourceBounds",
//                 attributes: [].span(),
//                 members: [
//                     MemberDefTrait::new::<felt252>("resource", [].span()),
//                     MemberDefTrait::new::<u64>("max_amount", [].span()),
//                     MemberDefTrait::new::<u128>("max_price_per_unit", [].span()),
//                 ]
//                     .span(),
//             },
//         )
//     }
// }

// pub impl TxInfoV2ISpec of IntrospectT<starknet::TxInfo> {
//     fn type_def() -> TypeDef {
//         TypeDef::Struct(
//             StructDef {
//                 name: "TxInfo",
//                 attributes: [].span(),
//                 members: [
//                     MemberDefTrait::new::<felt252>("version", [].span()),
//                     MemberDefTrait::new::<ContractAddress>("account_contract_address",
//                     [].span()), MemberDefTrait::new::<u128>("max_fee", [].span()),
//                     MemberDefTrait::new::<Span<felt252>>("signature", [].span()),
//                     MemberDefTrait::new::<felt252>("transaction_hash", [].span()),
//                     MemberDefTrait::new::<felt252>("chain_id", [].span()),
//                     MemberDefTrait::new::<felt252>("nonce", [].span()),
//                     MemberDefTrait::new::<
//                         Span<starknet::ResourcesBounds>,
//                     >("resource_bounds", [].span()),
//                     MemberDefTrait::new::<u128>("tip", [].span()),
//                     MemberDefTrait::new::<Span<felt252>>("paymaster_data", [].span()),
//                     MemberDefTrait::new::<u32>("nonce_data_availability_mode", [].span()),
//                     MemberDefTrait::new::<u32>("fee_data_availability_mode", [].span()),
//                     MemberDefTrait::new::<Span<felt252>>("account_deployment_data", [].span()),
//                 ]
//                     .span(),
//             },
//         )
//     }
// }


