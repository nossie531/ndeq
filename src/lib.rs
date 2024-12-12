//! Network diffusion simulator.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]

pub mod ode;
pub mod prelude;
pub mod util;

pub use ndeq_net::*;
pub use ndeq_sim::*;

mod ndeq_net;
mod ndeq_sim;
