//! Provider of [`MData`].

use crate::linear::Scalar;
use crate::linear::mds::Matc;
use std::collections::BTreeMap;

/// Matrix main data.
#[derive(Clone, PartialEq)]
pub enum MData<T> {
    Dense(Vec<T>),
    Sparse(BTreeMap<(usize, usize), T>),
}

impl<T> MData<T>
where
    T: Scalar,
{
    /// Creates a new value.
    pub fn new(m: usize, n: usize, sparse: bool) -> Self {
        if sparse {
            Self::Sparse(BTreeMap::new())
        } else {
            let size = m * n;
            let mut vals = Vec::with_capacity(size);
            vals.extend((0..size).map(|_| T::default()));
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
    pub fn get(&self, size: (usize, usize), pos: (usize, usize)) -> T {
        match self {
            Self::Dense(v) => v[pos.0 * size.1 + pos.1],
            Self::Sparse(m) => m.get(&pos).copied().unwrap_or_default(),
        }
    }

    pub fn none_zeros<'a>(
        &'a self,
        size: (usize, usize),
    ) -> Box<dyn Iterator<Item = Matc<T>> + 'a> {
        match self {
            MData::Dense(v) => {
                let ret = v
                    .iter()
                    .enumerate()
                    .filter(|&(_, v)| *v != T::default())
                    .map(move |(i, &v)| {
                        let row = i / size.1;
                        let col = i % size.1;
                        Matc::new((row, col), v)
                    });
                Box::new(ret) as Box<dyn Iterator<Item = _>>
            }
            MData::Sparse(m) => {
                let ret = m.iter().map(|(&pos, &v)| Matc::new(pos, v));
                Box::new(ret) as Box<dyn Iterator<Item = _>>
            }
        }
    }

    /// Sets value to specified position.
    pub fn set(&mut self, size: (usize, usize), pos: (usize, usize), val: T) {
        match self {
            Self::Dense(v) => v[pos.0 * size.1 + pos.1] = val,
            Self::Sparse(m) => {
                if val == T::default() {
                    m.remove(&pos);
                } else {
                    m.insert(pos, val);
                }
            }
        }
    }
}
