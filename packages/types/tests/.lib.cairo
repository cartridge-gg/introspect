pub mod fuzzable {
    pub mod attribute;
    pub mod schema;
    pub mod type_def;
    pub use attribute::FuzzableAttribute;
    pub use schema::FuzzableColumnDef;
    pub use type_def::{TypeDefFuzzable, TypeDefFuzzableToDepth};
}
mod iserde {
    mod attribute;
    mod byte_array;
    mod type_def;
}
