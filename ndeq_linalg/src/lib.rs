//! [Linear algebra] module for `ndeq`.
//!
//! [Linear algebra]: https://en.wikipedia.org/wiki/Linear_algebra

pub mod aliases;
pub mod iters;
pub mod parts;
pub mod prelude;

pub use matrix::*;

mod matrix;
