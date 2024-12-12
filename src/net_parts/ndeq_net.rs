//! Provider of [`NdeqNet`].

use crate::net_parts::NdeqNode;
use crate::values::Value;

/// Network.
#[derive(Default)]
pub struct NdeqNet<V> {
    /// Nodes of network.
    nodes: Vec<NdeqNode<V>>,
}

impl<V> NdeqNet<V>
where
    V: Value,
{
    /// Create new instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns nodes slice.
    pub fn nodes(&self) -> &[NdeqNode<V>] {
        self.nodes.as_slice()
    }

    /// Returns edges from specified node.
    pub fn edges_of(&self, node_idx: usize) -> impl Iterator<Item = (V, f32)> + '_ {
        let bwd_node = &self.nodes[node_idx];
        bwd_node.edges().map(|(fwd_node_idx, weight)| {
            let fwd_node = &self.nodes[fwd_node_idx];
            (fwd_node.value(), weight)
        })
    }

    /// Add node.
    pub fn add_node(&mut self, value: V) -> usize {
        self.nodes.push(NdeqNode::new(value));
        self.nodes.len() - 1
    }

    /// Add edge.
    ///
    /// # Panics
    ///
    /// Panics if `bwd_idx` or `fwd_idx` is out of range.
    pub fn add_edge(&mut self, bwd_idx: usize, fwd_idx: usize, weight: f32) {
        assert!((0..self.nodes.len()).contains(&bwd_idx));
        assert!((0..self.nodes.len()).contains(&fwd_idx));
        self.nodes[bwd_idx].add_edge(fwd_idx, weight);
    }

    /// Returns mutable nodes slice.
    pub fn nodes_mut(&mut self) -> &mut [NdeqNode<V>] {
        self.nodes.as_mut_slice()
    }

    /// Update node values.
    pub(crate) fn update_values(&mut self) {
        for node in self.nodes.iter_mut() {
            node.update_value();
        }
    }
}
