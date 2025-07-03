//! Type aliases.

use std::rc::Rc;

/// Derivative function type.
///
/// Internal closure calculates slope at point of the second
/// argument, and writes it to the first argument.
pub type FnSlope<'a, V> = Rc<dyn Fn(&mut V, &V) + 'a>;