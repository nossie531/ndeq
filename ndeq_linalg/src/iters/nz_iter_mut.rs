//! Provider of [`MCellsMut`].

use crate::aliases::Pos;
use crate::iters::MCellMut;
use crate::parts::Scalar;
use crate::parts::strage::{MatrixStrage, OwnStrage};
use std::collections::btree_map::IterMut as TreeIterMut;
use std::slice::IterMut as SliceIterMut;

/// Mutable iterator of matrix none zero component.
pub struct NzIterMut<'a, T>(BaseIterMut<'a, T>);

impl<'a, T> NzIterMut<'a, T>
where
    T: Scalar,
{
    /// Creates a new value.
    pub(crate) fn new(mdata: &'a mut OwnStrage<T>) -> Self {
        let cn = mdata.size().1;
        let base = match mdata {
            OwnStrage::Dense(vec) => BaseIterMut::ForSlice(vec.iter_mut(), cn, 0),
            OwnStrage::Sparse(tree) => BaseIterMut::ForTree(tree.iter_mut()),
        };

        Self(base)
    }
}

impl<'a, T> Iterator for NzIterMut<'a, T>
where
    T: Scalar,
{
    type Item = MCellMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
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
