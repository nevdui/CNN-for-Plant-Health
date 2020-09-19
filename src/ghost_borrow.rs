
//! The `GhostBorrow` trait allows simultaneously borrowing multiple `GhostCell` immutably.
//!
//! Technically, this is already allowed, however it can be useful to do so as a single expression, which this trait
//! provides.

use core::mem;
use core::ptr;

use crate::ghost_cell::*;

/// A trait for implementing multiple borrows for any number of arguments, using a `GhostToken<'a, 'brand>`.
///
/// Implemented for a mixture of tuple and array types.
pub trait GhostBorrow<'a, 'brand> {
    /// The references you get as a result.
    ///
    /// For example, if `Self` is `(&'a GhostCell<'brand, T0>, &'a GhostCell<'brand, T1>)` then `Result` is
    /// `(&'a T0, &'a T1)`.
    type Result;

    /// Borrows any number of `GhostCell`s at the same time.
    ///
    /// #   Example
    ///
    /// ```rust
    /// use ghost_cell::{GhostToken, GhostCell, GhostBorrow};
    ///
    /// let value = GhostToken::new(|mut token| {
    ///     let cell1 = GhostCell::new(42);
    ///     let cell2 = GhostCell::new(47);
    ///
    ///     let (reference1, reference2): (&i32, &i32) = (&cell1, &cell2).borrow(&token);
    ///
    ///     (*reference1, *reference2)
    /// });
    ///
    /// assert_eq!((42, 47), value);
    /// ```
    fn borrow(self, token: &'a GhostToken<'brand>) -> Self::Result;
}

impl<'a, 'brand, T> GhostBorrow<'a, 'brand> for &'a [GhostCell<'brand, T>] {
    type Result = &'a [T];

    fn borrow(self, _: &'a GhostToken<'brand>) -> Self::Result {
        //  Safety:
        //  -   Shared access to the `GhostToken` ensures shared access to the cells' content.
        //  -   `GhostCell` is `repr(transparent)`, hence `T` and `GhostCell<T>` have the same memory representation.
        unsafe { mem::transmute::<Self, Self::Result>(self) }
    }
}