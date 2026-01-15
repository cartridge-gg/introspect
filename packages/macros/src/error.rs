use thiserror::Error;

#[derive(Error, Debug)]
pub enum IntrospectError {
    #[error("'{0}' cannot be introspected. Only structs and enums are supported.")]
    UnsupportedItem(String),
    #[error("Could not find item to introspect.")]
    NoItem(),
    #[error("Could not find struct")]
    NoStruct(),
    #[error("Could not find enum")]
    NoEnum(),
    #[error("Derive '{0}' not unnamed variant.")]
    WrongDeriveVariant(String),
    #[error("Derive macro must have arguments.")]
    DeriveMacroMissingArgs,
    #[error("Invalid format for derives argument.")]
    InvalidDerivesArgumentFormat,
    #[error("attribute '{0}' has Invalid format.")]
    InvalidIntrospectAttributeFormat(String),
    #[error("Failed to parse type.")]
    FailedToParseType,
    #[error("Only ByteArray and Bytes31 core types are supported for raw types.")]
    UnsupportedRawType,
    #[error("Only ByteArray and Bytes31 core types are supported for encoded types.")]
    UnsupportedEncodedType,
    #[error("Mutiple type modifiers found on type.")]
    MultipleTypeModifiers,
    #[error("Derive attribute not supported for introspect attributes macros.")]
    DeriveCallNotSupported,
    #[error("Duplicate attribute '{0}'.")]
    DuplicateAttribute(String),
}

pub type IntrospectResult<T> = std::result::Result<T, IntrospectError>;
