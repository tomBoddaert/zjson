use crate::{
    any::Any,
    containers::{ParsePrompt, ParseStatus},
    debug::debug_impl,
    Parent,
};

mod error;
pub use error::{ParseAnyDocumentError, ParseDocumentError};

/// A JSON document created from a string.
pub struct Document<'json> {
    remaining: &'json str,
    parse_status: Option<ParseStatus>,
}

impl<'json> Parent<'json> for Document<'json> {
    fn set_remaining<'a>(&'a mut self, remaining: &'json str)
    where
        'json: 'a,
    {
        self.remaining = remaining;
        self.parse_status = Some(ParseStatus::Done);
    }

    fn debug_parents(&self, list: &mut core::fmt::DebugList<'_, '_>) {
        list.entry(&"Document");
    }
}

impl<'json> Document<'json> {
    #[must_use]
    #[inline]
    /// Create a new JSON document from a string.
    pub const fn new(json: &'json str) -> Self {
        Self {
            remaining: json,
            parse_status: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    /// Try to get the next value from the document.
    ///
    /// This will only yield one value, after which, it will yield [`None`].
    ///
    /// # Errors
    /// If parsing fails, this will return a [`ParseDocumentError`].
    /// Parsing will fail if the first non-whitespace character does not hint at a valid value or if there are any non-whitespace characters after the first value.
    pub fn next(&mut self) -> Result<Option<Any<'json, '_>>, ParseDocumentError> {
        loop {
            let end = match self.parse_status {
                None => false,

                Some(ParseStatus::Prompted(prompt)) => {
                    let remaining = self.remaining;
                    return Ok(Some(prompt.create(self, remaining)));
                }

                Some(ParseStatus::Done) => true,
            };

            let Some(c) = self.remaining.chars().next() else {
                return if end {
                    Ok(None)
                } else {
                    Err(ParseDocumentError::UnexpectedEnd)
                };
            };

            if c.is_whitespace() {
                // do nothing
            } else if end {
                return Err(ParseDocumentError::UnexpectedCharacter(c));
            } else if let Some(prompt) = ParsePrompt::get(c) {
                self.parse_status = Some(prompt.into());

                if prompt.keep_first() {
                    continue;
                }
            } else {
                return Err(ParseDocumentError::InvalidElement(c));
            }

            self.remaining = &self.remaining[c.len_utf8()..];
        }
    }

    /// Finish parsing this document.
    /// This can be used to make sure that there are no errors after the first value.
    ///
    /// If [`Self::next`] has returned [`None`], then this does not need to be called.
    ///
    /// # Errors
    /// If parsing fails in this document or a child, the error is returned as a [`ParseAnyDocumentError`].
    pub fn finish(&mut self) -> Result<(), ParseAnyDocumentError> {
        while let Some(mut value) = self.next()? {
            value.finish()?;
        }

        Ok(())
    }

    /// Runs `f` on the element in the document.
    ///
    /// [`Any::finish`] is automatically called on all values, so it is not needed in `f`.
    ///
    /// # Errors
    /// If parsing fails in this document or if `f` returns an error, a [`ParseAnyDocumentError`] is returned.
    pub fn for_each<F>(&mut self, mut f: F) -> Result<(), ParseAnyDocumentError>
    where
        F: FnMut(&mut Any<'json, '_>) -> Result<(), ParseAnyDocumentError>,
    {
        while let Some(mut value) = self.next()? {
            f(&mut value)?;
            value.finish()?;
        }

        Ok(())
    }

    /// Applies `f` to the accumulator, passing in the element in the document.
    ///
    /// The initial value of the accumulator is `init`.
    ///
    /// [`Any::finish`] is automatically called on the value, so it is not needed in `f`.
    ///
    /// # Errors
    /// If parsing fails in this document or if `f` returns an error, a [`ParseAnyDocumentError`] is returned.
    pub fn fold<B, F>(&mut self, init: B, mut f: F) -> Result<B, ParseAnyDocumentError>
    where
        F: FnMut(B, &mut Any<'json, '_>) -> Result<B, ParseAnyDocumentError>,
    {
        let mut accumulator = init;

        while let Some(mut value) = self.next()? {
            accumulator = f(accumulator, &mut value)?;
            value.finish()?;
        }

        Ok(accumulator)
    }

    /// Runs `f` on the element in the document.
    ///
    /// [`Any::finish`] is automatically called on the value, so it is not needed in `f`.
    ///
    /// # Errors
    /// If parsing fails in this document or if `f` returns an error, a [`ParseAnyDocumentError`] is returned.
    pub fn find<B, F>(&mut self, mut f: F) -> Result<Option<B>, ParseAnyDocumentError>
    where
        F: FnMut(&mut Any<'json, '_>) -> Result<Option<B>, ParseAnyDocumentError>,
    {
        while let Some(mut value) = self.next()? {
            let result = f(&mut value)?;
            value.finish()?;

            if result.is_some() {
                return Ok(result);
            }
        }

        Ok(None)
    }
}

debug_impl!("Document", Document<'json>, no_parents);

#[cfg(test)]
mod test {
    use super::{Document, ParseDocumentError};

    #[test]
    fn parse_string() {
        let expected = "Hello, World!";
        let json = format!("\"{expected}\"");

        let mut document = Document::new(&json);

        let parsed = document
            .next()
            .expect("failed to parse document")
            .expect("got no values in document")
            .string()
            .expect("expected string from document")
            .get()
            .expect("failed to parse string");

        assert_eq!(parsed, expected);
        assert!(document.next().expect("failed to parse document").is_none());
    }

    #[test]
    fn multiple_values() {
        let expected = "Hello, World!";
        let json = format!("\"{expected}\"\"s2\"");

        let mut document = Document::new(&json);

        let parsed = document
            .next()
            .expect("failed to parse document")
            .expect("got no values in document")
            .string()
            .expect("expected string from document")
            .get()
            .expect("failed to parse string");

        assert_eq!(parsed, expected);

        let error = document
            .next()
            .expect_err("failed to return error after parsing invalid document");

        assert_eq!(error, ParseDocumentError::UnexpectedCharacter('"'));
    }

    #[test]
    fn empty() {
        let error = Document::new("")
            .next()
            .expect_err("failed to return error after parsing empty document");

        assert_eq!(error, ParseDocumentError::UnexpectedEnd);
    }

    #[test]
    fn parse_invalid() {
        let invalid = 'j';
        let json = invalid.to_string();
        let mut document = Document::new(&json);

        let error = document
            .next()
            .expect_err("failed to return error after parsing invalid document");

        assert_eq!(error, ParseDocumentError::InvalidElement(invalid));
    }

    #[test]
    fn invalid_after_value() {
        let expected = "Hello, World!";
        let invalid = 'j';
        let json = format!("\"{expected}\"{invalid}");

        let mut document = Document::new(&json);

        let parsed = document
            .next()
            .expect("failed to parse document")
            .expect("got no values in document")
            .string()
            .expect("expected string from document")
            .get()
            .expect("failed to parse string");

        assert_eq!(parsed, expected);

        let error = document
            .next()
            .expect_err("failed to return error after parsing invalid document");

        assert_eq!(error, ParseDocumentError::UnexpectedCharacter(invalid));
    }
}
