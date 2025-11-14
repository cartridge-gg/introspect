use crate::Introspect;

#[derive(Drop, PartialEq, Default, Debug)]
pub enum TypeDef {
    #[default]
    None,
    Felt252,
    Bytes31,
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
    ShortString,
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
    Custom: felt252,
}

#[derive(Drop, Serde, PartialEq, Debug)]
pub struct TypeWithAttributes {
    pub type_def: TypeDef,
    pub attributes: Span<Attribute>,
}

#[generate_trait]
impl TypeDefImpl of TypeDefTrait {
    fn allowed_as_primary(self: @TypeDef) -> bool {
        match self {
            TypeDef::Felt252 | TypeDef::Bytes31 | TypeDef::Bool | TypeDef::U8 | TypeDef::U16 |
            TypeDef::U32 | TypeDef::U64 | TypeDef::U128 | TypeDef::ClassHash |
            TypeDef::ContractAddress | TypeDef::EthAddress => true,
            _ => false,
        }
    }
}


#[derive(Drop, Serde, PartialEq, Debug)]
pub struct Attribute {
    pub id: felt252,
    pub data: Span<felt252>,
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

#[generate_trait]
pub impl MemberDefImpl of MemberDefTrait {
    fn new<T, +Introspect<T>>(name: ByteArray, attributes: Span<Attribute>) -> MemberDef {
        MemberDef { name, type_def: Introspect::<T>::type_def(), attributes }
    }
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


mod selectors {
    pub const None: felt252 = 0;
    pub const Felt252: felt252 = 'felt252';
    pub const Bool: felt252 = 'bool';
    pub const U8: felt252 = 'u8';
    pub const U16: felt252 = 'u16';
    pub const U32: felt252 = 'u32';
    pub const U64: felt252 = 'u64';
    pub const U128: felt252 = 'u128';
    pub const U256: felt252 = 'u256';
    pub const U512: felt252 = 'u512';
    pub const I8: felt252 = 'i8';
    pub const I16: felt252 = 'i16';
    pub const I32: felt252 = 'i32';
    pub const I64: felt252 = 'i64';
    pub const I128: felt252 = 'i128';
    pub const Bytes31: felt252 = 'bytes31';
    pub const ShortString: felt252 = 'ShortString';
    pub const ClassHash: felt252 = 'ClassHash';
    pub const ContractAddress: felt252 = 'ContractAddress';
    pub const EthAddress: felt252 = 'EthAddress';
    pub const StorageAddress: felt252 = 'StorageAddress';
    pub const StorageBaseAddress: felt252 = 'StorageBaseAddress';
    pub const ByteArray: felt252 = 'ByteArray';
    pub const Tuple: felt252 = 'Tuple';
    pub const Array: felt252 = 'Array';
    pub const FixedArray: felt252 = 'FixedArray';
    pub const Felt252Dict: felt252 = 'Felt252Dict';
    pub const Struct: felt252 = 'struct';
    pub const Enum: felt252 = 'enum';
    pub const Ref: felt252 = 'ref';
    pub const RecursiveSelf: felt252 = 'recursive_self';
    pub const Recursive: felt252 = 'recursive_type';
    pub const Custom: felt252 = 'custom';
    pub const Option: felt252 = 'Option';
    pub const Result: felt252 = 'Result';
    pub const Nullable: felt252 = 'Nullable';
}

#[generate_trait]
impl TyImpl of TyTrait {
    const fn selector(self: @TypeDef) -> felt252 {
        match self {
            TypeDef::None => selectors::None,
            TypeDef::Felt252 => selectors::Felt252,
            TypeDef::Bytes31 => selectors::Bytes31,
            TypeDef::Bool => selectors::Bool,
            TypeDef::U8 => selectors::U8,
            TypeDef::U16 => selectors::U16,
            TypeDef::U32 => selectors::U32,
            TypeDef::U64 => selectors::U64,
            TypeDef::U128 => selectors::U128,
            TypeDef::U256 => selectors::U256,
            TypeDef::U512 => selectors::U512,
            TypeDef::I8 => selectors::I8,
            TypeDef::I16 => selectors::I16,
            TypeDef::I32 => selectors::I32,
            TypeDef::I64 => selectors::I64,
            TypeDef::I128 => selectors::I128,
            TypeDef::ShortString => selectors::ShortString,
            TypeDef::ClassHash => selectors::ClassHash,
            TypeDef::ContractAddress => selectors::ContractAddress,
            TypeDef::EthAddress => selectors::EthAddress,
            TypeDef::StorageAddress => selectors::StorageAddress,
            TypeDef::StorageBaseAddress => selectors::StorageBaseAddress,
            TypeDef::ByteArray => selectors::ByteArray,
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
            TypeDef::None | TypeDef::Felt252 | TypeDef::Bytes31 | TypeDef::Bool | TypeDef::U8 |
            TypeDef::U16 | TypeDef::U32 | TypeDef::U64 | TypeDef::U128 | TypeDef::U256 |
            TypeDef::U512 | TypeDef::I8 | TypeDef::I16 | TypeDef::I32 | TypeDef::I64 |
            TypeDef::I128 | TypeDef::ShortString | TypeDef::ClassHash | TypeDef::ContractAddress |
            TypeDef::EthAddress | TypeDef::StorageAddress |
            TypeDef::StorageBaseAddress => { output.append(self.selector()); },
            TypeDef::ByteArray => { output.append(self.selector()); },
            TypeDef::Ref(t) |
            TypeDef::Custom(t) => {
                output.append(self.selector());
                output.append(*t);
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
        } else if tag == selectors::Felt252 {
            Option::Some(TypeDef::Felt252)
        } else if tag == selectors::Bool {
            Option::Some(TypeDef::Bool)
        } else if tag == selectors::U8 {
            Option::Some(TypeDef::U8)
        } else if tag == selectors::U16 {
            Option::Some(TypeDef::U16)
        } else if tag == selectors::U32 {
            Option::Some(TypeDef::U32)
        } else if tag == selectors::U64 {
            Option::Some(TypeDef::U64)
        } else if tag == selectors::U128 {
            Option::Some(TypeDef::U128)
        } else if tag == selectors::U256 {
            Option::Some(TypeDef::U256)
        } else if tag == selectors::I8 {
            Option::Some(TypeDef::I8)
        } else if tag == selectors::I16 {
            Option::Some(TypeDef::I16)
        } else if tag == selectors::I32 {
            Option::Some(TypeDef::I32)
        } else if tag == selectors::I64 {
            Option::Some(TypeDef::I64)
        } else if tag == selectors::I128 {
            Option::Some(TypeDef::I128)
        } else if tag == selectors::ShortString {
            Option::Some(TypeDef::ShortString)
        } else if tag == selectors::ClassHash {
            Option::Some(TypeDef::ClassHash)
        } else if tag == selectors::ContractAddress {
            Option::Some(TypeDef::ContractAddress)
        } else if tag == selectors::EthAddress {
            Option::Some(TypeDef::EthAddress)
        } else if tag == selectors::ByteArray {
            Option::Some(TypeDef::ByteArray)
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
            Option::Some(TypeDef::Ref(*serialized.pop_front()?))
        } else if tag == selectors::Custom {
            Option::Some(TypeDef::Custom(*serialized.pop_front()?))
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


impl BoxTySerdeImpl<T, +Serde<T>> of Serde<Box<T>> {
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


impl ClassHashPartialEq<T, +PartialEq<T>> of PartialEq<Box<T>> {
    #[inline]
    fn eq(lhs: @Box<T>, rhs: @Box<T>) -> bool {
        PartialEq::<T>::eq(lhs.as_snapshot().unbox(), rhs.as_snapshot().unbox())
    }
}
