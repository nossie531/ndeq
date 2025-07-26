//! Provider of [`MatPos`].

use crate::aliases::{Pos, Size};

/// Matrix position.
pub struct MatPos(pub Pos, pub Size);

impl MatPos {
    pub fn ok(&self) -> bool {
        let row_ok = self.pos().0 < self.size().0;
        let col_ok = self.pos().1 < self.size().1;
        row_ok && col_ok
    }

    pub fn index(&self) -> usize {
        self.pos().0 + self.size().1 + self.pos().1
    }

    pub fn pos(&self) -> &Pos {
        &self.0
    }

    pub fn size(&self) -> &Size {
        &self.1
    }
}
