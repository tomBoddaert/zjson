use crate::{
    any::{Any, ParseAnyError},
    containers::ParseStatus,
    debug::debug_impl,
    string::{self, String},
    Parent,
};

mod error;
mod machine;
pub use error::ParseObjectError;
use machine::Machine;

/// A JSON object.
pub struct Object<'json, 'p> {
    parent: &'p mut dyn Parent<'json>,
    remaining: &'json str,
    machine: Machine<'json>,
}

impl<'json, 'p> Parent<'json> for Object<'json, 'p> {
    fn set_remaining<'a>(&'a mut self, remaining: &'json str)
    where
        'json: 'a,
    {
        self.remaining = remaining;
        if let Machine::Element { name: _, element } = &mut self.machine {
            *element = ParseStatus::Done;
        }
    }

    fn debug_parents(&self, list: &mut core::fmt::DebugList<'_, '_>) {
        self.parent.debug_parents(list.entry(&"Object"));
    }
}

impl<'json, 'p> Object<'json, 'p> {
    pub(crate) fn new(parent: &'p mut dyn Parent<'json>, remaining: &'json str) -> Self {
        Self {
            parent,
            remaining,
            machine: Machine::In { postcomma: false },
        }
    }

    #[allow(clippy::should_implement_trait)]
    /// Try to get the next key, value pair from the object.
    ///
    /// Once the object is exhausted, this will return [`None`].
    ///
    /// # Errors
    /// - If parsing the object fails, this will return a [`ParseObjectError`].
    /// - If parsing a key fails, the error will be the [`ParseObjectError::InvalidName`] variant.
    pub fn next(
        &mut self,
    ) -> Result<Option<(string::ParsedString<'json>, Any<'json, '_>)>, ParseObjectError> {
        loop {
            let remaining = self.remaining;

            match self.machine {
                Machine::In { .. }
                | Machine::Name(Some(_))
                | Machine::PreElement { .. }
                | Machine::Element {
                    element: ParseStatus::Done,
                    ..
                } => {}

                Machine::Name(None) => {
                    let mut name = String::<'json, '_>::new(self, remaining);
                    let name = name.get().map_err(ParseObjectError::InvalidName)?;
                    self.machine = Machine::Name(Some(name));
                }

                Machine::Element {
                    name,
                    element: ParseStatus::Prompted(prompt),
                } => {
                    return Ok(Some((name, prompt.create(self, remaining))));
                }

                Machine::End => {
                    self.parent.set_remaining(remaining);
                    return Ok(None);
                }
            }

            let c = self
                .remaining
                .chars()
                .next()
                .ok_or(ParseObjectError::UnexpectedEnd)?;
            self.machine = self.machine.apply(c)?;

            // If currently parsing a number or literal, don't remove `c` from `self.remaining`
            if let Machine::Element {
                element: ParseStatus::Prompted(prompt),
                ..
            } = self.machine
            {
                if prompt.keep_first() {
                    continue;
                }
            }

            self.remaining = &self.remaining[c.len_utf8()..];
        }
    }

    /// Finish parsing the object so that the parent can continue.
    ///
    /// # Errors
    /// If parsing fails in this object or a child, the error is returned as a [`ParseAnyError`].
    pub fn finish(&mut self) -> Result<(), ParseAnyError> {
        while let Some((_, mut value)) = self.next()? {
            value.finish()?;
        }

        Ok(())
    }
}

debug_impl!("Object", Object<'json, 'p>);
