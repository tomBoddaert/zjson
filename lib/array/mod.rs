use crate::{
    any::{Any, ParseAnyError},
    containers::ParseStatus,
    debug::debug_impl,
    Parent,
};

mod error;
mod machine;
pub use error::ParseArrayError;
use machine::Machine;

/// A JSON array.
pub struct Array<'json, 'p> {
    parent: &'p mut dyn Parent<'json>,
    remaining: &'json str,
    machine: Machine,
}

impl<'json, 'p> Parent<'json> for Array<'json, 'p> {
    fn set_remaining<'a>(&'a mut self, remaining: &'json str)
    where
        'json: 'a,
    {
        self.remaining = remaining;
        if let Machine::Element(prompt) = &mut self.machine {
            *prompt = ParseStatus::Done;
        }
    }

    fn debug_parents(&self, list: &mut core::fmt::DebugList<'_, '_>) {
        self.parent.debug_parents(list.entry(&"Array"));
    }
}

impl<'json, 'p> Array<'json, 'p> {
    pub(crate) fn new(parent: &'p mut dyn Parent<'json>, remaining: &'json str) -> Self {
        Self {
            parent,
            remaining,
            machine: Machine::In { postcomma: false },
        }
    }

    #[allow(clippy::should_implement_trait)]
    /// Try to get the next value from the array.
    ///
    /// Once the array is exhausted, this will return [`None`].
    ///
    /// # Errors
    /// If parsing the array fails, this will return a [`ParseArrayError`].
    pub fn next(&mut self) -> Result<Option<Any<'json, '_>>, ParseArrayError> {
        loop {
            match self.machine {
                Machine::In { .. } | Machine::Element(ParseStatus::Done) => {}

                Machine::Element(ParseStatus::Prompted(prompt)) => {
                    let remaining = self.remaining;
                    return Ok(Some(prompt.create(self, remaining)));
                }

                Machine::End => {
                    self.parent.set_remaining(self.remaining);
                    return Ok(None);
                }
            }

            let (i, c) = self
                .remaining
                .char_indices()
                .next()
                .ok_or(ParseArrayError::UnexpectedEnd)?;
            self.machine = self.machine.apply(c)?;

            // If currently parsing a number or literal, don't remove `c` from `self.remaining`
            if let Machine::Element(ParseStatus::Prompted(prompt)) = self.machine {
                if prompt.keep_first() {
                    continue;
                }
            }

            let next_i = i + c.len_utf8();
            self.remaining = &self.remaining[next_i..];
        }
    }

    /// Finish parsing the array so that the parent can continue.
    ///
    /// # Errors
    /// If parsing fails in this array or a child, the error is returned as a [`ParseAnyError`].
    pub fn finish(&mut self) -> Result<(), ParseAnyError> {
        while let Some(mut value) = self.next()? {
            value.finish()?;
        }

        Ok(())
    }
}

debug_impl!("Array", Array<'json, 'p>);
