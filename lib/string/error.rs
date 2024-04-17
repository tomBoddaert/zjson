use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// The error returned when parsing a [`String`](super::String) fails.
pub enum ParseStringError {
    /// The JSON string ended before the string was terminated.
    UnexpectedEnd,
    /// An invalid character was escaped.
    InvalidEscape(char),
    /// A non-hex character was used in a unicode escape.
    InvalidUnicodeEscape(char),
    /// A low surrogate was found that was not prefixed by a high surrogate.
    MissingHighSurrogate {
        /// The low surrogate found.
        low: u16,
    },
    /// A high surrogate was found that was not followed by a low surrogate.
    MissingLowSurrogate {
        /// The high surrogate found.
        high: u16,
    },
    /// A unicode escape sequence was found after a high surrogate but it was not a valid low surrogate.
    InvalidLowSurrogate {
        /// The high surrogate found.
        high: u16,
        /// The unicode escape sequence.
        low: u16,
    },
}

impl fmt::Display for ParseStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEnd => {
                write!(f, "Unexpected end of JSON string (missing \")!")
            }
            Self::InvalidEscape(c) => write!(f, "Invalid escape character ({c}) in JSON string!"),
            Self::InvalidUnicodeEscape(c) => {
                write!(
                    f,
                    "Invalid character ({c}) in unicode escape in JSON string!"
                )
            }
            Self::MissingHighSurrogate { low } => {
                write!(
                    f,
                    "Found a low surrogate (\\u{low:0>4x}) not prefixed with a high surrogate!"
                )
            }
            Self::MissingLowSurrogate { high } => {
                write!(
                    f,
                    "Found a high surrogate (\\u{high:0>4x}) not followed by a low surrogate!"
                )
            }
            Self::InvalidLowSurrogate { high, low } => {
                write!(f, "Invalid low surrogate (\\u{low:0>4x}) after a high surrogate (\\u{high:0>4x}!)")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseStringError {}
