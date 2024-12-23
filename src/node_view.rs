//! Provider of [`NodeView`].

/// Node.
pub trait NodeView<V> {
    /// Returns work index.
    /// 
    /// # Panics
    ///
    /// Panics if target is currently mutably borrowed.
    fn work_idx(&self) -> usize;

    /// Set work index.
    /// 
    /// # Panics
    ///
    /// Panics if target is currently borrowed.
    fn set_work_idx(&self, value: usize);

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
