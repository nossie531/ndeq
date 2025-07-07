//! Network diffusion simulator.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]

pub mod prelude;
pub mod solver;

pub use net_flow::*;
pub use node_val::*;
pub use node_vec::*;

mod net_flow;
mod node_val;
mod node_vec;
