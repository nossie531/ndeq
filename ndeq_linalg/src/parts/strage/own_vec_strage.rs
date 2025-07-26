//! Provider of [`OwnVecStrage`].

use crate::aliases::{Pos, Size};
use crate::parts::Scalar;
use std::slice::{Iter, IterMut};
use std::iter;

/// Self owning matrix strage with vector.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnVecStrage<T> {
    pub(crate) vec: Vec<T>,
    pub(crate) size: Size,
}

impl<T> OwnVecStrage<T>
where
    T: Scalar,
{
    /// Creates a new value.
    pub fn new(size: Size) -> Self {
        let len = size.0 * size.1;
        let mut vec = Vec::with_capacity(len);
        vec.extend(iter::repeat_with(T::default).take(len));
        Self { vec, size }
    }

    /// Returns iterator over the components.
    pub fn iter(&self) -> Iter<T> {
        self.vec.iter()
    }

    /// Returns mutable reference at specified position.
    pub fn at_mut(&mut self, pos: Pos) -> &mut T {
        &mut self.vec[pos.0 * self.size.1 + pos.1]
    }

    /// Returns mutable iterator over the components.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.vec.iter_mut()
    }
}
