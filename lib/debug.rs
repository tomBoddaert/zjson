use core::fmt;

use crate::Parent;

pub struct ParentDebugger<'json, 'p>(&'p dyn Parent<'json>);

impl<'json, 'p> ParentDebugger<'json, 'p> {
    pub fn new(parent: &'p dyn Parent<'json>) -> Self {
        Self(parent)
    }
}

impl<'json, 'p> fmt::Debug for ParentDebugger<'json, 'p> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        self.0.debug_parents(&mut list);
        list.finish()
    }
}

macro_rules! debug_impl {
    ( $name:literal, $t:ty ) => {
        impl<'json, 'p> core::fmt::Debug for $t {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct($name)
                    .field("parents", &$crate::debug::ParentDebugger::new(self.parent))
                    .field("remaining_json", &self.remaining)
                    .finish()
            }
        }
    };

    ( $name:literal, $t:ty, no_parents) => {
        impl<'json> core::fmt::Debug for $t {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct($name)
                    .field("remaining_json", &self.remaining)
                    .finish()
            }
        }
    };
}
pub(crate) use debug_impl;

#[allow(clippy::module_name_repetitions)]
pub struct DisplayAsDebug<'a, T: fmt::Display>(pub &'a T);

impl<'a, T: fmt::Display> fmt::Debug for DisplayAsDebug<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
