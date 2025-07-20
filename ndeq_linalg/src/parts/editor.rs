use crate::aliases::Pos;
use crate::parts::Scalar;
use crate::prelude::*;
use crate::parts::strage::SelfStrage;
use std::collections::btree_map::OccupiedEntry;
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
    pub fn new<R, C>(matrix: &'a mut Matrix<T, R, C>, pos: Pos) -> Self {
        match matrix.mdata_mut() {
            SelfStrage::Dense(x) => Self::Dense(x.at_mut(pos)),
            SelfStrage::Sparse(x) => {
                Self::Sparse(Some(x.at_mut(pos)))
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
