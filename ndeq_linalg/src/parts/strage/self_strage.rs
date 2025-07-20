//! Provider of [`SelfStrage`].

use crate::aliases::{Pos, Size};
use crate::iters::{NzIter, NzIterMut};
use crate::parts::Scalar;
use crate::parts::strage::{Dense, Sparse};

/// Self owning matrix strage.
#[derive(Clone, Debug, PartialEq)]
pub enum SelfStrage<T> {
    /// Dense strage format.
    Dense(Dense<T>),
    /// Sparse strage format.
    Sparse(Sparse<T>),
}

impl<T> SelfStrage<T>
where
    T: Scalar,
{
    /// Creates a new value.
    pub fn new(size: Size, sparse: bool) -> Self {
        if sparse {
            Self::Sparse(Sparse::new(size))
        } else {
            Self::Dense(Dense::new(size))
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
            Self::Dense(x) => x.size.0 * x.size.1,
            Self::Sparse(x) => x.size.0 * x.size.1,
        }
    }

    /// Returns size of this.
    pub fn size(&self) -> Size {
        match self {
            Self::Dense(x) => x.size,
            Self::Sparse(x) => x.size,
        }
    }

    /// Returns value at specified position.
    pub fn value(&self, pos: Pos) -> &T {
        match self {
            Self::Dense(x) => &x.vec[pos.0 * x.size.1 + pos.1],
            Self::Sparse(x) => x.map.get(&pos).unwrap_or_else(|| T::zero()),
        }
    }

    /// Returns none-zero components iterator.
    pub fn nz_iter<'a>(&'a self) -> NzIter<'a, T> {
        NzIter::new(self)
    }

    /// Sets value to specified position.
    pub fn set_value(&mut self, pos: Pos, value: T) {
        match self {
            Self::Dense(x) => {
                x.vec[pos.0 * x.size.1 + pos.1] = value
            },
            Self::Sparse(x) => {
                if value == *T::zero() {
                    x.map.remove(&pos);
                } else {
                    x.map.insert(pos, value);
                }
            },
        }
    }

    /// Returns mutable none-zero components iterator.
    pub fn nz_iter_mut<'a>(&'a mut self, size: Size) -> NzIterMut<'a, T> {
        NzIterMut::new(self, size)
    }
}
