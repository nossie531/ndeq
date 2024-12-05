//! Provider of [`Net`].

use crate::net_parts::Node;
use crate::values::Value;
use crate::NdeqNode;
use std::collections::BTreeMap;

/// Network.
#[derive(Default)]
pub struct Net<'a, V> {
    /// Nodes of network.
    nodes: Vec<Node<V>>,

    /// Original nodes.
    originals: Vec<&'a dyn NdeqNode<V>>,
}

impl<'a, V> Net<'a, V>
where
    V: Value,
{
    /// Returns nodes slice.
    pub fn nodes(&self) -> &[Node<V>] {
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
    pub fn nodes_mut(&mut self) -> &mut [Node<V>] {
        self.nodes.as_mut_slice()
    }

    /// Create new instance from network nodes.
    pub(crate) fn from_nodes<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a dyn NdeqNode<V>>,
    {
        let mut nodes = Vec::new();
        let mut originals = Vec::new();
        let mut key_to_index = BTreeMap::new();

        for (i, original) in iter.into_iter().enumerate() {
            nodes.push(Node::new(original.value()));
            originals.push(original);
            key_to_index.insert(original.key(), i);
        }

        for (i, original) in originals.iter().enumerate() {
            for (key, weight) in original.edges() {
                let index = key_to_index[&key];
                nodes[i].add_edge(index, weight);
            }
        }

        Self { nodes, originals }
    }

    /// Update node values.
    pub(crate) fn update_values(&mut self) {
        for node in self.nodes.iter_mut() {
            node.update_value();
        }
    }

    /// Update original node values from current node values.
    pub(crate) fn update_originals(&mut self) {
        for (i, original) in self.originals.iter_mut().enumerate() {
            let value = self.nodes[i].calced_value();
            original.set_value(value);
        }
    }
}
