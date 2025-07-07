//! Network ODE solver.

pub mod solvers;

pub use net_ode_solver::*;
pub use univ_net_ode_solver::*;

mod net_ode_solver;
mod univ_net_ode_solver;
