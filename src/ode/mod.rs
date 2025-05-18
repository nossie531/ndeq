//! [ODE] (ordinary differential equation) system.
//!
//! [ODE]: https://en.wikipedia.org/wiki/Ordinary_differential_equation

pub mod ode_util;
pub mod solver;
pub mod values;

pub use slope::*;

mod slope;
