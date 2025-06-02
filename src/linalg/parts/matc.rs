//! Provider of [`Matc`].

use crate::linalg::parts::{Pos, Scalar};

/// Matrix component.
#[derive(Clone, Copy, Debug)]
pub struct Matc<T> {
    /// Position.
    pos: Pos,
    /// Value.
    val: T,
}

impl<T> Matc<T>
where
    T: Scalar,
{
    /// Creates a new value.
    pub fn new(pos: Pos, val: T) -> Self {
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

    pub fn pos(&self) -> Pos {
        self.pos
    }

    /// Returns value.
    pub fn val(&self) -> T {
        self.val
    }
}
