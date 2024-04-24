use crate::{
    any::Any,
    containers::{ParsePrompt, ParseStatus},
    debug::debug_impl,
    Parent,
};

mod error;
pub use error::{ParseAnyMultiDocumentError, ParseMultiDocumentError};

/// A JSON document created from a string with multiple elements (or none).
pub struct MultiDocument<'json> {
    remaining: &'json str,
    parse_status: ParseStatus,
}

impl<'json> Parent<'json> for MultiDocument<'json> {
    fn set_remaining<'a>(&'a mut self, remaining: &'json str)
    where
        'json: 'a,
    {
        self.remaining = remaining;
        self.parse_status = ParseStatus::Done;
    }

    fn debug_parents(&self, list: &mut core::fmt::DebugList<'_, '_>) {
        list.entry(&"Document");
    }
}

impl<'json> MultiDocument<'json> {
    #[must_use]
    #[inline]
    /// Create a new JSON multi-document from a string.
    pub const fn new(json: &'json str) -> Self {
        Self {
            remaining: json,
            parse_status: ParseStatus::Done,
        }
    }

    #[allow(clippy::should_implement_trait)]
    /// Try to get the next value from the multi-document.
    ///
    /// # Errors
    /// If parsing fails, this will return a [`ParseMultiDocumentError`].
    pub fn next(&mut self) -> Result<Option<Any<'json, '_>>, ParseMultiDocumentError> {
        loop {
            if let ParseStatus::Prompted(prompt) = self.parse_status {
                let remaining = self.remaining;
                return Ok(Some(prompt.create(self, remaining)));
            }

            let Some(c) = self.remaining.chars().next() else {
                return Ok(None);
            };

            if c.is_whitespace() {
                // do nothing
            } else if let Some(prompt) = ParsePrompt::get(c) {
                self.parse_status = prompt.into();

                if prompt.keep_first() {
                    continue;
                }
            } else {
                return Err(ParseMultiDocumentError::InvalidElement(c));
            }

            self.remaining = &self.remaining[c.len_utf8()..];
        }
    }

    /// Finish parsing this multi-document.
    /// This can be used to make sure that there are no errors after the used values.
    ///
    /// If [`Self::next`] has returned [`None`], then this does not need to be called.
    ///
    /// # Errors
    /// If parsing fails in this document or a child, the error is returned as a [`ParseAnyMultiDocumentError`].
    pub fn finish(&mut self) -> Result<(), ParseAnyMultiDocumentError> {
        while let Some(mut value) = self.next()? {
            value.finish()?;
        }

        Ok(())
    }

    /// Runs `f` for each element in the multi-document.
    ///
    /// [`Any::finish`] is automatically called on all values, so it is not needed in `f`.
    ///
    /// # Errors
    /// If parsing fails in this multi-document or if `f` returns an error, a [`ParseAnyMultiDocumentError`] is returned.
    pub fn for_each<F>(&mut self, mut f: F) -> Result<(), ParseAnyMultiDocumentError>
    where
        F: FnMut(&mut Any<'json, '_>) -> Result<(), ParseAnyMultiDocumentError>,
    {
        while let Some(mut value) = self.next()? {
            f(&mut value)?;
            value.finish()?;
        }

        Ok(())
    }

    /// Applies `f` to the accumulator, passing in each element in the multi-document.
    ///
    /// The initial value of the accumulator is `init`.
    ///
    /// [`Any::finish`] is automatically called on all values, so it is not needed in `f`.
    ///
    /// # Errors
    /// If parsing fails in this multi-document or if `f` returns an error, a [`ParseAnyMultiDocumentError`] is returned.
    pub fn fold<B, F>(&mut self, init: B, mut f: F) -> Result<B, ParseAnyMultiDocumentError>
    where
        F: FnMut(B, &mut Any<'json, '_>) -> Result<B, ParseAnyMultiDocumentError>,
    {
        let mut accumulator = init;

        while let Some(mut value) = self.next()? {
            accumulator = f(accumulator, &mut value)?;
            value.finish()?;
        }

        Ok(accumulator)
    }

    /// Runs `f` on each element in the multi-document, stopping if `f` returns [`Some`].
    ///
    /// [`Any::finish`] is automatically called on each value, so it is not needed in `f`.
    ///
    /// # Errors
    /// If parsing fails in this multi-document or if `f` returns an error, a [`ParseAnyMultiDocumentError`] is returned.
    pub fn find<B, F>(&mut self, mut f: F) -> Result<Option<B>, ParseAnyMultiDocumentError>
    where
        F: FnMut(&mut Any<'json, '_>) -> Result<Option<B>, ParseAnyMultiDocumentError>,
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

debug_impl!("MultiDocument", MultiDocument<'json>, no_parents);

#[cfg(test)]
mod test {
    use super::{MultiDocument, ParseMultiDocumentError};

    #[test]
    fn parse_string() {
        let expected = "Hello, World!";
        let json = format!("\"{expected}\"");

        let mut document = MultiDocument::new(&json);

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
        let expected2 = "s2";
        let json = format!("\"{expected}\"\"s2\"");

        let mut document = MultiDocument::new(&json);

        let parsed = document
            .next()
            .expect("failed to parse document")
            .expect("got no values in document")
            .string()
            .expect("expected string from document")
            .get()
            .expect("failed to parse string");

        assert_eq!(parsed, expected);

        let parsed = document
            .next()
            .expect("failed to parse document")
            .expect("got no more values in document")
            .string()
            .expect("expected string from document")
            .get()
            .expect("failed to parse string");

        assert_eq!(parsed, expected2);

        assert!(document.next().expect("failed to parse document").is_none());
    }

    #[test]
    fn empty() {
        let mut document = MultiDocument::new("");
        let value = document.next().expect("failed to parse document");

        assert!(value.is_none());
    }

    #[test]
    fn parse_invalid() {
        let invalid = 'j';
        let json = invalid.to_string();
        let mut document = MultiDocument::new(&json);

        let error = document
            .next()
            .expect_err("failed to return error after parsing invalid document");

        assert_eq!(error, ParseMultiDocumentError::InvalidElement(invalid));
    }

    #[test]
    fn invalid_after_value() {
        let expected = "Hello, World!";
        let invalid = 'j';
        let json = format!("\"{expected}\"{invalid}");

        let mut document = MultiDocument::new(&json);

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

        assert_eq!(error, ParseMultiDocumentError::InvalidElement(invalid));
    }
}
