use core::fmt;

use crate::debug::DisplayAsDebug;

#[derive(Clone, Copy)]
/// A parsed JSON number.
pub struct ParsedNumber<'json> {
    json: &'json str,
}

macro_rules! as_impl {
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

    as_impl!(as_u8, u8);
    as_impl!(as_u16, u16);
    as_impl!(as_u32, u32);
    as_impl!(as_u64, u64);
    as_impl!(as_u128, u128);

    as_impl!(as_i8, i8);
    as_impl!(as_i16, i16);
    as_impl!(as_i32, i32);
    as_impl!(as_i64, i64);
    as_impl!(as_i128, i128);

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

macro_rules! eq_ord_impl {
    ( $t:ty, $as:expr ) => {
        impl PartialEq<$t> for ParsedNumber<'_> {
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                $as(*self).map_or(false, |n| n.eq(other))
            }
        }

        impl PartialOrd<$t> for ParsedNumber<'_> {
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<core::cmp::Ordering> {
                $as(*self).map(|n| n.cmp(other))
            }
        }
    };
}

eq_ord_impl!(u8, Self::as_u8);
eq_ord_impl!(u16, Self::as_u16);
eq_ord_impl!(u32, Self::as_u32);
eq_ord_impl!(u64, Self::as_u64);
eq_ord_impl!(u128, Self::as_u128);

eq_ord_impl!(i8, Self::as_i8);
eq_ord_impl!(i16, Self::as_i16);
eq_ord_impl!(i32, Self::as_i32);
eq_ord_impl!(i64, Self::as_i64);
eq_ord_impl!(i128, Self::as_i128);

impl PartialEq<f32> for ParsedNumber<'_> {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        self.as_f32().eq(other)
    }
}

impl PartialEq<f64> for ParsedNumber<'_> {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.as_f64().eq(other)
    }
}

#[cfg(test)]
mod test {
    use super::ParsedNumber;

    macro_rules! test_eq {
        ( $n:literal ) => {{
            let n = $n;
            let ns = n.to_string();

            let parsed = ParsedNumber::new(&ns);
            assert_eq!(parsed, n);
        }};
    }

    #[test]
    fn partial_eq_impl_zero() {
        test_eq!(0_u8);
        test_eq!(0_u16);
        test_eq!(0_u32);
        test_eq!(0_u64);
        test_eq!(0_u128);

        test_eq!(0_i8);
        test_eq!(0_i16);
        test_eq!(0_i32);
        test_eq!(0_i64);
        test_eq!(0_i128);

        test_eq!(0_f32);
        test_eq!(0_f64);
    }

    #[test]
    fn partial_eq_impl_positive() {
        test_eq!(53_u8);
        test_eq!(53_u16);
        test_eq!(53_u32);
        test_eq!(53_u64);
        test_eq!(53_u128);

        test_eq!(53_i8);
        test_eq!(53_i16);
        test_eq!(53_i32);
        test_eq!(53_i64);
        test_eq!(53_i128);

        test_eq!(53_f32);
        test_eq!(53_f64);
    }

    #[test]
    fn partial_eq_impl_negative() {
        test_eq!(-53_i8);
        test_eq!(-53_i16);
        test_eq!(-53_i32);
        test_eq!(-53_i64);
        test_eq!(-53_i128);

        test_eq!(-53_f32);
        test_eq!(-53_f64);
    }

    #[test]
    fn partial_eq_impl_decimal() {
        test_eq!(53.19_f32);
        test_eq!(53.19_f64);

        test_eq!(-53.19_f32);
        test_eq!(-53.19_f64);
    }

    #[test]
    fn partial_eq_impl_exponent() {
        test_eq!(53.19e5_f32);
        test_eq!(53.19e5_f64);

        test_eq!(-53.19e5_f32);
        test_eq!(-53.19e5_f64);

        test_eq!(53.19e-5_f32);
        test_eq!(53.19e-5_f64);

        test_eq!(-53.19e-5_f32);
        test_eq!(-53.19e-5_f64);
    }
}
