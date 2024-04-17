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
}

debug_impl!("MultiDocument", MultiDocument<'json>, no_parents);
