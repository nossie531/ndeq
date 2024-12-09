//! Provider of [`NodeView`].

/// Node view.
pub trait NodeView<V> {
    /// Returns key.
    ///
    /// The key must be unique in the network.
    ///
    /// For example.
    ///
    /// * Array index - When node is in array.
    /// * Pointer address - When node is on smart pointer.
    fn key(&self) -> usize;

    /// Returns node value.
    ///
    /// # Panics
    ///
    /// Panics if target is currently mutably borrowed.
    fn value(&self) -> V;

    /// Set node value.
    ///
    /// # Panics
    ///
    /// Panics if target is currently borrowed.
    fn set_value(&self, value: V);

    /// Get edges iterator.
    ///
    /// # Panics
    ///
    /// Panics if target is currently mutably borrowed.    
    fn edges(&self) -> Box<dyn Iterator<Item = (usize, f32)> + '_>;
}
