//! Network diffusion simulator.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

pub mod prelude;
pub mod step_algorithms;
pub mod values;

mod ndeq_node;
mod ndeq_runner;
mod step_algorithm;

pub use ndeq_node::*;
pub use ndeq_runner::*;
pub use step_algorithm::*;
