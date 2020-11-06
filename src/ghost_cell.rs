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
/// Each `GhostToken<'x>` is created