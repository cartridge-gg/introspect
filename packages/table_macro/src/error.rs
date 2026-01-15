use introspect_macros::IntrospectError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TableError {
    #[error("introspect error: {0}")]
    IntrospectError(#[from] IntrospectError),
    #[error("Keys must come before other columns in the struct.")]
    KeysNotFirst,
    #[error("Duplicate column attribute found")]
    DuplicateColumnAttribute,
    #[error("Parsing Column id")]
    ColumnIdParseError,
}

pub type Result<T> = std::result::Result<T, TableError>;
