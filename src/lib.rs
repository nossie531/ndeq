//! Network diffusion simulator.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]

pub mod ode;
pub mod parts;
pub mod prelude;

pub use ndeq_sim::*;

mod ndeq_sim;
mod util;
