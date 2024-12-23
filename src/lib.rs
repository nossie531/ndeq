//! Network diffusion simulator.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

pub mod ode;
pub mod prelude;
pub mod util;

pub use ndeq_sim::*;
pub use net_view::*;
pub use node_view::*;

mod ndeq_sim;
mod net_view;
mod node_view;
