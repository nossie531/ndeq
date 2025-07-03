//! Provider of [`NdeqNet`].

use dyn_compatible::prelude::*;
use ndeq_linalg::prelude::*;
use ndeq_ode::FnSlope;
use ndeq_ode::values::{OdeValue, RF32};
use std::rc::Rc;
use crate::net_ode::NodeValues;

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
    fn slope(&self) -> FnSlope<NodeValues<V>> {
        Rc::new(|result, value| {
            result.fill_zero();

            for (bwd_idx, fwd_idx, w) in self.edges() {
                if bwd_idx == fwd_idx {
                    continue;
                }

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

    /// Returns network laplacian.
    ///
    /// # Panics
    ///
    /// Panics if `self` or its nodes are currently mutably borrowed.
    fn laplacian(&self) -> Matrix<f32> {
        let mut ret = Matrix::new((self.len(), self.len()), true);

        for (bwd_idx, fwd_idx, w) in self.edges() {
            if bwd_idx == fwd_idx {
                continue;
            }

            *ret.cell((bwd_idx, fwd_idx)) = w;
            *ret.cell((fwd_idx, fwd_idx)) -= w;
        }

        ret
    }
}
