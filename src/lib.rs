//! Network diffusion simulator.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

pub mod prelude;
pub mod diffusion_sims;
pub mod values;

mod ndeq_node;
mod diffusion_sim;

pub use ndeq_node::*;
pub use diffusion_sim::*;
