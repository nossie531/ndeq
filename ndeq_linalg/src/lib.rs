//! [Linear algebra] module for `ndeq`.
//!
//! [Linear algebra]: https://en.wikipedia.org/wiki/Linear_algebra

pub mod aliases;
pub mod iters;
pub mod num;
pub mod parts;

pub use matrix::*;

mod matrix;
