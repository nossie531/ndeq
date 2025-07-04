//! Network diffusion simulator.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]

pub mod net_ode;
pub mod prelude;

pub use ndeq_flow::*;

mod ndeq_flow;
