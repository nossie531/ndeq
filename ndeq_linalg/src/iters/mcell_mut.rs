//! Provider of [`MCellMut`].

use crate::aliases::Pos;
use crate::parts::Scalar;

/// Mutable matrix component.
#[derive(Debug)]
pub struct MCellMut<'a, T> {
    /// Position.
    pos: Pos,
    /// Value.
    val: &'a mut T,
}

impl<'a, T> MCellMut<'a, T>
where
    T: Scalar,
{
    /// Creates a new value.
    pub fn new(pos: Pos, val: &'a mut T) -> Self {
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
    pub fn val(&'a self) -> &'a T {
        self.val
    }

    /// Returns mutable value reference.
    pub fn val_mut(&'a mut self) -> &'a mut T {
        self.val
    }
}
