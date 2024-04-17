use crate::{debug::debug_impl, Parent};

mod error;
mod machine;
mod parsed;
pub use error::ParseLiteralError;
use machine::Machine;
pub use parsed::ParsedLiteral;

/// A JSON literal, either `null`, `true` or `false`.
pub struct Literal<'json, 'p> {
    parent: &'p mut dyn Parent<'json>,
    remaining: &'json str,
}

impl<'json, 'p> Literal<'json, 'p> {
    pub(crate) fn new(parent: &'p mut dyn Parent<'json>, remaining: &'json str) -> Self {
        Self { parent, remaining }
    }

    /// Try to parse the literal.
    ///
    /// # Errors
    /// If parsing the literal fails, this will return a [`ParseLiteralError`].
    pub fn get(&mut self) -> Result<ParsedLiteral, ParseLiteralError> {
        let mut machine = Machine::Start;

        for (i, c) in self.remaining.char_indices() {
            machine = machine.apply(c)?;

            if let Machine::End(value) = machine {
                let next_i = i + c.len_utf8();
                let remaining = &self.remaining[next_i..];
                self.parent.set_remaining(remaining);
                return Ok(value);
            }
        }

        Err(ParseLiteralError::UnexpectedEnd)
    }

    #[inline]
    /// Finish parsing the literal so that the parent can continue.
    ///
    /// If [`Self::get`] has been called, this is not needed.
    ///
    /// # Errors
    /// If parsing fails in this literal, the error is returned as a [`ParseLiteralError`].
    pub fn finish(&mut self) -> Result<(), ParseLiteralError> {
        self.get().map(drop)
    }
}

debug_impl!("Literal", Literal<'json, 'p>);
