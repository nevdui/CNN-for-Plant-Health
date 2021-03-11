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
    /// Creates a fresh token to which `GhostCell`s can be tied to later.
    ///
    /// Due to the use of a lifetime, the `GhostCell`s tied to a given token can only live within the confines of the
    /// invocation of the `fun` closure.
    ///
    /// #   Example
    ///
    /// ```rust
    /// use ghost_cell::{GhostToken, GhostCell};
    ///
    /// let n = 12;
    ///
    /// let value = GhostToken::new(|mut token| {
    ///     let cell = GhostCell::new(42);
    ///
    ///     let vec: Vec<_> = (0..n).map(|_| &cell).collect();
    ///
    ///     *vec[n / 2].borrow_mut(&mut token) = 33;
    ///
    ///     *cell.borrow(&token)
    /// });
    ///
    /// assert_eq!(33, value);
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new<R, F>(fun: F) -> R
    where
        for<'new_brand> F: FnOnce(GhostToken<'new_brand>) -> R,
    {
        let token = Self { _marker: InvariantLifetime::default() };
        fun(token)
    }
}

/// A `GhostToken` is stateless, therefore it can safely be passed across threads.
unsafe impl<'brand> Send for GhostToken<'brand> {}

/// A `GhostToken` is stateless, therefore it can safely be accessed from different threads.
unsafe impl<'brand> Sync for GhostToken<'brand> {}

/// Branded wrapper for a value, whose type is `T`.
///
/// A `GhostCell<'x, T>` owns an instance of type `T`:
/// -   Unique access to the cell allows unimpeded access to the contained value.
/// -   Shared access to the cell requires mediating access through the associated `GhostToken<'x, T>` which will
///     enforce at compile-time the aliasing XOR mutability safety property.
#[repr(transparent)]
pub struct GhostCell<'brand, T: ?Sized> {
    _marker: InvariantLifetime<'brand>,
    value: UnsafeCell<T>,
}

impl<'brand, T> GhostCell<'brand, T> {
    /// Wraps some `T` into a `GhostCell` with brand `'brand` which associates it to one, and only one, `GhostToken`.
    ///
    /// #   Example
    ///
    /// ```rust
    /// use ghost_cell::{GhostToken, GhostCell};
    ///
    /// GhostToken::new(|token| {
    ///     let cell = GhostCell::new(42);
    ///
    ///     assert_eq!(42, *cell.borrow(&token));
    /// });
    /// ```
    pub const fn new(value: T) -> Self {
        let _marker = PhantomData;
        let value = UnsafeCell::new(value);

        Self { _marker, value }
    }

    /// Turns an owned `GhostCell` back into owned data.
    ///
    /// #   Example
    ///
    /// ```rust
    /// use ghost_cell::{GhostToken, GhostCell};
    ///
    /// let value = GhostToken::new(|mut token| {
    ///     let cell = GhostCell::new(42);
    ///
    ///     cell.into_inner()
    /// });
    ///
    /// assert_eq!(42, value);
    /// ```
    pub fn into_inner(self) -> T { self.value.into_inner() }
}

impl<'brand, T: ?Sized> GhostCell<'brand, T> {
    /// Immutably borrows the `GhostCell` with the same-branded token.
    ///
    /// #   Example
    ///
    /// ```rust
    /// use ghost_cell::{GhostToken, GhostCell};
    ///
    /// let n = 12;
    ///
    /// let value = GhostToken::new(|mut token| {
    ///     let cell = GhostCell::new(42);
    ///
    ///     let vec: Vec<_> = (0..n).map(|_| &cell).collect();
    ///
    ///     let one: &i32 = vec[1].borrow(&token);
    ///     let two: &i32 = vec[2].borrow(&token);
    ///
    ///     *one + *two
    /// });
    ///
    /// assert_eq!(84, value);
    /// ```
    pub fn borrow<'a>(&'a self, _: &'a GhostToken<'brand>) -> &'a T {
        //  Safety:
        //  -   The cell is borrowed immutably by this call, it therefore cannot already be borrowed mutably.
        //  -   The token is borrowed immutably by this call, it therefore cannot be already borrowed mutably.
        //  -   `self.value` therefore cannot be already borrowed mutably, as doing so requires calling either:
        //      -   `borrow_mut`, which would borrow the token mutably.
        //      -   `get_mut`, which woul