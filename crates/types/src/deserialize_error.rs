use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// Input ended unexpectedly.
    Eof,

    /// A tag/value was not one of the expected encodings for the given construct.
    /// Examples: option tag not in {0,1}, bool not in {0,1}, result tag not in {0,1}.
    InvalidTag { what: &'static str, value: u128 },

    /// A length/size value is invalid for the given construct.
    /// Examples: length > remaining, length exceeds configured max, negative via encoding, etc.
    InvalidLen {
        what: &'static str,
        len: usize,
        max: Option<usize>,
    },

    /// A value does not fit the target type's range/encoding.
    /// Examples: felt doesn't fit u8/u16/u32/u64/usize, bytes31 pending_len > 31.
    OutOfRange { what: &'static str, value: u128 },

    /// ByteArray / string decoding produced invalid UTF-8.
    InvalidUtf8,

    /// An internal invariant was violated (bug or inconsistent encoding).
    InvariantViolation(&'static str),

    /// Extra data remained after a `deserialize_end`-style decode.
    TrailingData { remaining: usize },

    /// Catch-all for permanent/structural errors with a static message.
    Message(&'static str),
}

impl DecodeError {
    #[inline]
    pub fn invalid_tag(what: &'static str, value: u128) -> Self {
        Self::InvalidTag { what, value }
    }

    #[inline]
    pub fn invalid_len(what: &'static str, len: usize, max: Option<usize>) -> Self {
        Self::InvalidLen { what, len, max }
    }

    #[inline]
    pub fn out_of_range(what: &'static str, value: u128) -> Self {
        Self::OutOfRange { what, value }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::Eof => write!(f, "unexpected end of input"),
            DecodeError::InvalidTag { what, value } => {
                write!(f, "invalid tag for {what}: {value}")
            }
            DecodeError::InvalidLen { what, len, max } => match max {
                Some(max) => write!(f, "invalid length for {what}: {len} (max {max})"),
                None => write!(f, "invalid length for {what}: {len}"),
            },
            DecodeError::OutOfRange { what, value } => {
                write!(f, "value out of range for {what}: {value}")
            }
            DecodeError::InvalidUtf8 => write!(f, "invalid UTF-8"),
            DecodeError::InvariantViolation(msg) => write!(f, "invariant violation: {msg}"),
            DecodeError::TrailingData { remaining } => {
                write!(
                    f,
                    "trailing data after decode: {remaining} item(s) remaining"
                )
            }
            DecodeError::Message(msg) => write!(f, "{msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}
