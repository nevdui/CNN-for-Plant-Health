
A novel safe and zero-cost borrow-checking paradigm from the
[`GhostCell`](https://plv.mpi-sws.org/rustbelt/ghostcell/) paper.


#   Motivation

A number of collections, such as linked lists, binary trees, or B-trees are most easily implemented with aliasing
pointers.

Traditionally, this means using run-time borrow-checking in order to still be able to mutate said structures, or using
`unsafe` in the name of performance.

By using _brands_, `GhostCell` separate the data from the permission to mutate it, and uses a unique `GhostToken` to
model this permission, tied at compile-time to a number of said `GhostCell`s via the _brand_.


#   Safety

In the GhostCell paper, Joshua Yanovski and his colleagues from MPI-SWS, Germany, formally demonstrate the safety of
`GhostCell` using the separation logic they have developed as part of the
[RustBelt project](https://plv.mpi-sws.org/rustbelt/). I personally would trust them on this.

The official implementation can be found at https://gitlab.mpi-sws.org/FP/ghostcell/-/tree/master/ghostcell, along with
examples. The current implementation will be upgraded soonish, now that I'm aware of it.

Use at your own risks!

_(And please report any issue)_


#   Maturity

This is very much an Alpha quality release, _at best_.

Documentation:

-   All methods are documented.
-   All non-trivial methods have examples.

Tests:

-   All non-trivial methods are tested, via their examples.
-   All methods with safety invariants are covered with compile-fail tests.
-   The entire test-suite, including examples, runs under Miri.


#   How to use?

Let's start with a self-contained example:

```rust
use ghost_cell::{GhostToken, GhostCell};

fn demo(n: usize) {
    let value = GhostToken::new(|mut token| {
        let cell = GhostCell::new(42);

        let vec: Vec<_> = (0..n).map(|_| &cell).collect();

        *vec[n / 2].borrow_mut(&mut token) = 33;

        *cell.borrow(&token)
    });

    assert_eq!(value, 33);
}
```

`GhostToken` uses the best known way to generate a unique lifetime, hence used as a brand, which is to combine:

-   A local variable, created within the `GhostToken::new` method.
-   A closure which must be valid for all lifetimes.

This means 2 restrictions:

-   The closure must be variant over the lifetimes, this does not always play well with closures already containing
    references.
-   None of the branded items can be returned by the closure.

Then, within the closure, any `GhostCell` can be associated to one, and only one, `GhostToken` which will encode its