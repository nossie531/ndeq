//! Provider of [`NdeqNode`].

use crate::values::Value;
use std::collections::BTreeMap;

/// Network node.
#[derive(Default)]
pub struct NdeqNode<V> {
    /// value.
    value: V,

    /// New value.
    new_value: V,

    /// Edges to other nodes.
    edges: BTreeMap<usize, f32>,
}

impl<V> NdeqNode<V>
where
    V: Value,
{
    /// Create new instance.
    pub fn new(value: V) -> Self {
        Self {
            value,
            new_value: Default::default(),
            edges: Default::default(),
        }
    }

    /// Returns value.
    pub fn value(&self) -> V {
        self.value
    }

    /// Returns new value.
    pub fn new_value(&self) -> V {
        self.new_value
    }

    /// Set new value.
    pub fn set_new_value(&mut self, value: V) {
        self.new_value = value
    }

    /// Returns edges (node-index and weight tuples) to other nodes.
    pub fn edges(&self) -> impl Iterator<Item = (usize, f32)> + '_ {
        self.edges.iter().map(|(&index, &weight)| (index, weight))
    }

    /// Add edge to other node.
    pub(crate) fn add_edge(&mut self, index: usize, weight: f32) {
        self.edges.insert(index, weight);
    }

    /// Update value from calced value.
    pub(crate) fn update_value(&mut self) {
        self.value = self.new_value;
    }
}
