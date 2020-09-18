
//! The `GhostBorrow` trait allows simultaneously borrowing multiple `GhostCell` immutably.
//!
//! Technically, this is already allowed, however it can be useful to do so as a single expression, which this trait
//! provides.

use core::mem;
use core::ptr;

use crate::ghost_cell::*;

/// A trait for implementing multiple borrows for any number of arguments, using a `GhostToken<'a, 'brand>`.