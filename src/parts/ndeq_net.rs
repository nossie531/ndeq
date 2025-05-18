//! Provider of [`NdeqNet`].

use crate::ode::Slope;
use crate::ode::values::{VArr, Value};
use std::rc::Rc;

/// Abstraction trait for Network.
pub trait NdeqNet<V>
where
    V: Value,
{
    /// Returns edges.
    ///
    /// # Panics
    ///
    /// Panics if `self` or its nodes are currently mutably borrowed.
    fn edges(&self) -> Box<dyn Iterator<Item = (usize, usize, f32)> + '_>;

    /// Imports node values from slice.
    ///
    /// # Panics
    ///
    /// Panics if any of the following occurs.
    ///
    /// * `values` length is not equal to nodes count.
    /// * `self` or its nodes are currently mutably borrowed.
    fn import_values(&self, values: &[V]);

    /// Exports node values to vector.
    ///
    /// # Panics
    ///
    /// Panics if `self` or its nodes are currently borrowed.
    fn export_values(&self, values: &mut Vec<V>);

    /// Returns derivative function for network diffusion.
    ///
    /// # Panics
    ///
    /// Panics if `self` or its nodes are currently mutably borrowed.
    fn slope(&self) -> Rc<Slope<VArr<V>>> {
        Rc::new(|result, value| {
            result.fill_zero();

            for (bwd_idx, fwd_idx, w) in self.edges() {
                let bwd_value = &value[bwd_idx];
                let fwd_value = &value[fwd_idx];
                let mut flow = V::default();
                flow += fwd_value;
                flow -= bwd_value;
                flow *= w;
                result[bwd_idx] += &flow;
            }
        })
    }
}
