use crate::{debug::debug_impl, Parent};

mod error;
mod machine;
mod parsed;
pub use error::ParseStringError;
use machine::Machine;
pub use parsed::ParsedString;

/// A JSON string.
pub struct String<'json, 'p> {
    parent: &'p mut dyn Parent<'json>,
    remaining: &'json str,
}

impl<'json, 'p> String<'json, 'p> {
    pub(crate) fn new(parent: &'p mut dyn Parent<'json>, remaining: &'json str) -> Self {
        Self { parent, remaining }
    }

    /// Try to parse the string.
    /// Note that escape sequences will not be evaluated!
    ///
    /// # Errors
    /// If parsing the string fails, this will return a [`ParseStringError`].
    pub fn get(&mut self) -> Result<ParsedString<'json>, ParseStringError> {
        let mut machine = Machine::In;

        for (i, c) in self.remaining.char_indices() {
            if let Some(next) = machine.apply(c)? {
                machine = next;
                continue;
            }

            let next_i = i + c.len_utf8();
            self.parent.set_remaining(&self.remaining[next_i..]);

            return Ok(ParsedString::new(&self.remaining[0..i]));
        }

        Err(ParseStringError::UnexpectedEnd)
    }

    #[inline]
    /// Finish parsing the string so that the parent can continue.
    ///
    /// If [`Self::get`] has been called, this is not needed.
    ///
    /// # Errors
    /// If parsing fails in this string, the error is returned as a [`ParseStringError`].
    pub fn finish(&mut self) -> Result<(), ParseStringError> {
        self.get().map(drop)
    }
}

debug_impl!("String", String<'json, 'p>);
