use crate::{array::Array, literal::Literal, number::Number, object::Object, string::String};

mod error;
pub use error::ParseAnyError;

#[derive(Debug)]
/// Any JSON value.
pub enum Any<'json, 'p> {
    /// A [`String`] value.
    String(String<'json, 'p>),
    /// A [`Number`] value.
    Number(Number<'json, 'p>),
    /// An [`Object`] value.
    Object(Object<'json, 'p>),
    /// An [`Array`] value.
    Array(Array<'json, 'p>),
    /// A [`Literal`] value.
    Literal(Literal<'json, 'p>),
}

impl<'json, 'p> Any<'json, 'p> {
    #[inline]
    /// Finish parsing the value so that the parent can continue.
    ///
    /// # Errors
    /// If parsing fails in this value or a child, the error is returned as a [`ParseAnyError`].
    pub fn finish(&mut self) -> Result<(), ParseAnyError> {
        match self {
            Self::String(string) => string.finish()?,
            Self::Number(number) => number.finish()?,
            Self::Object(object) => object.finish()?,
            Self::Array(array) => array.finish()?,
            Self::Literal(literal) => literal.finish()?,
        }

        Ok(())
    }
}
