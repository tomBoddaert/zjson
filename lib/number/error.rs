use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// The error returned when parsing a [`Number`](super::Number) fails.
pub enum ParseNumberError {
    /// The JSON string ended before the number was completed.
    UnexpectedEnd {
        /// If the next character could have been a minus sign.
        or_sign: bool,
    },
    /// The JSON string ended before the exponent (after an `e` or `E`).
    UnexpectedEndAfterExponent {
        /// If the next character could have been a sign.
        or_sign: bool,
    },
    /// A different character was found when a minus sign or digit was expected.
    ExpectedMinusOrDigit(char),
    /// A different character was found when a digit was expected.
    ExpectedDigit(char),
    /// A different character was found when a sign or digit was expected.
    ExpectedSignOrDigit(char),
}

impl fmt::Display for ParseNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEnd { or_sign: true } => {
                write!(
                    f,
                    "Unexpected end of JSON number (expected minus sign or digit)!"
                )
            }
            Self::UnexpectedEnd { or_sign: false } => {
                write!(f, "Unexpected end of JSON number (expected digit)!")
            }
            Self::UnexpectedEndAfterExponent { or_sign: true } => {
                write!(
                    f,
                    "Unexpected end of JSON number exponent (expected sign or digit)!"
                )
            }
            Self::UnexpectedEndAfterExponent { or_sign: false } => {
                write!(
                    f,
                    "Unexpected end of JSON number exponent (expected digit)!"
                )
            }
            Self::ExpectedMinusOrDigit(c) => {
                write!(
                    f,
                    "Invalid character ({c}) in JSON number (expected minus sign or digit)!"
                )
            }
            Self::ExpectedDigit(c) => {
                write!(
                    f,
                    "Invalid character ({c}) in JSON number (expected digit)!"
                )
            }
            Self::ExpectedSignOrDigit(c) => {
                write!(
                    f,
                    "Invalid character ({c}) in JSON number (expected sign or digit)!"
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseNumberError {}
