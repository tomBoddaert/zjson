use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// The error returned when parsing a [`Literal`](super::Literal) fails.
pub enum ParseLiteralError {
    /// The JSON string ended before the literal was finished.
    UnexpectedEnd,
    /// An invalid character was found in the literal.
    UnexpectedCharacter(char),
}

impl fmt::Display for ParseLiteralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEnd => write!(f, "Unexpected end of JSON literal!"),
            Self::UnexpectedCharacter(c) => write!(f, "Invalid character ({c}) in JSON literal!"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseLiteralError {}
