#[derive(Drop, PartialEq, Default)]
pub enum TypeDef {
    #[default]
    None,
    Felt252,
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
    ByteArray,
    ShortString,
    Tuple: Span<TypeDef>,
    Array: Box<TypeDef>,
    FixedArray: Box<FixedArrayDef>,
    Felt252Dict: Box<TypeDef>,
    Struct: StructDef,
    Enum: EnumDef,
    UnitEnum: UnitEnumDef,
    Option: Box<TypeDef>,
    Result: Box<ResultDef>,
    Nullable: Box<TypeDef>,
    Ref: felt252,
    Self,
    Recursive: felt252,
    Custom: felt252,

}

#[derive(Drop, Serde, PartialEq)]
struct Attribute {
    pub id: felt252,
    pub data: Span<felt252>,
}

#[derive(Drop, Serde, PartialEq)]
pub struct ColumnDef {
    pub id: felt252,
    pub name: ByteArray,
    pub attrs: Span<felt252>,
    pub type_def: TypeDef,
}

#[derive(Drop, Serde, PartialEq)]
pub struct FieldDef {
    pub name: ByteArray,
    pub attrs: Span<felt252>,
    pub type_def: TypeDef,
}

#[derive(Drop, Serde, PartialEq)]
pub struct StructDef {
    pub name: ByteArray,
    pub attrs: Span<felt252>,
    pub members: Span<MemberDef>,
}

#[derive(Drop, Serde, PartialEq)]
pub struct MemberDef {
    pub name: ByteArray,
    pub attrs: Span<felt252>,
    pub type_def: TypeDef,
}

#[derive(Drop, Serde, PartialEq)]
pub struct EnumDef {
    pub name: ByteArray,
    pub attrs: Span<Attribute>,
    pub variants: Span<VariantDef>,
}

pub struct UnitEnumDef {
    pub name: ByteArray,
    pub attrs: Span<Attribute>,
    pub variants: Span<UnitVariantDef>,
}



#[derive(Drop, Serde, PartialEq)]
pub struct VariantDef {
    pub selector: felt252,
    pub name: ByteArray,
    pub attrs: Span<Attribute>,
    pub type_def: TypeDef,
}

#[derive(Drop, Serde, PartialEq)]
pub struct UnitVariantDef {
    pub selector: felt252,
    pub name: ByteArray,
}

#[derive(Drop, Serde, PartialEq)]
pub struct FixedArrayDef {
    pub type_def: TypeDef,
    pub size: u32,
}


#[derive(Drop, Serde, PartialEq)]
pub struct ResultDef {
    pub ok: TypeDef,
    pub err: TypeDef,
}


mod selectors {
    pub const None: felt252 = 0;
    pub const Felt252: felt252 = selector!("felt252");
    pub const Bool: felt252 = selector!("bool");
    pub const U8: felt252 = selector!("u8");
    pub const U16: felt252 = selector!("u16");
    pub const U32: felt252 = selector!("u32");
    pub const U64: felt252 = selector!("u64");
    pub const U128: felt252 = selector!("u128");
    pub const U256: felt252 = selector!("u256");
    pub const I8: felt252 = selector!("i8");
    pub const I16: felt252 = selector!("i16");
    pub const I32: felt252 = selector!("i32");
    pub const I64: felt252 = selector!("i64");
    pub const I128: felt252 = selector!("i128");
    pub const USize: felt252 = selector!("usize");
    pub const ShortString: felt252 = selector!("short_string");
    pub const ClassHash: felt252 = selector!("class_hash");
    pub const ContractAddress: felt252 = selector!("contract_address");
    pub const EthAddress: felt252 = selector!("eth_address");
    pub const ByteArray: felt252 = selector!("byte_array");
    pub const Tuple: felt252 = selector!("tuple");
    pub const Array: felt252 = selector!("array");
    pub const FixedArray: felt252 = selector!("fixed_array");
    pub const Felt252Dict: felt252 = selector!("felt252_dict");
    pub const Struct: felt252 = selector!("struct");
    pub const Enum: felt252 = selector!("enum");
    pub const Ref: felt252 = selector!("ref");
    pub const Recursive: felt252 = selector!("recursive");
    pub const Custom: felt252 = selector!("custom");
    pub const Option: felt252 = selector!("option");
    pub const Result: felt252 = selector!("result");
    pub const Nullable: felt252 = selector!("nullable");
    pub const Encoded: felt252 = selector!("encoded");
    pub const DynamicEncoding: felt252 = selector!("dynamic_encoding");
}

#[generate_trait]
impl TyImpl of TyTrait {
    const fn selector(self: @TypeDef) -> felt252 {
        match self {
            TypeDef::None => selectors::None,
            TypeDef::Felt252 => selectors::Felt252,
            TypeDef::Bool => selectors::Bool,
            TypeDef::U8 => selectors::U8,
            TypeDef::U16 => selectors::U16,
            TypeDef::U32 => selectors::U32,
            TypeDef::U64 => selectors::U64,
            TypeDef::U128 => selectors::U128,
            TypeDef::U256 => selectors::U256,
            TypeDef::I8 => selectors::I8,
            TypeDef::I16 => selectors::I16,
            TypeDef::I32 => selectors::I32,
            TypeDef::I64 => selectors::I64,
            TypeDef::I128 => selectors::I128,
            TypeDef::USize => selectors::USize,
            TypeDef::ShortString => selectors::ShortString,
            TypeDef::ClassHash => selectors::ClassHash,
            TypeDef::ContractAddress => selectors::ContractAddress,
            TypeDef::EthAddress => selectors::EthAddress,
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
            TypeDef::Encoded(_) => selectors::Encoded,
            TypeDef::DynamicEncoding(_) => selectors::DynamicEncoding,
        }
    }
}


impl TySerde of Serde<TypeDef> {
    fn serialize(self: @TypeDef, ref output: Array<felt252>) {
        match self {
            TypeDef::None | TypeDef::Felt252 | TypeDef::Bool | TypeDef::U8 | TypeDef::U16 |
            TypeDef::U32 | TypeDef::U64 | TypeDef::U128 | TypeDef::U256 | TypeDef::I8 |
            TypeDef::I16 | TypeDef::I32 | TypeDef::I64 | TypeDef::I128 | TypeDef::USize |
            TypeDef::ShortString | TypeDef::ClassHash | TypeDef::ContractAddress |
            TypeDef::EthAddress | TypeDef::ByteArray |
            TypeDef::DynamicEncoding => { output.append(self.selector()); },
            TypeDef::Ref(t) | TypeDef::Custom(t) |
            TypeDef::Encoded(t) => { output.append_span([self.selector(), *t].span()); },
            TypeDef::Array(t) | TypeDef::Option(t) |
            TypeDef::Nullable(t) => {
                output.append(selectors::Array);
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
            TypeDef::Felt252Dict(t) => {
                output.append(selectors::Felt252Dict);
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
        } else if tag == selectors::USize {
            Option::Some(TypeDef::USize)
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
        } else if tag == selectors::Encoded {
            Option::Some(TypeDef::Encoded(*serialized.pop_front()?))
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
