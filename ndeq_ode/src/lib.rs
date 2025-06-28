//! [ODE] (ordinary differential equation) system for `ndeq`.
//!
//! [ODE]: https://en.wikipedia.org/wiki/Ordinary_differential_equation

pub mod prelude;
pub mod solver;
pub mod tools;
pub mod values;

pub use aliases::*;

mod aliases;
mod util;
