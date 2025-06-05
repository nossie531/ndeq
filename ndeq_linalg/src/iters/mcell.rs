//! Provider of [`MCell`].

use crate::aliases::Pos;

/// Matrix component.
#[derive(Clone, Copy, Debug)]
pub struct MCell<'a, T> {
    /// Position.
    pos: Pos,
    /// Value.
    val: &'a T,
}

impl<'a, T> MCell<'a, T> {
    /// Creates a new value.
    pub fn new(pos: Pos, val: &'a T) -> Self {
        Self { pos, val }
    }

    /// Returns row index.
    pub fn row(&self) -> usize {
        self.pos.0
    }

    /// Returns column index.
    pub fn col(&self) -> usize {
        self.pos.1
    }

    /// Returns position.
    pub fn pos(&self) -> Pos {
        self.pos
    }

    /// Returns value reference.
    pub fn val(&self) -> &'a T {
        self.val
    }
}
