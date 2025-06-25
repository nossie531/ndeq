//! Provider of [`NdeqNet`].

use dyn_compatible::prelude::*;
use ndeq_ode::Slope;
use ndeq_ode::values::{OdeValue, OdeVec, RF32};
use std::rc::Rc;

/// Abstraction trait for Network.
#[dyn_compatible(true)]
pub trait NdeqNet<V>
where
    V: OdeValue,
{
    /// Returns the number of nodes.
    ///
    /// # Panics
    ///
    /// Panics if `self` is currently mutably borrowed.
    fn len(&self) -> usize;

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
    fn slope(&self) -> Rc<Slope<OdeVec<V>>> {
        Rc::new(|result, value| {
            result.fill_zero();

            for (bwd_idx, fwd_idx, w) in self.edges() {
                let bwd_value = &value[bwd_idx];
                let fwd_value = &value[fwd_idx];
                let mut flow = V::default();
                flow += fwd_value;
                flow -= bwd_value;
                flow *= RF32(w);
                result[bwd_idx] += &flow;
            }
        })
    }

    // TODO;
    // fn laplacian(&self) -> Matrix<V> {
    //     let ret = Matrix::new((self.len(), self.len()), true);
    //     ret
    // }
}
