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

    /// Create new instance from network nodes.
    pub fn from_nodes(nodes: Vec<NdeqNode<V>>) -> Self {
        Self { nodes }
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
