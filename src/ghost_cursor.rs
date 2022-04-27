//! A `GhostCursor` implements a cursor to navigate across a web of `GhostCell`s.
//!
//! #   Safety
//!
//! This is an untrodden path, here be dragons.
//!
//! ##  Safety: Aliasing.
//!
//! The `GhostCursor` trivially enforces safe aliasing by always tying the lifetime of the token it materializes to its
//! own lifetime.
//!
//! The `GhostCursor` itself is therefore borrowed mutably or immutably for the duration of the lifetime of the token,
//! preventing any abuse.
//!
//! ##  Safety: Lifetime
//!
//! This is the crux of the issue, and the most likely place for unsoundness in the whole scheme.
//!
//! Let us start by a broken example to better understand what we are looking for. Let us imagine a simple doubly linked
//! list data structure where each node has two optional fields, a previous and a next field, to point to the previous and next node,
//! respectively.
//!
//! Imagine the following set-up with 2 nodes `a` and `b`:
//!
//! -   On the stack is `root`, a pointer owning half of `a` -- the other half doesn't matter here.
//! -   `a.prev` and `a.next` are each a pointer owning half of `b`.
//!
//! Any method which allows both obtaining a reference to `b` and simultaneously a mutable reference to `a` is unsound,
//! for owning a mutable reference to `a` allows setting both of its `prev` and `next` fields to `None`, dropping `b`,
//! and of course retaining a reference to a dropped element is opening the door to undefined behavior.
//!
//! Hence, the very stringent invariant that the `GhostCursor` must enforce is that apart from the currently mutable
//! element, no reference to other elements with no _separate_ anchoring ownership paths to the stack can be observed.
//!
//! Or, in short, the `GhostCursor` may allow either:
//!
//! -   Observing multiple immutable references at a time.
//! -   Or observing a single mutable reference at a time.
//!
//! A familiar restriction for Rust programmers.

use core::ptr::NonNull;

use super::{GhostCell, GhostToken};

/// A `GhostCursor`, to navigate across a web of `GhostCell`s.
pub struct GhostCursor<'a, 'brand, T: ?Sized> {
    token: NonNull<GhostToken<'brand>>,
    cell: Option<&'a GhostCell<'brand, T>>,
}

impl<'a, 'brand, T: ?Sized> GhostCursor<'a, 'brand, T> {
    /// Creates a new instance of the cursor.
    pub fn new(token: &'a mut GhostToken<'brand>, cell: Option<&'a GhostCell<'brand, T>>) -> Self {
        let token = NonNull::from(token);

        Self { token, cell }
    }

    /// Returns a mutable reference to the current element, if any.
    ///
    /// #   Safety
    ///
    /// The token is still mutably borrowed for as long as the return value lives.
    pub fn into_inner(self) -> Option<&'a mut T> {
        //  Safety:
        //  -   `self` is not borrowed, therefore the token is not borrowed.
        //  -   The lifetime of the result ensures that the token remains mutably borrowed for as long as the result
        //      exists.
        let token = unsafe { as_mut(self.token) };

        self.cell.map(move |cell| cell.borrow_mut(token))
    }

    /// Returns the token and currently pointed to cell.
    ///
    /// #   Safety
    ///
    /// The token is still mutably borrowed for as long as the return value lives.
    pub fn into_parts(self) -> (&'a GhostToken<'brand>, Option<&'a GhostCell<'brand, T>>) {
        //  Why cannot `into_parts` returns a mutable reference to `GhostToken`?
        //
        //  This is a tempting option, as it would match the constructor (`new`) perfectly. It is also unsound,
        //  unfortunately, as demonstrated in #25.
        //
        //  The current reference pointed to by the cursor may be owned (transitively) by another `GhostCell`.
        //  Returnin