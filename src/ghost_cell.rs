//! `GhostCell` and `GhostToken`, as per <https://plv.mpi-sws.org/rustbelt/ghostcell/>.
//!
//! Reference implementation at <https://gitlab.mpi-sws.org/FP/ghostcell/-/tree/master/ghostcell>.

use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem,
};

/// A `GhostToken<'x>` is _the_ key to access the content of any `&GhostCell<'x, _>` sharing the same brand.
///
/// Each `GhostToken<'x>` is created alongside a unique brand (its lifetime), and each `GhostCell<'x, T>` is associated
/// to one, and only one, `GhostToken` at a time via this brand. The entire set of `GhostCell<'x, T>` associated to a
/// given `GhostToken<'x>` creates a pool of cells all being accessible solely through the one token they are associated
/// to.
///
/// The pool of `GhostCell` associated to a token need not be homogeneous, each may own a value of a different type.
pub struct GhostToken<'brand> { _marker: InvariantLifetime<'brand> }

impl<'brand> GhostToken<'brand> {
    /// Creates a fresh to