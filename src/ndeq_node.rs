//! Provider of [`NdeqNode`].

use std::rc::Rc;

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

    /// Get intake edges iterator.
    ///
    /// # Panics
    ///
    /// Panics if target is currently mutably borrowed.
    fn in_edges(&self) -> Box<dyn Iterator<Item = (Rc<dyn NdeqNode<V>>, f32)> + '_>;
}
