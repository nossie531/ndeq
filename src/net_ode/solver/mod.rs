//! Network ODE solver.

pub mod adapters;

pub use ei_solver::*;
pub use net_ode_solver::*;

mod ei_solver;
mod net_ode_solver;
