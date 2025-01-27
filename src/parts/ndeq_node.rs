//! Provider of [`NdeqNode`].

/// Abstraction trait for network node.
pub trait NdeqNode<V> {
    /// Returns node index that is unique in network.
    ///
    /// # Panics
    ///
    /// Panics if target is currently mutably borrowed.
    fn idx(&self) -> usize;

    /// Returns node value.
    ///
    /// # Panics
    ///
    /// Panics if target is currently mutably borrowed.
    fn value(&self) -> V;

    /// Sets node index that is unique in network.
    ///
    /// # Panics
    ///
    /// Panics if target is currently borrowed.
    fn set_idx(&self, value: usize);

    /// Sets node value.
    ///
    /// # Panics
    ///
    /// Panics if target is currently borrowed.
    fn set_value(&self, value: V);

    /// Returns edges iterator.
    ///
    /// # Panics
    ///
    /// Panics if target is currently mutably borrowed.    
    fn edges(&self) -> Box<dyn Iterator<Item = (usize, f32)> + '_>;
}
