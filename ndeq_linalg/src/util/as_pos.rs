//! Provider of [`AsPos`].

use crate::aliases::Size;

/// Treat tuple as position.
pub struct AsPos(pub (usize, usize));

impl AsPos {
    pub fn is_in(&self, size: Size) -> bool {
        let this = self.0;
        this.0 < size.0 && this.1 < size.1
    }
}
