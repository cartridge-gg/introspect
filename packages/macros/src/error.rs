use thiserror::Error;


#[derive(Error, Debug)]
pub enum IntrospectError{
    #[error("The provided type '{0}' is not introspectable. Only structs and enums are supported.")]
    TypeNotIntrospectable(String),
}

pub type Result<T> = std::result::Result<T, IntrospectError>;