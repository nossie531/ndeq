//! Provider of [`Matc`].

use crate::linear::Scalar;

/// Matrix component.
pub struct Matc<T> {
    pos: (usize, usize),
    val: T,
}

impl<T> Matc<T>
where
    T: Scalar
{
    /// Creates a new value.
    pub fn new(pos: (usize, usize), val: T) -> Self {
        Self {
            pos, val, 
        }
    }

    /// Returns row index.
    pub fn row(&self) -> usize {
        self.pos.0
    }

    /// Returns column index.
    pub fn col(&self) -> usize {
        self.pos.1
    }

    pub fn pos(&self) -> (usize, usize) {
        self.pos
    }

    /// Returns value.
    pub fn val(&self) -> T {
        self.val
    }
}
