//! Provider of [`MData`].

use crate::aliases::{Pos, Size};
use crate::iters::{NzIter, NzIterMut};
use crate::parts::Scalar;
use crate::util::Vec2d;
use std::collections::BTreeMap;

/// Matrix main data.
#[derive(Clone, Debug, PartialEq)]
pub enum MData<T> {
    /// Dense strage format.
    Dense(Vec2d<T>),
    /// Sparse strage format.
    Sparse(BTreeMap<Pos, T>),
}

impl<T> MData<T>
where
    T: Scalar,
{
    /// Creates a new value.
    pub fn new(size: Size, sparse: bool) -> Self {
        if sparse {
            Self::Sparse(BTreeMap::new())
        } else {
            Self::Dense(Vec2d::new(size))
        }
    }

    /// Returns `true` if storage format is for sparse matrix.
    pub fn is_sparse(&self) -> bool {
        match self {
            Self::Dense(_) => false,
            Self::Sparse(_) => true,
        }
    }

    /// Returns internal data length.
    pub fn len(&self) -> usize {
        match self {
            Self::Dense(x) => x.len(),
            Self::Sparse(x) => x.len(),
        }
    }

    /// Returns component of specified position.
    pub fn get(&self, pos: Pos) -> &T {
        match self {
            Self::Dense(v) => &v[pos],
            Self::Sparse(m) => m.get(&pos).unwrap_or_else(|| T::zero()),
        }
    }

    /// Returns none-zero components iterator.
    pub fn nz_iter<'a>(&'a self) -> NzIter<'a, T> {
        NzIter::new(self)
    }

    /// Sets value to specified position.
    pub fn set(&mut self, pos: Pos, val: T) {
        match self {
            Self::Dense(v) => v[pos] = val,
            Self::Sparse(m) => {
                if val == *T::zero() {
                    m.remove(&pos);
                } else {
                    m.insert(pos, val);
                }
            }
        }
    }

    /// Returns mutable none-zero components iterator.
    pub fn nz_iter_mut<'a>(&'a mut self, size: Size) -> NzIterMut<'a, T> {
        NzIterMut::new(self, size)
    }
}
