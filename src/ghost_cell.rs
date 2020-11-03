//! `GhostCell` and `GhostToken`, as per <https://plv.mpi-sws.org/rustbelt/ghostcell/>.
//!
//! Reference implementation at <https://gitlab.mpi-sws.org/FP/ghostcell/-/tree/master/ghostcell>.

use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem,
};

/// A `Ghos