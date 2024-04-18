use core::{convert::identity, fmt, ops::Not};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// A parsed JSON literal.
pub enum ParsedLiteral {
    /// A JSON `true`.
    True,
    /// A JSON `false`.
    False,
    /// A JSON `null`.
    Null,
}

impl ParsedLiteral {
    #[must_use]
    #[inline]
    /// Get the literal as a [`prim@bool`].
    ///
    /// If the literal is [`Self::Null`], [`None`] is returned.
    pub const fn as_bool(self) -> Option<bool> {
        match self {
            Self::True => Some(true),
            Self::False => Some(false),
            Self::Null => None,
        }
    }

    #[must_use]
    #[inline]
    /// Returns [`prim@true`] if the literal is [`Self::Null`].
    pub const fn is_null(self) -> bool {
        matches!(self, Self::Null)
    }

    #[must_use]
    #[inline]
    /// Returns the literal as a string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::True => "true",
            Self::False => "false",
            Self::Null => "null",
        }
    }
}

impl From<ParsedLiteral> for Option<bool> {
    #[inline]
    fn from(value: ParsedLiteral) -> Self {
        value.as_bool()
    }
}

impl fmt::Display for ParsedLiteral {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq<bool> for ParsedLiteral {
    #[inline]
    fn eq(&self, other: &bool) -> bool {
        match self {
            Self::True => *other,
            Self::False => !other,
            Self::Null => false,
        }
    }
}

impl PartialEq<Option<bool>> for ParsedLiteral {
    #[inline]
    fn eq(&self, other: &Option<bool>) -> bool {
        match self {
            Self::True => other.map_or(false, identity),
            Self::False => other.map_or(false, Not::not),
            Self::Null => other.is_none(),
        }
    }
}
