//! Provider of [`NdeqNode`].

/// Target node of diffusion calculation.
pub trait NdeqNode<V> {
    /// Get node value.
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
    fn edges(&self) -> Box<dyn Iterator<Item = (V, f32)> + '_>;
}
