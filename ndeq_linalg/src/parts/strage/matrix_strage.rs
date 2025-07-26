//! Provider of [`MatrixStrage`].

use crate::aliases::{Pos, Size};
use crate::iters::{NzIter, NzIterMut};
use crate::parts::Scalar;

pub trait MatrixStrage<T>
where 
    T: Scalar
{
    /// Returns `true` if storage format is for sparse matrix.
    fn is_sparse(&self) -> bool;

    /// Returns size of this.
    fn size(&self) -> Size;

    /// Returns value at specified position.
    fn value(&self, pos: Pos) -> &T;

    /// Returns none-zero components iterator.
    fn nz_iter<'a>(&'a self) -> NzIter<'a, T>;

    /// Sets value at specified position.
    fn set_value(&mut self, pos: Pos, value: T);

    /// Returns mutable none-zero components iterator.
    fn nz_iter_mut<'a>(&'a mut self, size: Size) -> NzIterMut<'a, T>;    

    /// Returns internal data length.
    fn len(&self) -> usize {
        self.size().0 * self.size().1
    }
}