//! ODE solver.

pub mod solvers;

pub use univ_ode_solver::*;
pub use ode_solver::*;

mod univ_ode_solver;
mod ode_solver;
