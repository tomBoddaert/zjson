#![doc = include_str!("../README.md")]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::perf,
    clippy::cargo,
    clippy::alloc_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::get_unwrap,
    clippy::panic_in_result_fn,
    clippy::todo,
    clippy::undocumented_unsafe_blocks,
    clippy::error_impl_error,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]
#![cfg_attr(not(feature = "std"), no_std)]
// TODO: remove
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_panics_doc,
    clippy::cargo_common_metadata,
    clippy::panic_in_result_fn
)]

use core::fmt;

/// Types that abstract over all JSON types.
pub mod any;
/// Types related to JSON arrays.
pub mod array;
mod debug;
/// Types related to JSON documents.
pub mod document;
/// Types related to JSON `true`, `false` and `null` values.
pub mod literal;
/// Types related to JSON documents with multiple values.
pub mod multi_document;
/// Types related to JSON numbers.
pub mod number;
/// Types related to JSON objects.
pub mod object;
/// Types related to JSON strings.
pub mod string;

mod containers;
mod status;
#[cfg(test)]
mod test_parent;

trait Parent<'json> {
    fn set_remaining<'a>(&'a mut self, remaining: &'json str)
    where
        'json: 'a;

    fn debug_parents(&self, list: &mut fmt::DebugList<'_, '_>);
}
