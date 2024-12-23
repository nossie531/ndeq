//! Provider of [`NetView`].

use crate::ode::df::Yp;
use crate::ode::values::Value;

/// Network.
pub trait NetView<V>
where
    V: Value,
{
    /// Returns edges.
    /// 
    /// # Panics
    /// 
    /// Panics if `self` or its nodes are currently mutably borrowed.
    fn edges(&self) -> Box<dyn Iterator<Item = (usize, usize, f32)> + '_>;

    /// Load node values to vector.
    /// 
    /// # Panics
    /// 
    /// Panics if `self` or its nodes are currently borrowed.
    fn load_values(&self, values: &mut Vec<V>);

    /// Set node values.
    /// 
    /// # Panics
    /// 
    /// Panics if any of the following occurs.
    /// 
    /// * `values` length is not equal to nodes count.
    /// * `self` or its nodes are currently mutably borrowed.
    fn set_values(&self, values: &[V]);

    /// Returns derivative function for network diffusion.
    /// 
    /// # Panics
    /// 
    /// Panics if `self` or its nodes are currently mutably borrowed.
    fn yp(&self) -> Yp<'_, V> {
        Box::new(move |results, values| {
            results.fill(V::default());

            for (bwd_idx, fwd_idx, w) in self.edges() {
                let bwd_value = values[bwd_idx];
                let fwd_value = values[fwd_idx];
                let flow = (fwd_value - bwd_value) * w;
                results[bwd_idx] = results[bwd_idx] + flow;
            }
        })
    }
}
