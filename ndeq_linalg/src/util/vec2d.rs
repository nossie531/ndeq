//! Provider of [`Vec2d`].

use crate::aliases::{Pos, Size};
use std::ops::Index;
use std::slice::{Iter, IterMut};
use std::{iter, ops::IndexMut};

/// Two dimension vector.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vec2d<T> {
    vec: Vec<T>,
    size: Size,
}

impl<T> Vec2d<T> {
    /// Creates a new value.
    pub fn new(size: Size) -> Self
    where
        T: Default,
    {
        let len = size.0 * size.1;
        let mut vec = Vec::with_capacity(len);
        vec.extend(iter::repeat_with(T::default).take(len));
        Self { vec, size }
    }

    /// Returns the number of components.
    pub fn len(&self) -> usize {
        self.size.0 * self.size.1
    }

    /// Returns size of this.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Returns iterator over the components.
    pub fn iter(&self) -> Iter<T> {
        self.vec.iter()
    }

    /// Returns mutable iterator over the components.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.vec.iter_mut()
    }
}

impl<T> Index<Pos> for Vec2d<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.vec[index.0 * self.size.1 + index.1]
    }
}

impl<T> IndexMut<Pos> for Vec2d<T> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.vec[index.0 * self.size.1 + index.1]
    }
}
