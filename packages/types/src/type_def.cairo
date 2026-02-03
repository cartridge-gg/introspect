use cgg_utils::{CollectionSplit, IsTuple};
use core::integer::u512;
use core::poseidon::poseidon_hash_span;
use starknet::storage_access::StorageBaseAddress;
use starknet::{ClassHash, ContractAddress, EthAddress, StorageAddress};


pub trait TypeDef<T> {
    const SIZE: u32;
    fn serialize(ref output: Array<felt252>);
    fn collect_children(ref children: ChildDefs);
    fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs);
    fn to_span() -> Span<
        felt252,
    > {
        let mut output: Array<felt252> = Default::default();
        Self::serialize(ref output);
        output.span()
    }
    fn to_span_with_children(
        ref children: ChildDefs,
    ) -> Span<
        felt252,
    > {
        let mut output: Array<felt252> = Default::default();
        Self::serialize_with_children(ref output, ref children);
        output.span()
    }
}

pub trait DefaultToRef<T>;

pub trait TypeDefRef<T> {
    const SIZE: u32;
    fn serialize(ref output: Array<felt252>);
    fn collect_children(ref children: ChildDefs);
    fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs);
    fn to_span() -> Span<
        felt252,
    > {
        let mut output: Array<felt252> = Default::default();
        Self::serialize(ref output);
        output.span()
    }
    fn to_span_with_children(
        ref children: ChildDefs,
    ) -> Span<
        felt252,
    > {
        let mut output: Array<felt252> = Default::default();
        Self::serialize_with_children(ref output, ref children);
        output.span()
    }
}

pub trait TypeDefInline<T> {
    const SIZE: u32;
    fn serialize(ref output: Array<felt252>);
    fn collect_children(ref children: ChildDefs);
    fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs);
    fn to_span() -> Span<
        felt252,
    > {
        let mut output: Array<felt252> = Default::default();
        Self::serialize(ref output);
        output.span()
    }
    fn to_span_with_children(
        ref children: ChildDefs,
    ) -> Span<
        felt252,
    > {
        let mut output: Array<felt252> = Default::default();
        Self::serialize_with_children(ref output, ref children);
        output.span()
    }
}


impl TypeDefInlineImpl<T, impl TD: TypeDefInline<T>, -DefaultToRef<T>> of TypeDef<T> {
    const SIZE: u32 = TD::SIZE;
    fn serialize(ref output: Array<felt252>) {
        TD::serialize(ref output);
    }
    fn collect_children(ref children: ChildDefs) {
        TD::collect_children(ref children);
    }
    fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
        TD::serialize_with_children(ref type_def, ref children);
    }
}


impl RefDefDefaultImpl<T, impl TD: TypeDefRef<T>, +DefaultToRef<T>> of TypeDef<T> {
    const SIZE: u32 = TD::SIZE;
    fn serialize(ref output: Array<felt252>) {
        TD::serialize(ref output);
    }
    fn collect_children(ref children: ChildDefs) {
        TD::collect_children(ref children);
    }
    fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
        TD::serialize_with_children(ref type_def, ref children);
    }
}


impl TypeDefRefImpl<T, impl TD: TypeDefInline<T>> of TypeDefRef<T> {
    const SIZE: u32 = 2;
    fn serialize(ref output: Array<felt252>) {
        let type_def = TD::to_span();
        match is_known_type_def(type_def) {
            KnownType::Primitive(id) => { output.append(id); },
            KnownType::Ref(id) => {
                output.append(selectors::Ref);
                output.append(id);
            },
            KnownType::None => {
                output.append(selectors::Ref);
                output.append(poseidon_hash_span(type_def));
            },
        }
    }
    fn collect_children(ref children: ChildDefs) {
        TD::collect_children(ref children);
        let type_def = TD::to_span();
        if is_known_type_def(type_def) == KnownType::None {
            let id = poseidon_hash_span(type_def);
            children.add_child_def(id, type_def);
        }
    }
    fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
        let child_def = TD::to_span_with_children(ref children);
        match is_known_type_def(child_def) {
            KnownType::Primitive(id) => { type_def.append(id); },
            KnownType::Ref(id) => {
                type_def.append(selectors::Ref);
                type_def.append(id);
            },
            KnownType::None => {
                type_def.append(selectors::Ref);
                let id = poseidon_hash_span(child_def);
                type_def.append(id);
                children.append(ChildDef { id, type_def: child_def });
            },
        }
    }
}


#[generate_trait]
pub impl ChildDefsImpl of ChildDefsTrait {
    fn add_child_def(ref self: ChildDefs, hash: felt252, type_def_span: Span<felt252>) {
        for child in self.span() {
            if *child.id == hash {
                return;
            }
        }
        self.append(ChildDef { id: hash, type_def: type_def_span });
    }
}


#[derive(Drop)]
pub struct ChildDef {
    pub id: felt252,
    pub type_def: Span<felt252>,
}

pub type ChildDefs = Array<ChildDef>;

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

pub mod selectors {
    pub const None: core::felt252 = 0;
    pub const felt252: core::felt252 = 'felt252';
    pub const ShortUtf8: core::felt252 = 'short_utf8';
    pub const bytes31: core::felt252 = 'bytes31';
    pub const bytes31Encoded: core::felt252 = 'bytes31_encoded';
    pub const bool: core::felt252 = 'bool';
    pub const u8: core::felt252 = 'u8';
    pub const u16: core::felt252 = 'u16';
    pub const u32: core::felt252 = 'u32';
    pub const u64: core::felt252 = 'u64';
    pub const u128: core::felt252 = 'u128';
    pub const u256: core::felt252 = 'u256';
    pub const u512: core::felt252 = 'u512';
    pub const i8: core::felt252 = 'i8';
    pub const i16: core::felt252 = 'i16';
    pub const i32: core::felt252 = 'i32';
    pub const i64: core::felt252 = 'i64';
    pub const i128: core::felt252 = 'i128';
    pub const ClassHash: core::felt252 = 'class_hash';
    pub const ContractAddress: core::felt252 = 'contract_address';
    pub const EthAddress: core::felt252 = 'eth_address';
    pub const StorageAddress: core::felt252 = 'storage_address';
    pub const StorageBaseAddress: core::felt252 = 'storage_base_address';
    pub const ByteArray: core::felt252 = 'byte_array';
    pub const Utf8String: core::felt252 = 'utf8_string';
    pub const ByteArrayEncoded: core::felt252 = 'byte_array_encoded';
    pub const Tuple: core::felt252 = 'tuple';
    pub const Array: core::felt252 = 'array';
    pub const FixedArray: core::felt252 = 'fixed_array';
    pub const Felt252Dict: core::felt252 = 'felt252_dict';
    pub const Struct: core::felt252 = 'struct';
    pub const Enum: core::felt252 = 'enum';
    pub const Ref: core::felt252 = 'ref';
    pub const Custom: core::felt252 = 'custom';
    pub const Option: core::felt252 = 'option';
    pub const Result: core::felt252 = 'result';
    pub const Nullable: core::felt252 = 'nullable';
}


pub mod impls {
    use super::ChildDefs;
    pub impl Primary<T, const ID: felt252> of super::TypeDefInline<T> {
        const SIZE: u32 = 1;
        fn serialize(ref output: Array<felt252>) {
            output.append(ID);
        }
        fn collect_children(ref children: super::ChildDefs) {}
        fn serialize_with_children(ref type_def: Array<felt252>, ref children: super::ChildDefs) {
            Self::serialize(ref type_def);
        }
    }

    pub impl Nested<
        const SELECTOR: felt252, Wrapper, Inner, impl TD: super::TypeDef<Inner>,
    > of super::TypeDefInline<Wrapper> {
        const SIZE: u32 = TD::SIZE + 1;
        fn serialize(ref output: Array<felt252>) {
            output.append(SELECTOR);
            TD::serialize(ref output);
        }
        fn collect_children(ref children: ChildDefs) {
            TD::collect_children(ref children);
        }
        fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
            type_def.append(SELECTOR);
            TD::serialize_with_children(ref type_def, ref children);
        }
    }
}

pub mod custom {
    use super::{ChildDefs, TypeDef, TypeDefInline, TypeDefRef, impls, selectors};
    pub impl ByteArrayDef = impls::Primary<ByteArray, selectors::ByteArray>;
    pub impl Bytes31Def = impls::Primary<bytes31, selectors::bytes31>;
    pub impl ByteArrayEncoded<
        const SIZE: u32, const ENCODING: [felt252; SIZE],
    > of TypeDef<ByteArray> {
        const SIZE: u32 = SIZE + 1;
        fn serialize(ref output: Array<felt252>) {
            output.append(selectors::ByteArrayEncoded);
            output.append_span(ENCODING.span());
        }
        fn collect_children(ref children: ChildDefs) {}
        fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
            Self::serialize(ref type_def);
        }
    }
    pub impl Bytes31Encoded<const SIZE: u32, const ENCODING: [felt252; SIZE]> of TypeDef<bytes31> {
        const SIZE: u32 = SIZE + 1;
        fn serialize(ref output: Array<felt252>) {
            output.append(selectors::bytes31Encoded);
            output.append_span(ENCODING.span());
        }
        fn collect_children(ref children: ChildDefs) {}
        fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
            Self::serialize(ref type_def);
        }
    }

    pub impl AsRef<T, impl RD: TypeDefRef<T>> of TypeDef<T> {
        const SIZE: u32 = RD::SIZE;
        fn serialize(ref output: Array<felt252>) {
            RD::serialize(ref output);
        }
        fn collect_children(ref children: ChildDefs) {
            RD::collect_children(ref children);
        }
        fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
            RD::serialize_with_children(ref type_def, ref children);
        }
    }

    pub impl AsInline<T, impl TD: TypeDefInline<T>> of TypeDef<T> {
        const SIZE: u32 = TD::SIZE;
        fn serialize(ref output: Array<felt252>) {
            TD::serialize(ref output);
        }
        fn collect_children(ref children: ChildDefs) {
            TD::collect_children(ref children);
        }
        fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
            TD::serialize_with_children(ref type_def, ref children);
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
pub impl ShortUtf8Def = impls::Primary<bytes31, selectors::ShortUtf8>;
pub impl ClassHashDef = impls::Primary<ClassHash, selectors::ClassHash>;
pub impl ContractAddressDef = impls::Primary<ContractAddress, selectors::ContractAddress>;
pub impl EthAddressDef = impls::Primary<EthAddress, selectors::EthAddress>;
pub impl StorageAddressDef = impls::Primary<StorageAddress, selectors::StorageAddress>;
pub impl StorageBaseAddressDef = impls::Primary<StorageBaseAddress, selectors::StorageBaseAddress>;
pub impl Utf8StringDef = impls::Primary<ByteArray, selectors::Utf8String>;


pub impl FixedSizeArrayDef<T, const SIZE: u32, impl TD: TypeDef<T>> of TypeDef<[T; SIZE]> {
    const SIZE: u32 = TD::SIZE + 2;
    fn serialize(ref output: Array<felt252>) {
        output.append(selectors::FixedArray);
        TD::serialize(ref output);
        output.append(SIZE.into());
    }
    fn collect_children(ref children: ChildDefs) {
        TD::collect_children(ref children);
    }
    fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
        type_def.append(selectors::FixedArray);
        TD::serialize_with_children(ref type_def, ref children);
        type_def.append(SIZE.into());
    }
}
pub impl ArrayDef<T, impl TD: TypeDef<T>> = impls::Nested<selectors::Array, Array<T>, T, TD>;
pub impl SpanDef<T, impl TD: TypeDef<T>> = impls::Nested<selectors::Array, Span<T>, T, TD>;
pub impl OptionDef<T, impl TD: TypeDef<T>> = impls::Nested<selectors::Option, Option<T>, T, TD>;
pub impl NullableDef<T, impl TD: TypeDef<T>> =
    impls::Nested<selectors::Nullable, Nullable<T>, T, TD>;


pub impl BoxDef<T, impl TD: TypeDef<T>> of TypeDef<Box<T>> {
    const SIZE: u32 = TD::SIZE;
    fn serialize(ref output: Array<felt252>) {
        TD::serialize(ref output);
    }
    fn collect_children(ref children: ChildDefs) {
        TD::collect_children(ref children);
    }
    fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
        TD::serialize_with_children(ref type_def, ref children);
    }
}

pub impl ResultDef<T, E, impl TD: TypeDef<T>, impl ED: TypeDef<E>> of TypeDefInline<Result<T, E>> {
    const SIZE: u32 = TD::SIZE + ED::SIZE + 1;
    fn serialize(ref output: Array<felt252>) {
        output.append(selectors::Result);
        TD::serialize(ref output);
        ED::serialize(ref output);
    }
    fn collect_children(ref children: ChildDefs) {
        TD::collect_children(ref children);
        ED::collect_children(ref children);
    }
    fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
        type_def.append(selectors::Result);
        TD::serialize_with_children(ref type_def, ref children);
        ED::serialize_with_children(ref type_def, ref children);
    }
}

pub impl TupleDefImpl<T, impl TD: TupleDefTrait<T>, impl Tuple: IsTuple<T>> of TypeDefInline<T> {
    const SIZE: u32 = TD::SIZE + 2;
    fn serialize(ref output: Array<felt252>) {
        output.append(selectors::Tuple);
        output.append(Tuple::SIZE.into());
        TD::serialize(ref output);
    }
    fn collect_children(ref children: ChildDefs) {
        TD::collect_children(ref children);
    }
    fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
        type_def.append(selectors::Tuple);
        type_def.append(Tuple::SIZE.into());
        TD::serialize_with_children(ref type_def, ref children);
    }
}

impl CompoundDefImpl<
    T,
    const NAME_SIZE: u32,
    const ATTRIBUTES_SIZE: u32,
    impl Def: CompoundDef<T, NAME_SIZE, ATTRIBUTES_SIZE>,
    impl NameToSpan: ToSpanTrait<[felt252; NAME_SIZE], felt252>,
    impl AttrsToSpan: ToSpanTrait<[felt252; ATTRIBUTES_SIZE], felt252>,
> of TypeDefInline<T> {
    const SIZE: u32 = NAME_SIZE + ATTRIBUTES_SIZE + 2;
    fn serialize(ref output: Array<felt252>) {
        output.append(Def::DEF_SELECTOR);
        output.append_span(NameToSpan::span(@Def::NAME));
        output.append(Def::ATTRIBUTES_COUNT.into());
        output.append_span(AttrsToSpan::span(@Def::ATTRIBUTES));
        output.append(Def::FIELDS_COUNT.into());
        Def::serialize_fields(ref output);
    }
    fn collect_children(ref children: ChildDefs) {
        Def::collect_field_children(ref children);
    }
    fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
        type_def.append(Def::DEF_SELECTOR);
        type_def.append_span(NameToSpan::span(@Def::NAME));
        type_def.append(Def::ATTRIBUTES_COUNT.into());
        type_def.append_span(AttrsToSpan::span(@Def::ATTRIBUTES));
        type_def.append(Def::FIELDS_COUNT.into());
        Def::serialize_fields_with_children(ref type_def, ref children);
    }
}

trait TupleDefTrait<T> {
    const SIZE: u32;
    fn serialize(ref output: Array<felt252>);
    fn collect_children(ref children: ChildDefs);
    fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs);
}
impl TupleSize1Impl<T, impl HD: TypeDef<T>> of TupleDefTrait<(T,)> {
    const SIZE: u32 = HD::SIZE;
    fn serialize(ref output: Array<felt252>) {
        HD::serialize(ref output);
    }
    fn collect_children(ref children: ChildDefs) {
        HD::collect_children(ref children);
    }
    fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
        HD::serialize_with_children(ref type_def, ref children);
    }
}
impl NextTupleDefImpl<
    T, impl CS: CollectionSplit<T>, impl HD: TypeDef<CS::Head>, impl RD: TupleDefTrait<CS::Rest>,
> of TupleDefTrait<T> {
    const SIZE: u32 = HD::SIZE + RD::SIZE;
    fn serialize(ref output: Array<felt252>) {
        HD::serialize(ref output);
        RD::serialize(ref output);
    }
    fn collect_children(ref children: ChildDefs) {
        HD::collect_children(ref children);
        RD::collect_children(ref children);
    }
    fn serialize_with_children(ref type_def: Array<felt252>, ref children: ChildDefs) {
        HD::serialize_with_children(ref type_def, ref children);
        RD::serialize_with_children(ref type_def, ref children);
    }
}

pub trait CompoundDef<T, const NAME_SIZE: u32, const ATTRIBUTES_SIZE: u32> {
    const DEF_SELECTOR: felt252;
    const NAME: [felt252; NAME_SIZE];
    const ATTRIBUTES_COUNT: u32;
    const ATTRIBUTES: [felt252; ATTRIBUTES_SIZE];
    const FIELDS_COUNT: u32;
    // const FIELDS_SIZE: u32;
    fn serialize_fields(ref output: Array<felt252>);
    fn collect_field_children(ref children: ChildDefs) {}
    fn serialize_fields_with_children(ref type_def: Array<felt252>, ref children: ChildDefs);
}

pub trait FieldDefTrait<const META_SIZE: u32> {
    const META_DATA: [felt252; META_SIZE];
    type Type;
    impl TD: TypeDef<Self::Type>;
    const fn SIZE() -> u32 {
        META_SIZE + Self::TD::SIZE
    }
    fn serialize(
        ref output: Array<felt252>,
    ) {
        output.append_span(Self::META_DATA.span());
        Self::TD::serialize(ref output);
    }
    fn collect_children(ref children: ChildDefs) {
        Self::TD::collect_children(ref children);
    }
    fn serialize_with_children(
        ref type_def: Array<felt252>, ref child_defs: ChildDefs,
    ) {
        type_def.append_span(Self::META_DATA.span());
        Self::TD::serialize_with_children(ref type_def, ref child_defs);
    }
}

pub impl FieldDef<
    T, const META_SIZE: u32, const META_DATA: [felt252; META_SIZE], impl TD: TypeDef<T>,
> of FieldDefTrait<META_SIZE> {
    const META_DATA: [felt252; META_SIZE] = META_DATA;
    type Type = T;
    impl TD = TD;
}
