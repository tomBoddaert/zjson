use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// The error returned when parsing an [`Array`](super::Array) fails.
pub enum ParseArrayError {
    /// The JSON string ended before the array was terminated.
    UnexpectedEnd,
    /// A character that was not the start of an element was found where an element was expected.
    InvalidElement {
        /// The character found.
        c: char,
        /// If the array could have been terminated here.
        or_end: bool,
    },
    /// A character that was not a comma or a terminator (']') was found directly after a value.
    ExpectedCommaOrEnd(char),
    /// The array was terminated directly after a comma.
    TrailingComma,
}

impl fmt::Display for ParseArrayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEnd => write!(f, "Unexpected end of JSON array!"),
            Self::InvalidElement { c, or_end: true } => write!(
                f,
                "Invalid character ({c}) in JSON array (expected an element or an end, ']')!"
            ),
            Self::InvalidElement { c, or_end: false } => write!(
                f,
                "Invalid character ({c}) in JSON array (expected an element)!"
            ),
            Self::ExpectedCommaOrEnd(c) => write!(
                f,
                "Invalid character ({c}) in JSON array (expected a comma or an end, ']')"
            ),
            Self::TrailingComma => write!(f, "Trailing comma in JSON array!"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseArrayError {}
