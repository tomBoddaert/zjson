use core::fmt;

use crate::{any, array, literal, number, object, string};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// The error returned when parsing a [`MultiDocument`](super::MultiDocument) fails.
pub enum ParseMultiDocumentError {
    /// A character that was not the start of a valid element was found.
    InvalidElement(char),
}

impl fmt::Display for ParseMultiDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidElement(c) => write!(
                f,
                "Invalid character ({c}) in JSON document (expected an element)!"
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseMultiDocumentError {}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// The error returned when finishing parsing a [`MultiDocument`](super::MultiDocument) fails.
pub enum ParseAnyMultiDocumentError {
    /// Parsing the document failed.
    MultiDocument(ParseMultiDocumentError),
    /// Parsing a child failed.
    Any(any::ParseAnyError),
}

impl fmt::Display for ParseAnyMultiDocumentError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MultiDocument(err) => err.fmt(f),
            Self::Any(err) => err.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseAnyMultiDocumentError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::MultiDocument(err) => err,
            Self::Any(err) => err,
        })
    }
}

impl From<ParseMultiDocumentError> for ParseAnyMultiDocumentError {
    #[inline]
    fn from(value: ParseMultiDocumentError) -> Self {
        Self::MultiDocument(value)
    }
}

impl From<any::ParseAnyError> for ParseAnyMultiDocumentError {
    #[inline]
    fn from(value: any::ParseAnyError) -> Self {
        Self::Any(value)
    }
}

impl From<string::ParseStringError> for ParseAnyMultiDocumentError {
    #[inline]
    fn from(value: string::ParseStringError) -> Self {
        Self::Any(value.into())
    }
}

impl From<number::ParseNumberError> for ParseAnyMultiDocumentError {
    #[inline]
    fn from(value: number::ParseNumberError) -> Self {
        Self::Any(value.into())
    }
}

impl From<object::ParseObjectError> for ParseAnyMultiDocumentError {
    #[inline]
    fn from(value: object::ParseObjectError) -> Self {
        Self::Any(value.into())
    }
}

impl From<array::ParseArrayError> for ParseAnyMultiDocumentError {
    #[inline]
    fn from(value: array::ParseArrayError) -> Self {
        Self::Any(value.into())
    }
}

impl From<literal::ParseLiteralError> for ParseAnyMultiDocumentError {
    #[inline]
    fn from(value: literal::ParseLiteralError) -> Self {
        Self::Any(value.into())
    }
}
