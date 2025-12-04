use introspect_tests::{
    ByteArrayExt, FuzzableMaxDepth, FuzzableMaxDepthNode, FuzzableMaxDepthNodeImpl, Fuzzy,
    random_pascal_string, random_snake_string,
};
use introspect_types::{
    EnumDef, FixedArrayDef, MemberDef, ResultDef, StructDef, TypeDef, VariantDef,
};
use snforge_std::fuzzable::{Fuzzable, generate_arg};
use super::attribute::FuzzableAttribute;

pub impl TypeDefFuzzableToDepth<const MAX_DEPTH: u32> of Fuzzable<TypeDef> {
    fn blank() -> TypeDef {
        TypeDef::None
    }

    fn generate() -> TypeDef {
        FuzzableMaxDepth::<TypeDef>::generate(MAX_DEPTH)
    }
}


impl TypeDefFuzzableNode of FuzzableMaxDepthNode<TypeDef> {
    fn leaf() -> TypeDef {
        match generate_arg(0, 25) {
            0 => TypeDef::None,
            1 => TypeDef::Felt252,
            2 => TypeDef::ShortUtf8,
            3 => TypeDef::Bytes31,
            4 => TypeDef::Bytes31E("ascii"),
            5 => TypeDef::Bool,
            6 => TypeDef::U8,
            7 => TypeDef::U16,
            8 => TypeDef::U32,
            9 => TypeDef::U64,
            10 => TypeDef::U128,
            11 => TypeDef::U256,
            12 => TypeDef::U512,
            13 => TypeDef::I8,
            14 => TypeDef::I16,
            15 => TypeDef::I32,
            16 => TypeDef::I64,
            17 => TypeDef::I128,
            18 => TypeDef::ClassHash,
            19 => TypeDef::ContractAddress,
            20 => TypeDef::EthAddress,
            21 => TypeDef::StorageAddress,
            22 => TypeDef::StorageBaseAddress,
            23 => TypeDef::ByteArray,
            24 => TypeDef::Utf8String,
            25 => TypeDef::ByteArrayE("ascii"),
            _ => panic!("Unreachable"),
        }
    }


    fn node(depth_rem: u32) -> TypeDef {
        match generate_arg(0, 36) {
            0 => TypeDef::None,
            1 => TypeDef::Felt252,
            2 => TypeDef::ShortUtf8,
            3 => TypeDef::Bytes31,
            4 => TypeDef::Bytes31E("ascii"),
            5 => TypeDef::Bool,
            6 => TypeDef::U8,
            7 => TypeDef::U16,
            8 => TypeDef::U32,
            9 => TypeDef::U64,
            10 => TypeDef::U128,
            11 => TypeDef::U256,
            12 => TypeDef::U512,
            13 => TypeDef::I8,
            14 => TypeDef::I16,
            15 => TypeDef::I32,
            16 => TypeDef::I64,
            17 => TypeDef::I128,
            18 => TypeDef::ClassHash,
            19 => TypeDef::ContractAddress,
            20 => TypeDef::EthAddress,
            21 => TypeDef::StorageAddress,
            22 => TypeDef::StorageBaseAddress,
            23 => TypeDef::ByteArray,
            24 => TypeDef::Utf8String,
            25 => TypeDef::ByteArrayE("ascii"),
            26 => TypeDef::Tuple(TypeDefFuzzable::generate_span_lt(depth_rem, 16)),
            27 => TypeDef::Array(TypeDefFuzzable::generate_boxed(depth_rem)),
            28 => TypeDef::FixedArray(FixedArrayFuzzable::generate_boxed(depth_rem)),
            29 => TypeDef::Felt252Dict(TypeDefFuzzable::generate_boxed(depth_rem)),
            30 => TypeDef::Struct(StructDefFuzzable::generate(depth_rem)),
            31 => TypeDef::Enum(EnumDefFuzzable::generate(depth_rem)),
            32 => TypeDef::Ref(Fuzzable::generate()),
            33 => TypeDef::Custom("Custom"),
            34 => TypeDef::Option(TypeDefFuzzable::generate_boxed(depth_rem)),
            35 => TypeDef::Result(ResultFuzzable::generate_boxed(depth_rem)),
            36 => TypeDef::Nullable(TypeDefFuzzable::generate_boxed(depth_rem)),
            _ => panic!("Unreachable"),
        }
    }
}

pub impl TypeDefFuzzable = FuzzableMaxDepthNodeImpl<TypeDef, TypeDefFuzzableNode>;

impl FixedArrayFuzzable of FuzzableMaxDepth<FixedArrayDef> {
    fn generate(depth_rem: u32) -> FixedArrayDef {
        FixedArrayDef { type_def: TypeDefFuzzable::generate(depth_rem), size: generate_arg(0, 16) }
    }
}

impl ResultFuzzable of FuzzableMaxDepth<ResultDef> {
    fn generate(depth_rem: u32) -> ResultDef {
        ResultDef {
            ok: TypeDefFuzzable::generate(depth_rem), err: TypeDefFuzzable::generate(depth_rem),
        }
    }
}

impl StructDefFuzzable of FuzzableMaxDepth<StructDef> {
    fn generate(depth_rem: u32) -> StructDef {
        StructDef {
            name: random_pascal_string(31, 4),
            attributes: Fuzzy::generate_span_lt(16),
            members: MemberDefFuzzable::generate_span_lt(depth_rem, 10),
        }
    }
}

impl MemberDefFuzzable of FuzzableMaxDepth<MemberDef> {
    fn generate(depth_rem: u32) -> MemberDef {
        MemberDef {
            name: random_snake_string(31, 4),
            attributes: Fuzzy::generate_span_lt(16),
            type_def: TypeDefFuzzable::generate(depth_rem),
        }
    }
}


impl VariantDefFuzzable of FuzzableMaxDepth<VariantDef> {
    fn generate(depth_rem: u32) -> VariantDef {
        let name = random_pascal_string(31, 4);
        let selector = name.selector();
        VariantDef {
            name,
            selector,
            attributes: Fuzzy::generate_span_lt(16),
            type_def: TypeDefFuzzable::generate(depth_rem),
        }
    }
}

impl EnumDefFuzzable of FuzzableMaxDepth<EnumDef> {
    fn generate(depth_rem: u32) -> EnumDef {
        let variants_count = generate_arg(1, 10);
        EnumDef {
            name: random_pascal_string(31, 4),
            attributes: Fuzzy::generate_span_lt(16),
            variants: VariantDefFuzzable::generate_span(depth_rem, variants_count),
        }
    }
}

