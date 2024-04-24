use crate::{
    any::Any, array::Array, literal::Literal, number::Number, object::Object, string::String,
    Parent,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParsePrompt {
    String,
    Number,
    Object,
    Array,
    Literal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseStatus {
    Prompted(ParsePrompt),
    Done,
}

impl ParsePrompt {
    #[inline]
    pub const fn get(c: char) -> Option<Self> {
        match c {
            '"' => Some(Self::String),
            '0'..='9' | '-' => Some(Self::Number),
            '{' => Some(Self::Object),
            '[' => Some(Self::Array),
            't' | 'f' | 'n' => Some(Self::Literal),

            _ => None,
        }
    }

    #[inline]
    pub const fn keep_first(self) -> bool {
        matches!(self, Self::Number | Self::Literal)
    }

    pub fn create<'json, 'p>(
        self,
        parent: &'p mut dyn Parent<'json>,
        remaining: &'json str,
    ) -> Any<'json, 'p>
    where
        'json: 'p,
    {
        match self {
            Self::String => Any::String(String::new(parent, remaining)),
            Self::Number => Any::Number(Number::new(parent, remaining)),
            Self::Object => Any::Object(Object::new(parent, remaining)),
            Self::Array => Any::Array(Array::new(parent, remaining)),
            Self::Literal => Any::Literal(Literal::new(parent, remaining)),
        }
    }
}

impl From<ParsePrompt> for ParseStatus {
    #[inline]
    fn from(value: ParsePrompt) -> Self {
        Self::Prompted(value)
    }
}

macro_rules! fff_impl {
    (
        type: $type_name:literal
        $f:ident ( $( $param:ty ),* ) -> Result<_, $error:ty>;
        $acc:ident, $pat:pat =>
            $call:expr,
            $call_fold:expr,
            $value:expr;
        $any_err:expr
    ) => {
        $crate::containers::fff_impl! {
            type: $type_name
            value: "value"
            $f( $($param),* ) -> Result<_, $error>;
            $acc, $pat =>
                $call,
                $call_fold,
                $value;
            $any_err
        }
    };

    (
        type: $type_name:literal
        value: $value_name:literal
        $f:ident ( $( $param:ty ),* ) -> Result<_, $error:ty>;
        $acc:ident, $pat:pat =>
            $call:expr,
            $call_fold:expr,
            $value:expr;
        $any_err:expr
    ) => {
        #[doc = concat!("Runs `f` for each ", $value_name, " in the ", $type_name, ".")]
        ///
        /// [`Any::finish`] is automatically called on all values, so it is not needed in `f`.
        ///
        /// # Errors
        #[doc = concat!("If parsing fails in this ", $type_name, " or if `f` returns an error, an instance of `E` is returned.")]
        #[doc = concat!("If you do not need a custom error type, use [`", stringify!($error), "`] as `E`.")]
        pub fn for_each<F, E>(&mut self, mut $f: F) -> Result<(), E>
        where
            F: FnMut($($param),*) -> Result<(), $error>,
            E: From<$error>,
        {
            while let Some( $pat ) = self.next().map_err($any_err)? {
                // f(key, &mut value)?;
                $call?;
                $value.finish().map_err(From::from)?;
            }

            Ok(())
        }

        #[doc = concat!("Applies `f` to the accumulator, passing in each ", $value_name, " in the ", $type_name, ".")]
        ///
        /// The initial value of the accumulator is `init`.
        ///
        /// [`Any::finish`] is automatically called on all values, so it is not needed in `f`.
        ///
        /// # Errors
        #[doc = concat!("If parsing fails in this ", $type_name, " or if `f` returns an error, an instance of `E` is returned.")]
        #[doc = concat!("If you do not need a custom error type, use [`", stringify!($error), "`] as `E`.")]
        pub fn fold<B, F, E>(&mut self, init: B, mut $f: F) -> Result<B, E>
        where
            F: FnMut(B, $($param),*) -> Result<B, $error>,
            E: From<$error>,
        {
            let mut $acc = init;

            while let Some( $pat ) = self.next().map_err($any_err)? {
                $acc = $call_fold?;
                $value.finish().map_err(From::from)?;
            }

            Ok($acc)
        }

        #[doc = concat!("Runs `f` for each ", $value_name, " in the ", $type_name, ", stopping if `f` returns [`Some`].")]
        ///
        /// [`Any::finish`] is automatically called on each value iterated over, so it is not needed in `f`.
        ///
        /// # Errors
        #[doc = concat!("If parsing fails in this ", $type_name, " or if `f` returns an error, an instance of `E` is returned.")]
        #[doc = concat!("If you do not need a custom error type, use [`", stringify!($error), "`] as `E`.")]
        pub fn find<B, F, E>(&mut self, mut $f: F) -> Result<Option<B>, E>
        where
            F: FnMut( $($param),* ) -> Result<Option<B>, E>,
            E: From<$error>,
        {
            while let Some( $pat ) = self.next().map_err($any_err)? {
                let result = $call?;
                $value.finish().map_err(From::from)?;

                if result.is_some() {
                    return Ok(result);
                }
            }

            Ok(None)
        }
    };
}
pub(crate) use fff_impl;
