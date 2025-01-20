//! [ODE] (ordinary differential equation) system.
//!
//! [ODE]: https://en.wikipedia.org/wiki/Ordinary_differential_equation

pub mod solver;
pub mod values;

pub use yp::*;

mod yp;
