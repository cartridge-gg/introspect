#[derive(Drop, PartialEq, Default)]
pub enum Ty {
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
    I8,
    I16,
    I32,
    I64,
    I128,
    USize,
    ShortString,
    ClassHash,
    ContractAddress,
    EthAddress,
    ByteArray,
    Tuple: Span<Ty>,
    Array: Box<Ty>,
    FixedArray: Box<FixedArray>,
    Felt252Dict: Box<Ty>,
    Struct: Struct,
    Enum: Enum,
    Ref: felt252,
    Schema: Span<Field>,
    Encoded: felt252,
    Custom: felt252,
    Option: Box<Ty>,
    Result: Box<CairoResult>,
    Nullable: Box<Ty>,
    DynamicEncoding,
}

#[derive(Drop, Serde, PartialEq)]
pub struct Field {
    pub selector: felt252,
    pub name: ByteArray,
    pub attrs: Span<felt252>,
    pub ty: Ty,
}

#[derive(Drop, Serde, PartialEq)]
pub struct Struct {
    pub name: ByteArray,
    pub attrs: Span<felt252>,
    pub children: Span<Member>,
}

#[derive(Drop, Serde, PartialEq)]
pub struct Enum {
    pub name: ByteArray,
    pub attrs: Span<felt252>,
    pub children: Span<Field>,
}

#[derive(Drop, Serde, PartialEq)]
pub struct FixedArray {
    pub ty: Ty,
    pub size: u32,
}

#[derive(Drop, Serde, PartialEq)]
pub struct Member {
    pub name: ByteArray,
    pub attrs: Span<felt252>,
    pub ty: Ty,
}


#[derive(Drop, Serde, PartialEq)]
pub struct CairoResult {
    pub ok: Ty,
    pub err: Ty,
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
    pub const Custom: felt252 = selector!("custom");
    pub const Schema: felt252 = selector!("schema");
    pub const Option: felt252 = selector!("option");
    pub const Result: felt252 = selector!("result");
    pub const Nullable: felt252 = selector!("nullable");
    pub const Encoded: felt252 = selector!("encoded");
    pub const DynamicEncoding: felt252 = selector!("dynamic_encoding");
}

#[generate_trait]
impl TyImpl of TyTrait {
    const fn selector(self: @Ty) -> felt252 {
        match self {
            Ty::None => selectors::None,
            Ty::Felt252 => selectors::Felt252,
            Ty::Bool => selectors::Bool,
            Ty::U8 => selectors::U8,
            Ty::U16 => selectors::U16,
            Ty::U32 => selectors::U32,
            Ty::U64 => selectors::U64,
            Ty::U128 => selectors::U128,
            Ty::U256 => selectors::U256,
            Ty::I8 => selectors::I8,
            Ty::I16 => selectors::I16,
            Ty::I32 => selectors::I32,
            Ty::I64 => selectors::I64,
            Ty::I128 => selectors::I128,
            Ty::USize => selectors::USize,
            Ty::ShortString => selectors::ShortString,
            Ty::ClassHash => selectors::ClassHash,
            Ty::ContractAddress => selectors::ContractAddress,
            Ty::EthAddress => selectors::EthAddress,
            Ty::ByteArray => selectors::ByteArray,
            Ty::Tuple(_) => selectors::Tuple,
            Ty::Array(_) => selectors::Array,
            Ty::FixedArray(_) => selectors::FixedArray,
            Ty::Felt252Dict(_) => selectors::Felt252Dict,
            Ty::Struct(_) => selectors::Struct,
            Ty::Enum(_) => selectors::Enum,
            Ty::Ref(_) => selectors::Ref,
            Ty::Schema(_) => selectors::Schema,
            Ty::Custom(_) => selectors::Custom,
            Ty::Option(_) => selectors::Option,
            Ty::Result(_) => selectors::Result,
            Ty::Nullable(_) => selectors::Nullable,
            Ty::Encoded(_) => selectors::Encoded,
            Ty::DynamicEncoding(_) => selectors::DynamicEncoding,
        }
    }
}


impl TySerde of Serde<Ty> {
    fn serialize(self: @Ty, ref output: Array<felt252>) {
        match self {
            Ty::None | Ty::Felt252 | Ty::Bool | Ty::U8 | Ty::U16 | Ty::U32 | Ty::U64 | Ty::U128 |
            Ty::U256 | Ty::I8 | Ty::I16 | Ty::I32 | Ty::I64 | Ty::I128 | Ty::USize |
            Ty::ShortString | Ty::ClassHash | Ty::ContractAddress | Ty::EthAddress | Ty::ByteArray |
            Ty::DynamicEncoding => { output.append(self.selector()); },
            Ty::Ref(t) | Ty::Custom(t) |
            Ty::Encoded(t) => { output.append_span([self.selector(), *t].span()); },
            Ty::Array(t) | Ty::Option(t) |
            Ty::Nullable(t) => {
                output.append(selectors::Array);
                Serde::serialize(t, ref output);
            },
            Ty::Tuple(t) => {
                output.append(selectors::Tuple);
                Serde::serialize(t, ref output);
            },
            Ty::FixedArray(t) => {
                output.append(selectors::FixedArray);
                Serde::serialize(t, ref output);
            },
            Ty::Felt252Dict(t) => {
                output.append(selectors::Felt252Dict);
                Serde::serialize(t, ref output);
            },
            Ty::Struct(t) => {
                output.append(selectors::Struct);
                Serde::serialize(t, ref output);
            },
            Ty::Enum(t) => {
                output.append(selectors::Enum);
                Serde::serialize(t, ref output);
            },
            Ty::Result(t) => {
                output.append(selectors::Result);
                Serde::serialize(t, ref output);
            },
            Ty::Schema(t) => {
                output.append(selectors::Schema);
                Serde::serialize(t, ref output);
            },
        }
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<Ty> {
        let tag = *serialized.pop_front()?;

        if tag == 0 {
            Option::Some(Ty::None)
        } else if tag == selectors::Felt252 {
            Option::Some(Ty::Felt252)
        } else if tag == selectors::Bool {
            Option::Some(Ty::Bool)
        } else if tag == selectors::U8 {
            Option::Some(Ty::U8)
        } else if tag == selectors::U16 {
            Option::Some(Ty::U16)
        } else if tag == selectors::U32 {
            Option::Some(Ty::U32)
        } else if tag == selectors::U64 {
            Option::Some(Ty::U64)
        } else if tag == selectors::U128 {
            Option::Some(Ty::U128)
        } else if tag == selectors::U256 {
            Option::Some(Ty::U256)
        } else if tag == selectors::I8 {
            Option::Some(Ty::I8)
        } else if tag == selectors::I16 {
            Option::Some(Ty::I16)
        } else if tag == selectors::I32 {
            Option::Some(Ty::I32)
        } else if tag == selectors::I64 {
            Option::Some(Ty::I64)
        } else if tag == selectors::I128 {
            Option::Some(Ty::I128)
        } else if tag == selectors::USize {
            Option::Some(Ty::USize)
        } else if tag == selectors::ShortString {
            Option::Some(Ty::ShortString)
        } else if tag == selectors::ClassHash {
            Option::Some(Ty::ClassHash)
        } else if tag == selectors::ContractAddress {
            Option::Some(Ty::ContractAddress)
        } else if tag == selectors::EthAddress {
            Option::Some(Ty::EthAddress)
        } else if tag == selectors::ByteArray {
            Option::Some(Ty::ByteArray)
        } else if tag == selectors::Tuple {
            Option::Some(Ty::Tuple(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Array {
            Option::Some(Ty::Array(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::FixedArray {
            Option::Some(Ty::FixedArray(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Felt252Dict {
            Option::Some(Ty::Felt252Dict(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Struct {
            Option::Some(Ty::Struct(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Enum {
            Option::Some(Ty::Enum(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Ref {
            Option::Some(Ty::Ref(*serialized.pop_front()?))
        } else if tag == selectors::Custom {
            Option::Some(Ty::Custom(*serialized.pop_front()?))
        } else if tag == selectors::Schema {
            Option::Some(Ty::Schema(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Option {
            Option::Some(Ty::Option(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Result {
            Option::Some(Ty::Result(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Nullable {
            Option::Some(Ty::Nullable(Serde::deserialize(ref serialized)?))
        } else if tag == selectors::Encoded {
            Option::Some(Ty::Encoded(*serialized.pop_front()?))
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
