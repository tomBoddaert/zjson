use crate::{
    array::Array, literal::Literal, number::Number, object::Object, string::String, Parent,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestParent<'json> {
    pub remaining: &'json str,
}

impl<'json> Parent<'json> for TestParent<'json> {
    #[inline]
    fn set_remaining<'a>(&'a mut self, remaining: &'json str)
    where
        'json: 'a,
    {
        self.remaining = remaining;
    }

    fn debug_parents(&self, list: &mut core::fmt::DebugList<'_, '_>) {
        list.entry(&"TestParent");
    }
}

impl<'json> TestParent<'json> {
    #[inline]
    #[must_use]
    pub const fn new(json: &'json str) -> Self {
        Self { remaining: json }
    }

    #[inline]
    #[must_use]
    pub fn string(&mut self) -> String<'json, '_> {
        let remaining = self.remaining;
        String::new(self, remaining)
    }

    #[inline]
    #[must_use]
    pub fn number(&mut self) -> Number<'json, '_> {
        let remaining = self.remaining;
        Number::new(self, remaining)
    }

    #[inline]
    #[must_use]
    pub fn object(&mut self) -> Object<'json, '_> {
        let remaining = self.remaining;
        Object::new(self, remaining)
    }

    #[inline]
    #[must_use]
    pub fn array(&mut self) -> Array<'json, '_> {
        let remaining = self.remaining;
        Array::new(self, remaining)
    }

    #[inline]
    #[must_use]
    pub fn literal(&mut self) -> Literal<'json, '_> {
        let remaining = self.remaining;
        Literal::new(self, remaining)
    }
}
