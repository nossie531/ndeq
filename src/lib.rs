//! Network diffusion simulator.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]

pub mod diffusers;
pub mod prelude;
pub mod util;
pub mod values;

mod ndeq_node;
mod ndeq_sim;
mod net_parts;

pub use ndeq_node::*;
pub use ndeq_sim::*;
pub use net_parts::*;
