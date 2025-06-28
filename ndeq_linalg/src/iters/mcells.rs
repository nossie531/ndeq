//! Provider of [`MCells`].

use crate::aliases::{Pos, Size};
use crate::iters::MCell;
use crate::parts::{MData, Scalar};
use std::collections::btree_map::Iter as TreeIter;
use std::slice::Iter as SliceIter;

/// Iterator of matrix component.
#[derive(Clone)]
pub struct MCells<'a, T> {
    base: BaseIter<'a, T>,
}

impl<'a, T> MCells<'a, T> {
    /// Creates a new value.
    pub(crate) fn new(mdata: &'a MData<T>, size: Size) -> Self {
        Self {
            base: match mdata {
                MData::Dense(vec) => BaseIter::ForSlice(vec.iter(), size.1, 0),
                MData::Sparse(tree) => BaseIter::ForTree(tree.iter()),
            },
        }
    }
}

impl<'a, T> Iterator for MCells<'a, T>
where
    T: Scalar,
{
    type Item = MCell<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.base {
            BaseIter::ForSlice(iter, m, i) => {
                while let Some(val) = iter.next() {
                    let pos = (*i / *m, *i % *m);
                    *i += 1;

                    if val != T::zero() {
                        return Some(MCell::new(pos, val));
                    }
                }

                None
            }
            BaseIter::ForTree(iter) => {
                let entry = iter.next();
                entry.map(|(&pos, val)| MCell::new(pos, val))
            }
        }
    }
}

/// Base iterator.
#[derive(Clone)]
enum BaseIter<'a, T> {
    /// For slice iterator.
    ForSlice(SliceIter<'a, T>, usize, usize),
    /// For tree iterator.
    ForTree(TreeIter<'a, Pos, T>),
}
