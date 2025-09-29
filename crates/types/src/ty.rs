pub struct Field {
    pub selector: String,
    pub name: String,
    pub attrs: Vec<String>,
    pub ty: Ty,
}

pub struct Struct {
    pub name: String,
    pub attrs: Vec<String>,
    pub children: Vec<Member>,
}

pub struct Enum {
    pub name: String,
    pub attrs: Vec<String>,
    pub children: Vec<Field>,
}

pub struct Member {
    pub name: String,
    pub attrs: Vec<String>,
    pub ty: Ty,
}

pub struct Function {
    pub name: String,
    pub attrs: Vec<String>,
    pub args: Vec<Member>,
    pub ret: Box<Ty>,
}

pub enum Ty {
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
    Tuple(Box<Ty>),
    Array(Box<Ty>),
    FixedArray(Box<Ty>, u32),
    Felt252Dict(Box<Ty>),
    Struct(Struct),
    Enum(Enum),
    Schema(String),
    Custom(String),
    Option(Box<Ty>),
    Result(Box<Ty>, Box<Ty>),
    Nullable(Box<Ty>),
    Function(Function),
    Encoding(String),
}

impl Ty {
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Ty::Felt252
                | Ty::Bool
                | Ty::Uint8
                | Ty::Uint16
                | Ty::Uint32
                | Ty::Uint64
                | Ty::Uint128
                | Ty::Uint256
                | Ty::Int8
                | Ty::Int16
                | Ty::Int32
                | Ty::Int64
                | Ty::Int128
                | Ty::USize
                | Ty::ShortString
                | Ty::ClassHash
                | Ty::ContractAddress
                | Ty::EthAddress
        )
    }

    fn parse(&self, ) -> 
}
