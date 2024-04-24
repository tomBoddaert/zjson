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

macro_rules! as_impl {
    (
        $variant:pat => $type:ty [$type_name:literal] $value:ident:
        $as:ident, $as_or:ident, $as_or_else:ident,
        $mut_as:ident, $mut_as_or:ident, $mut_as_or_else:ident
    ) => {
        as_impl! {
            $variant => $type ["a", $type_name] $value:
            $as, $as_or, $as_or_else,
            $mut_as, $mut_as_or, $mut_as_or_else
        }
    };

    (
        $variant:pat => $type:ty [$a:literal, $type_name:literal] $value:ident:
        $as:ident, $as_or:ident, $as_or_else:ident,
        $mut_as:ident, $mut_as_or:ident, $mut_as_or_else:ident
    ) => {
        #[inline]
        #[must_use]
        #[doc = concat!("Get the value as ", $a, " [`", $type_name, "`].")]
        pub const fn $as(self) -> Option<$type> {
            if let $variant = self {
                Some($value)
            } else {
                None
            }
        }

        #[inline]
        #[must_use]
        #[doc = concat!("Get the value as ", $a, " [`&mut ", $type_name, "`].")]
        pub fn $mut_as(&mut self) -> Option<&mut $type> {
            if let $variant = self {
                Some($value)
            } else {
                None
            }
        }

        #[inline]
        #[doc = concat!("Try to get the value as ", $a, " [`", $type_name, "`].")]
        ///
        #[doc = concat!(
            "Arguments passed to `", stringify!($as_or), "` are eagerly evaluated; ",
            "if you are passing the result of a function call, it is recommended to ",
            "use [`Self::", stringify!($as_or_else), "`], which is lazily evaluated."
        )]
        ///
        /// # Errors
        #[doc = concat!("Returns [`Err(err)`](Err) if `self` is not ", $a, " [`", $type_name, "`]")]
        pub fn $as_or<E>(self, err: E) -> Result<$type, E> {
            if let $variant = self {
                Ok($value)
            } else {
                Err(err)
            }
        }

        #[inline]
        #[doc = concat!("Try to get the value as ", $a, " [`&mut ", $type_name, "`].")]
        ///
        #[doc = concat!(
            "Arguments passed to `", stringify!($as_or), "` are eagerly evaluated; ",
            "if you are passing the result of a function call, it is recommended to ",
            "use [`Self::", stringify!($as_or_else), "`], which is lazily evaluated."
        )]
        ///
        /// # Errors
        #[doc = concat!("Returns [`Err(err)`](Err) if `self` is not ", $a, " [`", $type_name, "`]")]
        pub fn $mut_as_or<E>(&mut self, err: E) -> Result<&mut $type, E> {
            if let $variant = self {
                Ok($value)
            } else {
                Err(err)
            }
        }

        #[inline]
        #[doc = concat!("Try to get the value as ", $a, " [`", $type_name, "`].")]
        ///
        /// # Errors
        #[doc = concat!("Returns [`Err(err())`](Err) if `self` is not ", $a, " [`", $type_name, "`]")]
        pub fn $as_or_else<E, F>(self, err: F) -> Result<$type, E>
        where
            F: FnOnce() -> E,
        {
            if let $variant = self {
                Ok($value)
            } else {
                Err(err())
            }
        }

        #[inline]
        #[doc = concat!("Try to get the value as ", $a, " [`&mut ", $type_name, "`].")]
        ///
        /// # Errors
        #[doc = concat!("Returns [`Err(err())`](Err) if `self` is not ", $a, " [`", $type_name, "`]")]
        pub fn $mut_as_or_else<E, F>(&mut self, err: F) -> Result<&mut $type, E>
        where
            F: FnOnce() -> E,
        {
            if let $variant = self {
                Ok($value)
            } else {
                Err(err())
            }
        }
    };
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

    as_impl! {
        Self::String(value) => String<'json, 'p> ["String"] value:
        string, string_or, string_or_else,
        mut_string, mut_string_or, mut_string_or_else
    }

    as_impl! {
        Self::Number(value) => Number<'json, 'p> ["Number"] value:
        number, number_or, number_or_else,
        mut_number, mut_number_or, mut_number_or_else
    }

    as_impl! {
        Self::Object(value) => Object<'json, 'p> ["an", "Object"] value:
        object, object_or, object_or_else,
        mut_object, mut_object_or, mut_object_or_else
    }

    as_impl! {
        Self::Array(value) => Array<'json, 'p> ["an", "Array"] value:
        array, array_or, array_or_else,
        mut_array, mut_array_or, mut_array_or_else
    }

    as_impl! {
        Self::Literal(value) => Literal<'json, 'p> ["Literal"] value:
        literal, literal_or, literal_or_else,
        mut_literal, mut_literal_or, mut_literal_or_else
    }
}
