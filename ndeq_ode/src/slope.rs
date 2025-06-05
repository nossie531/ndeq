//! Provider of [`Slope`].

/// Derivative function type.
///
/// Internal closure calculates slope at point of the second
/// argument, and writes it to the first argument.
pub type Slope<'a, V> = dyn Fn(&mut V, &V) + 'a;
