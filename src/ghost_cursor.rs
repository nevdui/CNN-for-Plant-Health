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
//! list data structure where each node has two optional fields, a previous and a next field