//! A simple interface for extending external types with custom implementations of [Debug] and
//! [Display] traits.
//!
//! All examples are in [this directory].
//!
//! [Debug]: core::fmt::Debug
//! [Display]: core::fmt::Display
//! [this directory]: https://github.com/ilyavenner/fmt-ext/tree/master/examples

#![no_std]
#![deny(missing_docs)]

pub use crate::carrier::Carrier;
use crate::{debug::AttachDebug, display::AttachDisplay};

/// Contains [Carrier] implementation.
mod carrier;

pub mod debug;
pub mod display;

/// A trait extension which adds a `.debug()` method like [Path::display] does this for any type.
/// Its method returns an object that implements [Debug] and can be used for formatting.
///
/// [Path::display]: https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.display
///
/// [Debug]: core::fmt::Debug
pub trait DebugExt<D> {
    /// Returns a wrapper that implements [Debug] via [CustomDebug].
    ///
    /// [Debug]: core::fmt::Debug
    /// [CustomDebug]: crate::debug::CustomDebug
    fn debug(&self) -> Carrier<'_, Self, D> {
        Carrier::new(self)
    }
}

/// A trait extension which adds a method like [Path::display] does this for any type. Its method
/// returns an object that implements [Display] and can be used for formatting.
///
/// [Path::display]: https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.display
/// [Display]: core::fmt::Display
pub trait DisplayExt<D> {
    /// Returns a wrapper that implements [Display] via [CustomDisplay].
    ///
    /// [Display]: core::fmt::Display
    /// [CustomDisplay]: crate::display::CustomDisplay
    fn display(&self) -> Carrier<'_, Self, D> {
        Carrier::new(self)
    }
}

impl<T, D> DebugExt<D> for T where T: AttachDebug<D> + ?Sized {}

impl<T, D> DisplayExt<D> for T where T: AttachDisplay<D> + ?Sized {}
