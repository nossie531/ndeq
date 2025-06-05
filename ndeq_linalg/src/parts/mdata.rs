//! Provider of [`MData`].

use crate::aliases::{Pos, Size};
use crate::iters::{MCells, MCellsMut};
use crate::parts::Scalar;
use std::collections::BTreeMap;

/// Matrix main data.
#[derive(Clone, Debug, PartialEq)]
pub enum MData<T> {
    /// Dense strage format.
    Dense(Vec<T>),
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
            let len = size.0 * size.1;
            let mut vals = Vec::with_capacity(len);
            vals.extend((0..len).map(|_| T::zero()));
            Self::Dense(vals)
        }
    }

    /// Returns `true` if storage format is for sparse matrix.
    pub fn is_sparse(&self) -> bool {
        match self {
            Self::Dense(_) => false,
            Self::Sparse(_) => true,
        }
    }

    /// Returns matrix value of specified position.
    pub fn get(&self, size: Size, pos: Pos) -> T {
        match self {
            Self::Dense(v) => v[pos.0 * size.1 + pos.1],
            Self::Sparse(m) => m.get(&pos).copied().unwrap_or_else(|| T::zero()),
        }
    }

    /// Returns none-zero components iterator.
    pub fn nz_iter<'a>(&'a self, size: Size) -> MCells<'a, T> {
        MCells::new(self, size)
    }

    /// Sets value to specified position.
    pub fn set(&mut self, size: Size, pos: Pos, val: T) {
        match self {
            Self::Dense(v) => v[pos.0 * size.1 + pos.1] = val,
            Self::Sparse(m) => {
                if val == T::zero() {
                    m.remove(&pos);
                } else {
                    m.insert(pos, val);
                }
            }
        }
    }

    /// Returns mutable none-zero components iterator.
    pub fn nz_iter_mut<'a>(&'a mut self, size: Size) -> MCellsMut<'a, T> {
        MCellsMut::new(self, size)
    }
}
