//! Network diffusion simulator.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]

pub mod diffusers;
pub mod net_parts;
pub mod prelude;
pub mod util;
pub mod values;

mod ndeq_sim;

pub use ndeq_sim::*;
