//! Network ODE solver.

pub mod solvers;

pub use ei_solver::*;
pub use net_ode_solver::*;
pub use univ_net_ode_solver::*;

mod ei_solver;
mod net_ode_solver;
mod univ_net_ode_solver;
