//! Provider of [`NdeqNet`].

use crate::ode::df::Yp;
use crate::ode::values::Value;
use std::collections::BTreeMap;

/// Network.
#[derive(Default, Clone)]
pub struct NdeqNet<V> {
    /// Node values.
    values: Vec<V>,
    /// Node Edges.
    edges: Vec<BTreeMap<usize, f32>>,
}

impl<V> NdeqNet<V>
where
    V: Value,
{
    /// Creates a new instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns node values.
    pub fn values(&self) -> &[V] {
        self.values.as_slice()
    }

    /// Returns mutable node values.
    pub fn values_mut(&mut self) -> &mut [V] {
        self.values.as_mut_slice()
    }

    /// Returns derivative function for network diffusion.
    pub fn yp(&self) -> Yp<V> {
        let edges = self.edges.clone();
        Box::new(move |results, values| {
            assert_eq!(values.len(), edges.len());
            for (bwd_idx, edges) in edges.iter().enumerate() {
                let curr = values[bwd_idx];
                let fwds = edges.iter().map(|(&i, &w)| (values[i], w));
                let flows = fwds.map(|(v, w)| (v - curr) * w);
                results[bwd_idx] = V::sum(flows);
            }
        })
    }

    /// Adds a node and returns added node index.
    pub fn add_node(&mut self, value: V) -> usize {
        self.values.push(value);
        self.edges.push(BTreeMap::new());
        self.values.len() - 1
    }

    /// Adds an edge.
    ///
    /// # Panics
    ///
    /// Panics if `bwd_idx` or `fwd_idx` is out of range.
    pub fn add_edge(&mut self, bwd_idx: usize, fwd_idx: usize, weight: f32) {
        assert!((0..self.values.len()).contains(&bwd_idx));
        assert!((0..self.values.len()).contains(&fwd_idx));
        self.edges[bwd_idx].insert(fwd_idx, weight);
    }
}
