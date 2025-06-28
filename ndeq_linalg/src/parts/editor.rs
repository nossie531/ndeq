use crate::aliases::Pos;
use crate::parts::{MData, Scalar};
use crate::prelude::*;
use std::collections::btree_map::{Entry, OccupiedEntry};
use std::mem;
use std::ops::{Deref, DerefMut};

/// Matrix component editer.
pub enum Editor<'a, T>
where
    T: Scalar,
{
    Dense(&'a mut T),
    Sparse(Option<OccupiedEntry<'a, Pos, T>>),
}

impl<'a, T> Editor<'a, T>
where
    T: Scalar,
{
    /// Creates a new instance.
    pub fn new(matrix: &'a mut Matrix<T>, pos: Pos) -> Self {
        let n = matrix.n();
        match matrix.mdata_mut() {
            MData::Dense(vec) => Self::Dense(&mut vec[pos.0 * n + pos.1]),
            MData::Sparse(map) => {
                /* Waiting new feature `BTreeMap::insert_entry`.
                Entry search after entry insert is inefficient.
                https://github.com/rust-lang/rust/issues/65225
                */
                map.entry(pos).or_insert(*T::zero());
                let Entry::Occupied(entry) = map.entry(pos) else {
                    panic!()
                };
                Self::Sparse(Some(entry))
            }
        }
    }
}

impl<T> Deref for Editor<'_, T>
where
    T: Scalar,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Dense(x) => x,
            Self::Sparse(x) => x.as_ref().unwrap().get(),
        }
    }
}

impl<T> DerefMut for Editor<'_, T>
where
    T: Scalar,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Dense(x) => x,
            Self::Sparse(x) => x.as_mut().unwrap().get_mut(),
        }
    }
}

impl<T> Drop for Editor<'_, T>
where
    T: Scalar,
{
    fn drop(&mut self) {
        if (*self).deref() == T::zero() {
            if let Self::Sparse(entry) = self {
                let entry = mem::take(entry).unwrap();
                entry.remove();
            }
        }
    }
}
