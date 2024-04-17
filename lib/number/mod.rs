use crate::{debug::debug_impl, status::Status, Parent};

mod error;
mod machine;
mod parsed;
pub use error::ParseNumberError;
use machine::Machine;
pub use parsed::ParsedNumber;

/// A JSON number.
pub struct Number<'json, 'p> {
    parent: &'p mut dyn Parent<'json>,
    remaining: &'json str,
}

impl<'json, 'p> Number<'json, 'p> {
    pub(crate) fn new(parent: &'p mut dyn Parent<'json>, remaining: &'json str) -> Self {
        Self { parent, remaining }
    }

    /// Try to parse the number.
    ///
    /// # Errors
    /// If parsing fails, this will return a [`ParseNumberError`].
    pub fn get(&mut self) -> Result<ParsedNumber, ParseNumberError> {
        let mut machine = Machine::Start { signed: false };
        let mut end = self.remaining.len();

        let mut chars = self.remaining.char_indices();
        loop {
            let Some((i, c)) = chars.next() else {
                machine.valid_end()?;
                break;
            };

            let Status::Parsing(next) = machine.apply(c)? else {
                end = i;
                break;
            };

            machine = next;
        }

        let remaining = &self.remaining[end..];
        self.parent.set_remaining(remaining);

        let number_string = &self.remaining[..end];
        Ok(ParsedNumber::new(number_string))
    }

    #[inline]
    /// Finish parsing the number so that the parent can continue.
    ///
    /// If [`Self::get`] has been called, this is not needed.
    ///
    /// # Errors
    /// If parsing fails in this string, the error is returned as a [`ParseNumberError`].
    pub fn finish(&mut self) -> Result<(), ParseNumberError> {
        self.get().map(drop)
    }
}

debug_impl!("Number", Number<'json, 'p>);
