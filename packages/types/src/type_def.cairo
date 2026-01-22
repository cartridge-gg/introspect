use crate::{Attribute, ISerde, Introspect};

#[derive(Drop, PartialEq, Default, Debug)]
pub enum TypeDef {
    #[default]
    None,
    Felt252,
    ShortUtf8,
    Bytes31,
    Bytes31Encoded: ByteArray,
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    U512,
    I8,
    I16,
    I32,
    I64,
    I128,
    ClassHash,
    ContractAddress,
    EthAddress,
    StorageAddress,
    StorageBaseAddress,
    ByteArray,
    Utf8String,
    ByteArrayEncoded: ByteArray,
    Tuple: Span<TypeDef>,
    Array: Box<TypeDef>,
    FixedArray: Box<FixedArrayDef>,
    Felt252Dict: Box<TypeDef>,
    Struct: StructDef,
    Enum: EnumDef,
    Option: Box<TypeDef>,
    Result: Box<ResultDef>,
    Nullable: Box<TypeDef>,
    Ref: felt252,
    Custom: ByteArray,
}

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct StructDef {
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub members: Span<MemberDef>,
}

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct MemberDef {
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub type_def: TypeDef,
}


#[derive(Drop, Serde, PartialEq, Debug)]
pub struct EnumDef {
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub variants: Span<VariantDef>,
}

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct VariantDef {
    pub selector: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct FixedArrayDef {
    pub type_def: TypeDef,
    pub size: u32,
}

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct ResultDef {
    pub ok: TypeDef,
    pub err: TypeDef,
}

#[generate_trait]
pub impl MemberDefImpl of MemberDefTrait {
    fn new<T, +Introspect<T>>(name: ByteArray, attributes: Span<Attribute>) -> MemberDef {
        MemberDef { name, attributes, type_def: Introspect::<T>::type_def() }
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

pub trait SelectorTrait<T> {
    const fn selector(self: @T) -> felt252 nopanic;
}

impl TypeDefSelector of SelectorTrait<TypeDef> {
    const fn selector(self: @TypeDef) -> felt252 nopanic {
        match self {
            TypeDef::None => selectors::None,
            TypeDef::Felt252 => selectors::felt252,
            TypeDef::ShortUtf8 => selectors::ShortUtf8,
            TypeDef::Bytes31 => selectors::bytes31,
            TypeDef::Bytes31Encoded(_) => selectors::bytes31Encoded,
            TypeDef::Bool => selectors::bool,
            TypeDef::U8 => selectors::u8,
            TypeDef::U16 => selectors::u16,
            TypeDef::U32 => selectors::u32,
            TypeDef::U64 => selectors::u64,
            TypeDef::U128 => selectors::u128,
            TypeDef::U256 => selectors::u256,
            TypeDef::U512 => selectors::u512,
            TypeDef::I8 => selectors::i8,
            TypeDef::I16 => selectors::i16,
            TypeDef::I32 => selectors::i32,
            TypeDef::I64 => selectors::i64,
            TypeDef::I128 => selectors::i128,
            TypeDef::ClassHash => selectors::ClassHash,
            TypeDef::ContractAddress => selectors::ContractAddress,
            TypeDef::EthAddress => selectors::EthAddress,
            TypeDef::StorageAddress => selectors::StorageAddress,
            TypeDef::StorageBaseAddress => selectors::StorageBaseAddress,
            TypeDef::ByteArray => selectors::ByteArray,
            TypeDef::Utf8String => selectors::Utf8String,
            TypeDef::ByteArrayEncoded(_) => selectors::ByteArrayEncoded,
            TypeDef::Tuple(_) => selectors::Tuple,
            TypeDef::Array(_) => selectors::Array,
            TypeDef::FixedArray(_) => selectors::FixedArray,
            TypeDef::Felt252Dict(_) => selectors::Felt252Dict,
            TypeDef::Struct(_) => selectors::Struct,
            TypeDef::Enum(_) => selectors::Enum,
            TypeDef::Ref(_) => selectors::Ref,
            TypeDef::Custom(_) => selectors::Custom,
            TypeDef::Option(_) => selectors::Option,
            TypeDef::Result(_) => selectors::Result,
            TypeDef::Nullable(_) => selectors::Nullable,
        }
    }
}


impl TySerde of Serde<TypeDef> {
    fn serialize(self: @TypeDef, ref output: Array<felt252>) {
        match self {
            TypeDef::None | TypeDef::Felt252 | TypeDef::ShortUtf8 | TypeDef::Bytes31 |
            TypeDef::Bool | TypeDef::U8 | TypeDef::U16 | TypeDef::U32 | TypeDef::U64 |
            TypeDef::U128 | TypeDef::U256 | TypeDef::U512 | TypeDef::I8 | TypeDef::I16 |
            TypeDef::I32 | TypeDef::I64 | TypeDef::I128 | TypeDef::ClassHash |
            TypeDef::ContractAddress | TypeDef::EthAddress | TypeDef::StorageAddress |
            TypeDef::ByteArray | TypeDef::Utf8String |
            TypeDef::StorageBaseAddress => { output.append(self.selector()); },
            TypeDef::Ref(t) => {
                output.append(self.selector());
                output.append(*t);
            },
            TypeDef::Custom(t) | TypeDef::Bytes31Encoded(t) |
            TypeDef::ByteArrayEncoded(t) => {
                output.append(self.selector());
                t.serialize(ref output)
            },
            TypeDef::Nullable(t) | TypeDef::Array(t) | TypeDef::Option(t) |
            TypeDef::Felt252Dict(t) => {
                output.append(self.selector());
                Serde::serialize(t, ref output);
            },
            TypeDef::Tuple(t) => {
                output.append(selectors::Tuple);
                Serde::serialize(t, ref output);
            },
            TypeDef::FixedArray(t) => {
                output.append(selectors::FixedArray);
                Serde::serialize(t, ref output);
            },
            TypeDef::Struct(t) => {
                output.append(selectors::Struct);
                Serde::serialize(t, ref output);
            },
            TypeDef::Enum(t) => {
                output.append(selectors::Enum);
                Serde::serialize(t, ref output);
            },
            TypeDef::Result(t) => {
                output.append(selectors::Result);
                Serde::serialize(t, ref output);
            },
        }
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<TypeDef> {
        let tag = *serialized.pop_front()?;

        if tag == 0 {
            Option::Some(TypeDef::None)
        } else if tag == selectors::felt252 {
            Option::Some(TypeDef::Felt252)
        } else if tag == selectors::ShortUtf8 {
            Option::Some(TypeDef::ShortUtf8)
        } else if tag == selectors::bytes31 {
            Option::Some(TypeDef::Bytes31)
        } else if tag == selectors::bytes31Encoded {
            Option::Some(TypeDef::Bytes31Encoded(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::bool {
            Option::Some(TypeDef::Bool)
        } else if tag == selectors::u8 {
            Option::Some(TypeDef::U8)
        } else if tag == selectors::u16 {
            Option::Some(TypeDef::U16)
        } else if tag == selectors::u32 {
            Option::Some(TypeDef::U32)
        } else if tag == selectors::u64 {
            Option::Some(TypeDef::U64)
        } else if tag == selectors::u128 {
            Option::Some(TypeDef::U128)
        } else if tag == selectors::u256 {
            Option::Some(TypeDef::U256)
        } else if tag == selectors::u512 {
            Option::Some(TypeDef::U512)
        } else if tag == selectors::i8 {
            Option::Some(TypeDef::I8)
        } else if tag == selectors::i16 {
            Option::Some(TypeDef::I16)
        } else if tag == selectors::i32 {
            Option::Some(TypeDef::I32)
        } else if tag == selectors::i64 {
            Option::Some(TypeDef::I64)
        } else if tag == selectors::i128 {
            Option::Some(TypeDef::I128)
        } else if tag == selectors::ClassHash {
            Option::Some(TypeDef::ClassHash)
        } else if tag == selectors::ContractAddress {
            Option::Some(TypeDef::ContractAddress)
        } else if tag == selectors::EthAddress {
            Option::Some(TypeDef::EthAddress)
        } else if tag == selectors::StorageAddress {
            Option::Some(TypeDef::StorageAddress)
        } else if tag == selectors::StorageBaseAddress {
            Option::Some(TypeDef::StorageBaseAddress)
        } else if tag == selectors::ByteArray {
            Option::Some(TypeDef::ByteArray)
        } else if tag == selectors::Utf8String {
            Option::Some(TypeDef::Utf8String)
        } else if tag == selectors::ByteArrayEncoded {
            Option::Some(TypeDef::ByteArrayEncoded(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Tuple {
            Option::Some(TypeDef::Tuple(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Array {
            Option::Some(TypeDef::Array(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::FixedArray {
            Option::Some(TypeDef::FixedArray(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Felt252Dict {
            Option::Some(TypeDef::Felt252Dict(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Struct {
            Option::Some(TypeDef::Struct(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Enum {
            Option::Some(TypeDef::Enum(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Ref {
            Option::Some(TypeDef::Ref(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Custom {
            Option::Some(TypeDef::Custom(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Option {
            Option::Some(TypeDef::Option(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Result {
            Option::Some(TypeDef::Result(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Nullable {
            Option::Some(TypeDef::Nullable(Serde::deserialize(ref serialized)?))
        } else {
            Option::None
        }
    }
}


impl TyISerde of ISerde<TypeDef> {
    fn iserialize(self: @TypeDef, ref output: Array<felt252>) {
        match self {
            TypeDef::None | TypeDef::Felt252 | TypeDef::ShortUtf8 | TypeDef::Bytes31 |
            TypeDef::Bool | TypeDef::U8 | TypeDef::U16 | TypeDef::U32 | TypeDef::U64 |
            TypeDef::U128 | TypeDef::U256 | TypeDef::U512 | TypeDef::I8 | TypeDef::I16 |
            TypeDef::I32 | TypeDef::I64 | TypeDef::I128 | TypeDef::ClassHash |
            TypeDef::ContractAddress | TypeDef::EthAddress | TypeDef::StorageAddress |
            TypeDef::ByteArray | TypeDef::Utf8String |
            TypeDef::StorageBaseAddress => { output.append(self.selector()); },
            TypeDef::Ref(t) => {
                output.append(self.selector());
                output.append(*t);
            },
            TypeDef::Custom(t) | TypeDef::Bytes31Encoded(t) |
            TypeDef::ByteArrayEncoded(t) => {
                output.append(self.selector());
                t.iserialize(ref output)
            },
            TypeDef::Nullable(t) | TypeDef::Array(t) | TypeDef::Option(t) |
            TypeDef::Felt252Dict(t) => {
                output.append(self.selector());
                ISerde::iserialize(t, ref output);
            },
            TypeDef::Tuple(t) => {
                output.append(selectors::Tuple);
                ISerde::iserialize(t, ref output);
            },
            TypeDef::FixedArray(t) => {
                output.append(selectors::FixedArray);
                ISerde::iserialize(t, ref output);
            },
            TypeDef::Struct(t) => {
                output.append(selectors::Struct);
                ISerde::iserialize(t, ref output);
            },
            TypeDef::Enum(t) => {
                output.append(selectors::Enum);
                ISerde::iserialize(t, ref output);
            },
            TypeDef::Result(t) => {
                output.append(selectors::Result);
                ISerde::iserialize(t, ref output);
            },
        }
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<TypeDef> {
        let tag = *serialized.pop_front()?;

        if tag == 0 {
            Option::Some(TypeDef::None)
        } else if tag == selectors::felt252 {
            Option::Some(TypeDef::Felt252)
        } else if tag == selectors::ShortUtf8 {
            Option::Some(TypeDef::ShortUtf8)
        } else if tag == selectors::bytes31 {
            Option::Some(TypeDef::Bytes31)
        } else if tag == selectors::bytes31Encoded {
            Option::Some(TypeDef::Bytes31Encoded(ISerde::ideserialize(ref serialized)?))
        } else if tag == selectors::bool {
            Option::Some(TypeDef::Bool)
        } else if tag == selectors::u8 {
            Option::Some(TypeDef::U8)
        } else if tag == selectors::u16 {
            Option::Some(TypeDef::U16)
        } else if tag == selectors::u32 {
            Option::Some(TypeDef::U32)
        } else if tag == selectors::u64 {
            Option::Some(TypeDef::U64)
        } else if tag == selectors::u128 {
            Option::Some(TypeDef::U128)
        } else if tag == selectors::u256 {
            Option::Some(TypeDef::U256)
        } else if tag == selectors::u512 {
            Option::Some(TypeDef::U512)
        } else if tag == selectors::i8 {
            Option::Some(TypeDef::I8)
        } else if tag == selectors::i16 {
            Option::Some(TypeDef::I16)
        } else if tag == selectors::i32 {
            Option::Some(TypeDef::I32)
        } else if tag == selectors::i64 {
            Option::Some(TypeDef::I64)
        } else if tag == selectors::i128 {
            Option::Some(TypeDef::I128)
        } else if tag == selectors::ClassHash {
            Option::Some(TypeDef::ClassHash)
        } else if tag == selectors::ContractAddress {
            Option::Some(TypeDef::ContractAddress)
        } else if tag == selectors::EthAddress {
            Option::Some(TypeDef::EthAddress)
        } else if tag == selectors::StorageAddress {
            Option::Some(TypeDef::StorageAddress)
        } else if tag == selectors::StorageBaseAddress {
            Option::Some(TypeDef::StorageBaseAddress)
        } else if tag == selectors::ByteArray {
            Option::Some(TypeDef::ByteArray)
        } else if tag == selectors::Utf8String {
            Option::Some(TypeDef::Utf8String)
        } else if tag == selectors::ByteArrayEncoded {
            Option::Some(TypeDef::ByteArrayEncoded(ISerde::ideserialize(ref serialized)?))
        } else if tag == selectors::Tuple {
            Option::Some(TypeDef::Tuple(ISerde::ideserialize(ref serialized)?))
        } else if tag == selectors::Array {
            Option::Some(TypeDef::Array(ISerde::ideserialize(ref serialized)?))
        } else if tag == selectors::FixedArray {
            Option::Some(TypeDef::FixedArray(ISerde::ideserialize(ref serialized)?))
        } else if tag == selectors::Felt252Dict {
            Option::Some(TypeDef::Felt252Dict(ISerde::ideserialize(ref serialized)?))
        } else if tag == selectors::Struct {
            Option::Some(TypeDef::Struct(ISerde::ideserialize(ref serialized)?))
        } else if tag == selectors::Enum {
            Option::Some(TypeDef::Enum(ISerde::ideserialize(ref serialized)?))
        } else if tag == selectors::Ref {
            Option::Some(TypeDef::Ref(ISerde::ideserialize(ref serialized)?))
        } else if tag == selectors::Custom {
            Option::Some(TypeDef::Custom(ISerde::ideserialize(ref serialized)?))
        } else if tag == selectors::Option {
            Option::Some(TypeDef::Option(ISerde::ideserialize(ref serialized)?))
        } else if tag == selectors::Result {
            Option::Some(TypeDef::Result(ISerde::ideserialize(ref serialized)?))
        } else if tag == selectors::Nullable {
            Option::Some(TypeDef::Nullable(ISerde::ideserialize(ref serialized)?))
        } else {
            Option::None
        }
    }
}

impl BoxSerdeImpl<T, +Serde<T>> of Serde<Box<T>> {
    fn serialize(self: @Box<T>, ref output: Array<felt252>) {
        Serde::<T>::serialize(self.as_snapshot().unbox(), ref output);
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<Box<T>> {
        match Serde::<T>::deserialize(ref serialized) {
            Option::Some(t) => Option::Some(BoxTrait::new(t)),
            Option::None => Option::None,
        }
    }
}


impl BoxPartialEq<T, +PartialEq<T>> of PartialEq<Box<T>> {
    #[inline]
    fn eq(lhs: @Box<T>, rhs: @Box<T>) -> bool {
        PartialEq::<T>::eq(lhs.as_snapshot().unbox(), rhs.as_snapshot().unbox())
    }
}


impl StructDefISerde of ISerde<StructDef> {
    fn iserialize(self: @StructDef, ref output: Array<felt252>) {
        self.name.iserialize(ref output);
        self.attributes.iserialize(ref output);
        self.members.iserialize(ref output);
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<StructDef> {
        let name = ISerde::ideserialize(ref serialized)?;
        let attributes = ISerde::ideserialize(ref serialized)?;
        let members = ISerde::ideserialize(ref serialized)?;
        Some(StructDef { name, attributes, members })
    }
}

impl MemberDefISerde of ISerde<MemberDef> {
    fn iserialize(self: @MemberDef, ref output: Array<felt252>) {
        self.name.iserialize(ref output);
        self.attributes.iserialize(ref output);
        self.type_def.iserialize(ref output);
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<MemberDef> {
        let name = ISerde::ideserialize(ref serialized)?;
        let attributes = ISerde::ideserialize(ref serialized)?;
        let type_def = ISerde::ideserialize(ref serialized)?;
        Some(MemberDef { name, attributes, type_def })
    }
}


impl EnumDefISerde of ISerde<EnumDef> {
    fn iserialize(self: @EnumDef, ref output: Array<felt252>) {
        self.name.iserialize(ref output);
        self.attributes.iserialize(ref output);
        self.variants.iserialize(ref output);
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<EnumDef> {
        let name = ISerde::ideserialize(ref serialized)?;
        let attributes = ISerde::ideserialize(ref serialized)?;
        let variants = ISerde::ideserialize(ref serialized)?;
        Some(EnumDef { name, attributes, variants })
    }
}

impl VariantDefISerde of ISerde<VariantDef> {
    fn iserialize(self: @VariantDef, ref output: Array<felt252>) {
        output.append(*self.selector);
        self.name.iserialize(ref output);
        self.attributes.iserialize(ref output);
        self.type_def.iserialize(ref output);
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<VariantDef> {
        let selector = *serialized.pop_front()?;
        let name = ISerde::ideserialize(ref serialized)?;
        let attributes = ISerde::ideserialize(ref serialized)?;
        let type_def = ISerde::ideserialize(ref serialized)?;
        Some(VariantDef { selector, name, attributes, type_def })
    }
}


impl FixedArrayDefISerde of ISerde<FixedArrayDef> {
    fn iserialize(self: @FixedArrayDef, ref output: Array<felt252>) {
        self.type_def.iserialize(ref output);
        output.append((*self.size).into());
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<FixedArrayDef> {
        let type_def = ISerde::ideserialize(ref serialized)?;
        let size: u32 = (*serialized.pop_front()?).try_into()?;
        Some(FixedArrayDef { type_def, size })
    }
}

impl ResultDefISerde of ISerde<ResultDef> {
    fn iserialize(self: @ResultDef, ref output: Array<felt252>) {
        self.ok.iserialize(ref output);
        self.err.iserialize(ref output);
    }

    fn ideserialize(ref serialized: Span<felt252>) -> Option<ResultDef> {
        let ok = ISerde::ideserialize(ref serialized)?;
        let err = ISerde::ideserialize(ref serialized)?;
        Some(ResultDef { ok, err })
    }
}
