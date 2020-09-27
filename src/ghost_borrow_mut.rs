
//! The `GhostBorrowMut` trait, which allows mutably borrowing multiple `GhostCell`s simultaneously.
//!
//! This module implement the `GhostBorrowMut` trait for:
//!
//! -   A slice of `GhostCell`s.
//! -   An array of `GhostCell`s of any size.
//! -   A tuple of `GhostCell`s of up to 12 elements.
//! -   A tuple of references to `GhostCell`s of up to 12 elements.
//!
//! #   Performance
//!
//! In general borrowing is free of cost, however a special-case is necessary for the tuple of references, as then the
//! references may alias.
//!
//! #   Experimental
//!
//! The feature is experimental, to enable, use the feature "experimental-multiple-mutable-borrows".

use core::{convert::Infallible, mem, ptr};

use crate::ghost_cell::*;

/// An error signifying that two `GhostCell`s that need to be distinct were actually the same cell.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct GhostAliasingError;

// For uniformity, if anyone wants it. Can't do
// impl<T> From<Infallible> for T
// because of conflicting implementations.
impl From<Infallible> for GhostAliasingError {
    fn from(_: Infallible) -> Self {
        unreachable!("Infallible cannot be constructed")
    }
}

/// A trait for implementing multiple borrows for any number of arguments, using a `GhostToken<'a, 'brand>`.
///
/// Implemented for a mixture of tuple and array types.
///
/// #   Experimental
///
/// The feature is experimental, to enable, use the feature "experimental-multiple-mutable-borrows".
pub trait GhostBorrowMut<'a, 'brand> {
    /// The references you get as a result.
    ///
    /// For example, if `Self` is `(&'a GhostCell<'brand, T0>, &'a GhostCell<'brand, T1>)` then `Result` is
    /// `(&'a mut T0, &'a mut T1)`.
    type Result;

    /// The error case.
    ///
    /// Use a never type such as `!` or `Infallible` if an error is impossible, and `GhostAliasingError` otherwise.
    type Error: Into<GhostAliasingError>;

    /// Borrows any number of `GhostCell`s mutably at the same time.
    ///
    /// If any of them are the same `GhostCell`, returns `Error`.
    ///
    /// #   Performance
    ///
    /// In general, borrowing should be free of cost if possible. This can be ascertained by checking the error type:
    /// if it is a never type, then the operation is infallible, indicating no run-time check is necessary.
    ///
    /// If the operation is not infallible, then a runtime check is necessary, in which case the unchecked version of
    /// the operation may be used if performance matters and the caller is certain of the absence of problems.
    ///