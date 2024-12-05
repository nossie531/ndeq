//! Provider of [`Node`].

use crate::values::Value;
use std::collections::BTreeMap;

/// Network node.
pub struct Node<V> {
    /// value.
    value: V,

    /// Calced value.
    calced_value: V,

    /// Edges to other nodes.
    edges: BTreeMap<usize, f32>,
}

impl<V> Node<V>
where
    V: Value,
{
    /// Returns value.
    pub fn value(&self) -> V {
        self.value
    }

    /// Returns calced value.
    pub fn calced_value(&self) -> V {
        self.calced_value
    }

    /// Set calced value.
    pub fn set_calced_value(&mut self, value: V) {
        self.calced_value = value
    }

    /// Create instance.
    pub(crate) fn new(value: V) -> Self {
        Self {
            value,
            calced_value: Default::default(),
            edges: Default::default(),
        }
    }

    /// Returns edges (node-index and weight tuples) to other nodes.
    pub(crate) fn edges(&self) -> impl Iterator<Item = (usize, f32)> + '_ {
        self.edges.iter().map(|(&index, &weight)| (index, weight))
    }

    /// Update value from calced value.
    pub(crate) fn update_value(&mut self) {
        self.value = self.calced_value;
    }

    /// Add edge to other node.
    pub(crate) fn add_edge(&mut self, index: usize, weight: f32) {
        self.edges.insert(index, weight);
    }
}
