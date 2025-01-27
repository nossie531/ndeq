//! Provider of [`Yp`].

/// Derivative function type.
///
/// Internal closure calculates slope at point of the second
/// argument, and writes it to the first argument.
pub type Yp<'a, V> = dyn Fn(&mut V, &V) + 'a;
