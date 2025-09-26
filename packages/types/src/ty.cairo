#[derive(Drop, Copy, Serde, PartialEq)]
pub struct Field {
    pub selector: felt252,
    pub name: felt252,
    pub attrs: Span<felt252>,
    pub ty: Ty,
}

#[derive(Drop, Copy, Serde, PartialEq)]
pub struct Struct {
    pub name: felt252,
    pub attrs: Span<felt252>,
    pub children: Span<Member>,
}

#[derive(Drop, Copy, Serde, PartialEq)]
pub struct Enum {
    pub name: felt252,
    pub attrs: Span<felt252>,
    pub children: Span<Field>,
}

#[derive(Drop, Copy, Serde, PartialEq)]
pub struct Member {
    pub name: felt252,
    pub attrs: Span<felt252>,
    pub ty: Ty,
}

#[derive(Drop, Copy, Serde, PartialEq)]
pub struct Function {
    pub name: felt252,
    pub attrs: Span<felt252>,
    pub args: Span<Member>,
    pub ret: Box<Ty>,
}

#[derive(Drop, Copy, Serde, PartialEq, Default)]
pub enum Ty {
    #[default]
    None,
    Felt252,
    Bool,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Uint256,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    USize,
    ShortString,
    ClassHash,
    ContractAddress,
    EthAddress,
    ByteArray,
    Tuple: Span<Ty>,
    Array: Box<Ty>,
    FixedArray: (Box<Ty>, u32),
    Struct: Struct,
    Enum: Enum,
    Schema: felt252,
    Custom: felt252,
    Option: Box<Ty>,
    Result: (Box<Ty>, Box<Ty>),
    Nullable: Box<Ty>,
    Function: Function,
    Encoding: felt252,
    Felt252Dict: Box<Ty>,
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
