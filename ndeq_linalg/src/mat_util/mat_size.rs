//! Provider of [`MatSize`].

use crate::aliases::Size;

/// Matrix size.
pub struct MatSize(pub Size);

impl MatSize {
    pub fn len(&self) -> usize {
        self.0.0 * self.0.1
    }
}
