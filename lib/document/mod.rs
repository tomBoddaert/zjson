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
}

debug_impl!("Document", Document<'json>, no_parents);
