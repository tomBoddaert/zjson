use core::fmt;

use crate::{
    array::ParseArrayError, literal::ParseLiteralError, number::ParseNumberError,
    object::ParseObjectError, string::ParseStringError,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// An error from parsing any JSON type.
pub enum ParseAnyError {
    /// A [`ParseStringError`] from parsing a [`String`](crate::string::String).
    String(ParseStringError),
    /// A [`ParseNumberError`] from parsing a [`Number`](crate::number::Number).
    Number(ParseNumberError),
    /// A [`ParseObjectError`] from parsing an [`Object`](crate::object::Object).
    Object(ParseObjectError),
    /// A [`ParseArrayError`] from parsing an [`Array`](crate::array::Array).
    Array(ParseArrayError),
    /// A [`ParseLiteralError`] from parsing a [`Literal`](crate::literal::Literal).
    Literal(ParseLiteralError),
}

impl fmt::Display for ParseAnyError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(err) => err.fmt(f),
            Self::Number(err) => err.fmt(f),
            Self::Object(err) => err.fmt(f),
            Self::Array(err) => err.fmt(f),
            Self::Literal(err) => err.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseAnyError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::String(err) => err,
            Self::Number(err) => err,
            Self::Object(err) => err,
            Self::Array(err) => err,
            Self::Literal(err) => err,
        })
    }
}

impl From<ParseStringError> for ParseAnyError {
    #[inline]
    fn from(value: ParseStringError) -> Self {
        Self::String(value)
    }
}

impl From<ParseNumberError> for ParseAnyError {
    #[inline]
    fn from(value: ParseNumberError) -> Self {
        Self::Number(value)
    }
}

impl From<ParseObjectError> for ParseAnyError {
    #[inline]
    fn from(value: ParseObjectError) -> Self {
        Self::Object(value)
    }
}

impl From<ParseArrayError> for ParseAnyError {
    #[inline]
    fn from(value: ParseArrayError) -> Self {
        Self::Array(value)
    }
}

impl From<ParseLiteralError> for ParseAnyError {
    #[inline]
    fn from(value: ParseLiteralError) -> Self {
        Self::Literal(value)
    }
}
