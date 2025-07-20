//! Provider of [`MCellsMut`].

use crate::aliases::{Pos, Size};
use crate::iters::MCellMut;
use crate::parts::Scalar;
use crate::parts::strage::SelfStrage;
use std::collections::btree_map::IterMut as TreeIterMut;
use std::slice::IterMut as SliceIterMut;

/// Mutable iterator of matrix none zero component.
pub struct NzIterMut<'a, T>
where
    T: Scalar,
{
    base: BaseIterMut<'a, T>,
}

impl<'a, T> NzIterMut<'a, T>
where
    T: Scalar,
{
    /// Creates a new value.
    pub(crate) fn new(mdata: &'a mut SelfStrage<T>, size: Size) -> Self {
        Self {
            base: match mdata {
                SelfStrage::Dense(vec) => BaseIterMut::ForSlice(vec.iter_mut(), size.1, 0),
                SelfStrage::Sparse(tree) => BaseIterMut::ForTree(tree.iter_mut()),
            },
        }
    }
}

impl<'a, T> Iterator for NzIterMut<'a, T>
where
    T: Scalar,
{
    type Item = MCellMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.base {
            BaseIterMut::ForSlice(iter, m, i) => {
                while let Some(val) = iter.next() {
                    let pos = (*i / *m, *i % *m);
                    *i += 1;

                    if val != T::zero() {
                        return Some(MCellMut::new(pos, val));
                    }
                }

                None
            }
            BaseIterMut::ForTree(iter) => {
                let entry = iter.next();
                entry.map(|(&pos, val)| MCellMut::new(pos, val))
            }
        }
    }
}

/// Base iterator.
enum BaseIterMut<'a, T> {
    /// For slice iterator.
    ForSlice(SliceIterMut<'a, T>, usize, usize),
    /// For tree iterator.
    ForTree(TreeIterMut<'a, Pos, T>),
}
