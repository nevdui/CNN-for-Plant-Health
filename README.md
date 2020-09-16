
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