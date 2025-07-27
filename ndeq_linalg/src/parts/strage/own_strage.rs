//! Provider of [`OwnStrage`].

use crate::aliases::{Pos, Size};
use crate::iters::{NzIter, NzIterMut};
use crate::mat_util::MatPos;
use crate::parts::Scalar;
use crate::parts::strage::{MatrixStrage, MatrixStrageMut, OwnDokStrage, OwnVecStrage};

/// Self owning matrix strage.
#[derive(Clone, Debug, PartialEq)]
pub enum OwnStrage<T> {
    /// Dense strage format.
    Dense(OwnVecStrage<T>),
    /// Sparse strage format.
    Sparse(OwnDokStrage<T>),
}

impl<T> OwnStrage<T>
where 
    T: Scalar
{
    /// Creates a new value.
    pub fn new(size: Size, sparse: bool) -> Self {
        if sparse {
            Self::Sparse(OwnDokStrage::new(size))
        } else {
            Self::Dense(OwnVecStrage::new(size))
        }
    }
}

impl<T> MatrixStrage<T> for OwnStrage<T>
where 
    T: Scalar,
{
    /// Returns `true` if storage format is for sparse matrix.
    fn is_sparse(&self) -> bool {
        match self {
            Self::Dense(_) => false,
            Self::Sparse(_) => true,
        }
    }

    /// Returns size of this.
    fn size(&self) -> Size {
        match self {
            Self::Dense(x) => x.size,
            Self::Sparse(x) => x.size,
        }
    }

    /// Returns value at specified position.
    fn value(&self, pos: Pos) -> &T {
        match self {
            Self::Dense(x) => &x.vec[MatPos(pos).on(x.size)],
            Self::Sparse(x) => x.map.get(&pos).unwrap_or_else(|| T::zero()),
        }
    }

    /// Returns none-zero components iterator.
    fn nz_iter(&self) -> NzIter<'_, T> {
        NzIter::new(self)
    }
}

impl<T> MatrixStrageMut<T> for OwnStrage<T>
where
    T: Scalar,
{
    /// Sets value to specified position.
    fn set_value(&mut self, pos: Pos, value: T) {
        match self {
            Self::Dense(x) => {
                x.vec[MatPos(pos).on(x.size)] = value
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
    fn nz_iter_mut<'a>(&'a mut self) -> NzIterMut<'a, T> {
        NzIterMut::new(self)
    }
}
