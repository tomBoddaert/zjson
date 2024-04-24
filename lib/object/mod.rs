use crate::{
    any::{Any, ParseAnyError},
    containers::ParseStatus,
    debug::debug_impl,
    string::{self, ParsedString, String},
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

    /// Runs `f` for each key, value pair in the object.
    ///
    /// [`Any::finish`] is automatically called on all values, so it is not needed in `f`.
    ///
    /// # Errors
    /// If parsing fails in this object or if `f` returns an error, a [`ParseAnyError`] is returned.
    pub fn for_each<F>(&mut self, mut f: F) -> Result<(), ParseAnyError>
    where
        F: FnMut(ParsedString<'json>, &mut Any<'json, '_>) -> Result<(), ParseAnyError>,
    {
        while let Some((key, mut value)) = self.next()? {
            f(key, &mut value)?;
            value.finish()?;
        }

        Ok(())
    }

    /// Applies `f` to the accumulator, passing in each key, value pair in the object.
    ///
    /// The initial value of the accumulator is `init`.
    ///
    /// [`Any::finish`] is automatically called on all values, so it is not needed in `f`.
    ///
    /// # Errors
    /// If parsing fails in this multi-document or if `f` returns an error, a [`ParseAnyError`] is returned.
    pub fn fold<B, F>(&mut self, init: B, mut f: F) -> Result<B, ParseAnyError>
    where
        F: FnMut(B, ParsedString<'json>, &mut Any<'json, '_>) -> Result<B, ParseAnyError>,
    {
        let mut accumulator = init;

        while let Some((key, mut value)) = self.next()? {
            accumulator = f(accumulator, key, &mut value)?;
            value.finish()?;
        }

        Ok(accumulator)
    }

    /// Runs `f` for each key, value pair in the object, stopping if `f` returns [`Some`].
    ///
    /// [`Any::finish`] is automatically called on each value iterated over, so it is not needed in `f`.
    ///
    /// # Errors
    /// If parsing fails in this object or if `f` returns an error, a [`ParseAnyError`] is returned.
    pub fn find<B, F>(&mut self, mut f: F) -> Result<Option<B>, ParseAnyError>
    where
        F: FnMut(ParsedString<'json>, &mut Any<'json, '_>) -> Result<Option<B>, ParseAnyError>,
    {
        while let Some((key, mut value)) = self.next()? {
            let result = f(key, &mut value)?;
            value.finish()?;

            if result.is_some() {
                return Ok(result);
            }
        }

        Ok(None)
    }
}

debug_impl!("Object", Object<'json, 'p>);

#[cfg(test)]
mod test {
    use crate::test_parent::TestParent;

    use super::ParseObjectError;

    #[test]
    fn empty() {
        let mut parent = TestParent::new("}");
        let mut object = parent.object();

        let value = object.next().expect("failed to parse object");
        assert!(value.is_none());

        assert!(parent.remaining.is_empty());
    }

    #[test]
    fn string() {
        let expected_key = "key1";
        let expected_value = "value1";
        let json = format!("\"{expected_key}\": \"{expected_value}\"}}");

        let mut parent = TestParent::new(&json);
        let mut object = parent.object();

        let (key, value) = object
            .next()
            .expect("failed to parse object")
            .expect("failed to get value from object");

        let value = value
            .string()
            .expect("failed to get string from object")
            .get()
            .expect("failed to parse string");

        assert_eq!(key, expected_key);
        assert_eq!(value, expected_value);

        let next = object.next().expect("failed to parse object");
        assert!(next.is_none());

        assert!(parent.remaining.is_empty());
    }

    #[test]
    fn invalid() {
        let invalid = 'j';
        let json = invalid.to_string();

        let mut parent = TestParent::new(&json);
        let mut object = parent.object();

        let error = object
            .next()
            .expect_err("failed to return error from invalid object");

        assert_eq!(
            error,
            ParseObjectError::ExpectedName {
                c: invalid,
                or_end: true
            }
        );

        assert_eq!(parent.remaining, json);
    }

    #[test]
    fn invalid_after_valid() {
        let expected_key = "key1";
        let expected_value = "value1";
        let invalid = 'j';

        let json = format!("\"{expected_key}\": \"{expected_value}\", {invalid}");

        let mut parent = TestParent::new(&json);
        let mut object = parent.object();

        let (key, value) = object
            .next()
            .expect("failed to parse object")
            .expect("failed to get value from object");

        let value = value
            .string()
            .expect("failed to get string from object")
            .get()
            .expect("failed to parse string");

        assert_eq!(key, expected_key);
        assert_eq!(value, expected_value);

        let error = object
            .next()
            .expect_err("failed to return error from invalid object");

        assert_eq!(
            error,
            ParseObjectError::ExpectedName {
                c: invalid,
                or_end: false
            }
        );

        assert_eq!(parent.remaining, json);
    }
}
