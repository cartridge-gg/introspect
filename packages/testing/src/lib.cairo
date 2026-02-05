pub mod types {
    pub mod attribute;
    pub mod entry;
    pub mod primary;
    pub mod type_def;
    pub use attribute::{Attribute, FuzzableAttribute};
    pub use type_def::{
        EnumDef, FixedSizeArrayDef, MemberDef, ResultDef, StructDef, TypeDef,
        TypeDefFuzzableToDepth, VariantDef,
    };
}
