use core::{fmt, hash, str};
#[cfg(feature = "alloc")]
extern crate alloc;

use crate::status::Status;

use super::machine::EscapeMachine;

#[derive(Clone, Copy)]
/// A parsed JSON string.
pub struct ParsedString<'json> {
    json: &'json str,
}

impl<'json> ParsedString<'json> {
    #[must_use]
    #[inline]
    pub(super) const fn new(json: &'json str) -> Self {
        Self { json }
    }

    #[must_use]
    #[inline]
    /// Returns the unescaped string.
    /// This will be the same as the JSON string without the quotation marks.
    pub const fn unescaped(self) -> &'json str {
        self.json
    }

    #[must_use]
    #[inline]
    /// Returns an iterator over the characters in the escaped string.
    pub fn chars(self) -> Chars<'json> {
        Chars {
            json: self.json.chars(),
        }
    }

    #[cfg(feature = "alloc")]
    #[must_use]
    #[inline]
    /// Collects the escaped string into a [`String`](alloc::string::String).
    pub fn escaped(self) -> alloc::string::String {
        self.chars().collect()
    }
}

impl<'json> fmt::Debug for ParsedString<'json> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("string::Parsed").field(&self.json).finish()
    }
}

impl<'json> fmt::Display for ParsedString<'json> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.chars() {
            write!(f, "{c}")?;
        }

        Ok(())
    }
}

impl<'json> PartialEq<str> for ParsedString<'json> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.chars().eq(other.chars())
    }
}

impl<'json> PartialEq<&str> for ParsedString<'json> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl<'json> PartialEq<ParsedString<'_>> for ParsedString<'json> {
    #[inline]
    fn eq(&self, other: &ParsedString<'_>) -> bool {
        self.chars().eq(other.chars())
    }
}
impl<'json> Eq for ParsedString<'json> {}

impl<'json> hash::Hash for ParsedString<'json> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        for c in self.chars() {
            c.hash(state);
        }

        // Terminate with 0xff, like for str because we don't know the
        // length in advance
        state.write_u8(0xff);
    }
}

#[derive(Clone)]
pub struct Chars<'json> {
    json: str::Chars<'json>,
}

impl<'json> Iterator for Chars<'json> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.json.next()?;

        if c != '\\' {
            return Some(c);
        }

        let mut machine = EscapeMachine::Awaiting;

        for c in &mut self.json {
            match machine
                .apply(c)
                .expect("failed to parse an escape in a parsed string")
            {
                Status::Parsing(next) => machine = next,
                Status::Done(result) => {
                    return Some(result);
                }
            }
        }

        panic!("ran out of characters whilst parsing an escape in a parsed string");
    }
}

impl<'json> fmt::Debug for Chars<'json> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParsedString")
            .field("remaining_unescaped", &self.json)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::ParsedString;

    #[test]
    fn test_surrogate_pair() {
        let unescaped = r"\ud83d\ude03";
        let parsed = ParsedString::new(unescaped);
        assert_eq!(parsed, "😃");
    }
}
