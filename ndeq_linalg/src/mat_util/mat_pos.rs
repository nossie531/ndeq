//! Provider of [`MatPos`].

use crate::aliases::{Pos, Size};

/// Matrix position.
pub struct MatPos(pub Pos);

impl MatPos {
    pub fn is_in(&self, size: Size) -> bool {
        self.base().0 < size.0 && self.base().1 < size.1
    }

    pub fn on(&self, size: Size) -> usize {
        self.base().0 * size.1 + self.base().1
    }

    pub fn base(&self) -> &Pos {
        &self.0
    }
}
