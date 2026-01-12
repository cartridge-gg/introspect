use introspect_macros::IntrospectError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TableError {
    #[error("introspect error: {0}")]
    IntrospectError(#[from] IntrospectError),
}

pub type Result<T> = std::result::Result<T, TableError>;
