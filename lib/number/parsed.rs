use core::fmt;

use crate::debug::DisplayAsDebug;

#[derive(Clone, Copy)]
/// A parsed JSON number.
pub struct ParsedNumber<'json> {
    json: &'json str,
}

macro_rules! parsed_number_fn {
    ( $name:ident, $t:ty ) => {
        #[must_use]
        #[inline]
        #[doc = concat!("Try to represent the number as a [`prim@", stringify!($t), "`].")]
        pub fn $name(self) -> Option<$t> {
            self.json.parse().ok()
        }
    };
}

impl<'json> ParsedNumber<'json> {
    #[must_use]
    #[inline]
    pub(super) const fn new(json: &'json str) -> Self {
        Self { json }
    }

    #[must_use]
    #[inline]
    /// Get the number as a string.
    pub const fn as_str(self) -> &'json str {
        self.json
    }

    parsed_number_fn!(as_u8, u8);
    parsed_number_fn!(as_u16, u16);
    parsed_number_fn!(as_u32, u32);
    parsed_number_fn!(as_u64, u64);
    parsed_number_fn!(as_u128, u128);

    parsed_number_fn!(as_i8, i8);
    parsed_number_fn!(as_i16, i16);
    parsed_number_fn!(as_i32, i32);
    parsed_number_fn!(as_i64, i64);
    parsed_number_fn!(as_i128, i128);

    #[must_use]
    /// Get the number as a [`prim@f32`].
    pub fn as_f32(self) -> f32 {
        self.json
            .parse()
            .expect("failed to parse a number as an f32")
    }

    #[must_use]
    /// Get the number as a [`prim@f64`].
    pub fn as_f64(self) -> f64 {
        self.json
            .parse()
            .expect("failed to parse a number as an f32")
    }
}

impl<'json> fmt::Debug for ParsedNumber<'json> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ParsedNumber")
            .field(&DisplayAsDebug(self))
            .finish()
    }
}

impl<'json> fmt::Display for ParsedNumber<'json> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.sign_plus() && !matches!(self.json.chars().next(), Some('-')) {
            f.write_str("+")?;
        }

        f.write_str(self.json)
    }
}
