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

    /// Runs `f` for each element in the array.
    ///
    /// [`Any::finish`] is automatically called on all values, so it is not needed in `f`.
    ///
    /// # Errors
    /// If parsing fails in this array or if `f` returns an error, a [`ParseAnyError`] is returned.
    pub fn for_each<F>(&mut self, mut f: F) -> Result<(), ParseAnyError>
    where
        F: FnMut(&mut Any<'json, '_>) -> Result<(), ParseAnyError>,
    {
        while let Some(mut value) = self.next()? {
            f(&mut value)?;
            value.finish()?;
        }

        Ok(())
    }

    /// Applies `f` to the accumulator, passing in each element in the array.
    ///
    /// The initial value of the accumulator is `init`.
    ///
    /// [`Any::finish`] is automatically called on all values, so it is not needed in `f`.
    ///
    /// # Errors
    /// If parsing fails in this array or if `f` returns an error, a [`ParseAnyError`] is returned.
    pub fn fold<B, F>(&mut self, init: B, mut f: F) -> Result<B, ParseAnyError>
    where
        F: FnMut(B, &mut Any<'json, '_>) -> Result<B, ParseAnyError>,
    {
        let mut accumulator = init;

        while let Some(mut value) = self.next()? {
            accumulator = f(accumulator, &mut value)?;
            value.finish()?;
        }

        Ok(accumulator)
    }

    /// Runs `f` for each value in the array, stopping if `f` returns [`Some`].
    ///
    /// [`Any::finish`] is automatically called on each value iterated over, so it is not needed in `f`.
    ///
    /// # Errors
    /// If parsing fails in this array or if `f` returns an error, a [`ParseAnyError`] is returned.
    pub fn find<B, F>(&mut self, mut f: F) -> Result<Option<B>, ParseAnyError>
    where
        F: FnMut(&mut Any<'json, '_>) -> Result<Option<B>, ParseAnyError>,
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

debug_impl!("Array", Array<'json, 'p>);

#[cfg(test)]
mod test {
    use crate::test_parent::TestParent;

    use super::ParseArrayError;

    #[test]
    fn empty() {
        let mut parent = TestParent::new("]");
        let mut array = parent.array();

        let value = array.next().expect("failed to parse array");
        assert!(value.is_none());

        assert!(parent.remaining.is_empty());
    }

    #[test]
    fn string() {
        let expected_value = "value1";
        let json = format!("\"{expected_value}\"]");

        let mut parent = TestParent::new(&json);
        let mut array = parent.array();

        let value = array
            .next()
            .expect("failed to parse array")
            .expect("failed to get value from array")
            .string()
            .expect("failed to get string from array")
            .get()
            .expect("failed to parse string");

        assert_eq!(value, expected_value);

        let next = array.next().expect("failed to parse array");
        assert!(next.is_none());

        assert!(parent.remaining.is_empty());
    }

    #[test]
    fn invalid() {
        let invalid = 'j';
        let json = invalid.to_string();

        let mut parent = TestParent::new(&json);
        let mut array = parent.array();

        let error = array
            .next()
            .expect_err("failed to return error from invalid array");

        assert_eq!(
            error,
            ParseArrayError::InvalidElement {
                c: invalid,
                or_end: true
            }
        );

        assert_eq!(parent.remaining, json);
    }

    #[test]
    fn invalid_after_valid() {
        let expected = "value1";
        let invalid = 'j';

        let json = format!("\"{expected}\", {invalid}");

        let mut parent = TestParent::new(&json);
        let mut array = parent.array();

        let value = array
            .next()
            .expect("failed to parse array")
            .expect("failed to get value from array")
            .string()
            .expect("failed to get string from array")
            .get()
            .expect("failed to parse string");

        assert_eq!(value, expected);

        let error = array
            .next()
            .expect_err("failed to return error from invalid array");

        assert_eq!(
            error,
            ParseArrayError::InvalidElement {
                c: invalid,
                or_end: false
            }
        );

        assert_eq!(parent.remaining, json);
    }
}
