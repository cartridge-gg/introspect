use starknet_types_core::felt::Felt;
use thiserror::Error;

pub type DecodeResult<T> = Result<T, DecodeError>;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DecodeError {
    #[error("unexpected end of input")]
    Eof,

    #[error("expected end of input, but more data is available")]
    NotEof,

    #[error("invalid tag for {what}: {value}")]
    InvalidTag { what: &'static str, value: String },

    #[error("invalid length for {what}: {len} (max {max:?})")]
    InvalidLen {
        what: &'static str,
        len: usize,
        max: Option<usize>,
    },

    #[error("value out of range for {target}: {value:?}")]
    OutOfRangeFeltConversion { target: &'static str, value: Felt },

    #[error("invalid byte array: {0}")]
    InvalidByteArray(#[from] ByteArrayError),

    #[error("invalid Bytes31 encoding: {0}")]
    InvalidBytes31Encoding(&'static str),

    #[error("invalid UTF-8 (valid up to {valid_up_to}, error length {error_len:?})")]
    Utf8 {
        valid_up_to: usize,
        error_len: Option<usize>,
    },

    #[error("trailing data after decode: {remaining} item(s) remaining")]
    TrailingData { remaining: usize },

    #[error("invariant violation: {0}")]
    InvariantViolation(&'static str),

    #[error("invalid {what} encoding")]
    InvalidEncoding { what: &'static str },

    #[error("failed to convert Felt into primitive type: {what}")]
    PrimitiveFromFelt { what: &'static str },

    #[error("invalid enum selector for {what}: {value:?}")]
    InvalidEnumSelector { what: &'static str, value: Felt },

    #[error("unexpected length for {what}: expected {expected}, got {got}")]
    UnexpectedLen {
        what: &'static str,
        expected: usize,
        got: usize,
    },

    #[error("{0}")]
    Message(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ByteArrayError {
    #[error("pending length out of range: {len}")]
    PendingLenOutOfRange { len: usize },

    #[error("invalid chunk length: {len}")]
    InvalidChunkLen { len: usize },

    #[error("non-canonical encoding: {0}")]
    NonCanonicalEncoding(&'static str),
}

impl DecodeError {
    #[inline]
    pub fn out_of_range<T>(value: Felt) -> Self {
        Self::OutOfRangeFeltConversion {
            target: core::any::type_name::<T>(),
            value,
        }
    }

    #[inline]
    pub fn invalid_tag<T: ToString>(what: &'static str, value: T) -> Self {
        Self::InvalidTag {
            what,
            value: value.to_string(),
        }
    }

    #[inline]
    pub fn invalid_len(what: &'static str, len: usize, max: Option<usize>) -> Self {
        Self::InvalidLen { what, len, max }
    }

    #[inline]
    pub fn trailing_data(remaining: usize) -> Self {
        Self::TrailingData { remaining }
    }

    #[inline]
    pub fn invalid_enum_selector<T: Into<Felt>>(what: &'static str, value: T) -> Self {
        Self::InvalidEnumSelector {
            what,
            value: value.into(),
        }
    }

    #[inline]
    pub fn unexpected_len(what: &'static str, expected: usize, got: usize) -> Self {
        Self::UnexpectedLen {
            what,
            expected,
            got,
        }
    }
}

impl From<core::str::Utf8Error> for DecodeError {
    #[inline]
    fn from(e: core::str::Utf8Error) -> Self {
        Self::Utf8 {
            valid_up_to: e.valid_up_to(),
            error_len: e.error_len(),
        }
    }
}
