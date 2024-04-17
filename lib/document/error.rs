use core::fmt;

use crate::{any, array, literal, number, object, string};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// The error returned when parsing a [`Document`](super::Document) fails.
pub enum ParseDocumentError {
    /// The JSON string ended before a value was found.
    UnexpectedEnd,
    /// The first non-whitespace character is not the start of a valid value.
    InvalidElement(char),
    /// A non-whitespace character was found after the first value.
    UnexpectedCharacter(char),
}

impl fmt::Display for ParseDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEnd => write!(
                f,
                "Unexpected end of JSON document (expected a JSON value)!"
            ),
            Self::InvalidElement(c) => write!(
                f,
                "Invalid character ({c}) in JSON document (expected an element)!"
            ),
            Self::UnexpectedCharacter(c) => {
                write!(f, "Unexpected character ({c}) at the end of JSON document!")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseDocumentError {}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// The error returned when finishing parsing a [`Document`](super::Document) fails.
pub enum ParseAnyDocumentError {
    /// Parsing the document failed.
    Document(ParseDocumentError),
    /// Parsing a child failed.
    Any(any::ParseAnyError),
}

impl fmt::Display for ParseAnyDocumentError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Document(err) => err.fmt(f),
            Self::Any(err) => err.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseAnyDocumentError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::Document(err) => err,
            Self::Any(err) => err,
        })
    }
}

impl From<ParseDocumentError> for ParseAnyDocumentError {
    #[inline]
    fn from(value: ParseDocumentError) -> Self {
        Self::Document(value)
    }
}

impl From<any::ParseAnyError> for ParseAnyDocumentError {
    #[inline]
    fn from(value: any::ParseAnyError) -> Self {
        Self::Any(value)
    }
}

impl From<string::ParseStringError> for ParseAnyDocumentError {
    #[inline]
    fn from(value: string::ParseStringError) -> Self {
        Self::Any(value.into())
    }
}

impl From<number::ParseNumberError> for ParseAnyDocumentError {
    #[inline]
    fn from(value: number::ParseNumberError) -> Self {
        Self::Any(value.into())
    }
}

impl From<object::ParseObjectError> for ParseAnyDocumentError {
    #[inline]
    fn from(value: object::ParseObjectError) -> Self {
        Self::Any(value.into())
    }
}

impl From<array::ParseArrayError> for ParseAnyDocumentError {
    #[inline]
    fn from(value: array::ParseArrayError) -> Self {
        Self::Any(value.into())
    }
}

impl From<literal::ParseLiteralError> for ParseAnyDocumentError {
    #[inline]
    fn from(value: literal::ParseLiteralError) -> Self {
        Self::Any(value.into())
    }
}
