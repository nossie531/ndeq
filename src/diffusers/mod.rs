//! Diffusion algorithms.

mod diffuser;
mod euler;
mod runge_kutta;

pub use diffuser::*;
pub use euler::*;
pub use runge_kutta::*;
