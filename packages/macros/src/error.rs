use thiserror::Error;

#[derive(Error, Debug)]
pub enum IntrospectError {
    #[error("'{0}' cannot be introspected. Only structs and enums are supported.")]
    UnsupportedItem(String),
    #[error("'{0}' is not  a struct.")]
    NotAStruct(String),
    #[error("Derive '{0}' not unnamed variant.")]
    WrongDeriveVariant(String),
    #[error("Derive macro must have arguments.")]
    DeriveMacroMissingArgs,
}

pub type Result<T> = std::result::Result<T, IntrospectError>;
