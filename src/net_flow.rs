//! Provider of [`NetFlow`].

use crate::NodeVec;
use dyn_compatible::prelude::*;
use ndeq_linalg::prelude::*;
use ndeq_num::prelude::*;
use ndeq_ode::FnSlope;
use std::rc::Rc;

/// Network value flow.
#[dyn_compatible(true)]
pub trait NetFlow<V>
where
    V: Float,
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

    /// Imports current node values from slice.
    ///
    /// # Panics
    ///
    /// Panics if any of the following occurs.
    ///
    /// * `values` length is not equal to nodes count.
    /// * `self` or its nodes are currently mutably borrowed.
    fn import_curr_values(&self, values: &[V]);

    /// Exports last node values to vector.
    ///
    /// # Panics
    ///
    /// Panics if `self` or its nodes are currently borrowed.
    fn export_last_values(&self, values: &mut Vec<V>);

    /// Returns derivative function for network diffusion.
    ///
    /// # Panics
    ///
    /// Panics if `self` or its nodes are currently mutably borrowed.
    fn slope(&self) -> FnSlope<NodeVec<V>> {
        Rc::new(|result, value| {
            *result *= V::zero();

            for (bwd_idx, fwd_idx, w) in self.edges() {
                if bwd_idx == fwd_idx {
                    continue;
                }

                let bwd_value = &value[bwd_idx];
                let fwd_value = &value[fwd_idx];
                let mut flow = V::zero();
                flow += fwd_value;
                flow -= bwd_value;
                flow *= V::from(w);
                result[bwd_idx] += &flow;
            }
        })
    }

    /// Returns network laplacian.
    ///
    /// # Panics
    ///
    /// Panics if `self` or its nodes are currently mutably borrowed.
    fn laplacian(&self) -> DMatrix<f32> {
        let mut ret = DMatrix::new_sparse((self.len(), self.len()));

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
