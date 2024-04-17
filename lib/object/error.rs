use core::fmt;

use crate::string;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// The error returned when parsing an [`Object`](super::Object) fails.
pub enum ParseObjectError {
    /// The JSON string ended before the object was terminated.
    UnexpectedEnd,
    /// A character that was not the start of a string was found where a name (key) was expected.
    ExpectedName {
        /// The character found.
        c: char,
        /// If the object could have been terminated here.
        or_end: bool,
    },
    /// Parsing a name (key) failed.
    InvalidName(string::ParseStringError),
    /// A different character was found where a colon was expected.
    ExpectedColon(char),
    /// A character that was not the start of an element was found where an element was expected.
    InvalidElement(char),
    /// A character that was not a comma or a terminator (`}`) was found directly after a value.
    ExpectedCommaOrEnd(char),
    /// The object was terminated directly after a comma.
    TrailingComma,
}

impl fmt::Display for ParseObjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEnd => write!(f, "Unexpected end of JSON object!"),
            Self::ExpectedName { c, or_end: true } => write!(
                f,
                "Invalid character ({c}) in JSON object (expected a string name or an end, '}}')!"
            ),
            Self::ExpectedName { c, or_end: false } => write!(
                f,
                "Invalid character ({c}) in JSON object (expected a string name)!"
            ),
            Self::InvalidName(err) => err.fmt(f),
            Self::ExpectedColon(c) => {
                write!(
                    f,
                    "Invalid character ({c}) in JSON object (expected a colon, ':')!"
                )
            }
            Self::InvalidElement(c) => write!(
                f,
                "Invalid character ({c}) in JSON object (expected a value)!"
            ),
            Self::ExpectedCommaOrEnd(c) => write!(
                f,
                "Invalid character ({c}) in JSON object (expected a comma or an end, '}}')!"
            ),
            Self::TrailingComma => write!(f, "Trailing comma in JSON object!"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseObjectError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::InvalidName(err) = self {
            Some(err)
        } else {
            None
        }
    }
}
