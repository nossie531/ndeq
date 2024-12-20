//! Provider of [`Yp`].

/// Derivative function type.
///
/// Internal closure calculates slope at point of the second
/// argument, and writes it to the first argument.
pub type Yp<V> = Box<dyn Fn(&mut [V], &[V])>;
