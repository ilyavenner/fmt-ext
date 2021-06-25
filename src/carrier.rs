use core::{fmt, marker::PhantomData};

use crate::{debug::*, display::*};

/// A type that make possible formatting for [CustomDebug] and [CustomDisplay] implementors.
///
/// [Debug]: fmt::Debug
/// [Display]: fmt::Display
pub struct Carrier<'t, T, D>
where
    T: ?Sized,
{
    t: &'t T,
    _pd: PhantomData<D>,
}

impl<'t, T, D> Carrier<'t, T, D>
where
    T: ?Sized,
{
    /// Creates new instance over `T`.
    pub const fn new(t: &'t T) -> Self {
        Self {
            t,
            _pd: PhantomData,
        }
    }
}

impl<T, D> fmt::Debug for Carrier<'_, T, D>
where
    T: AttachDebug<D> + ?Sized,
    D: CustomDebug<Target = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        D::fmt_target(&self.t, f)
    }
}

impl<T, D> fmt::Display for Carrier<'_, T, D>
where
    T: AttachDisplay<D> + ?Sized,
    D: CustomDisplay<Target = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        D::fmt_target(&self.t, f)
    }
}
