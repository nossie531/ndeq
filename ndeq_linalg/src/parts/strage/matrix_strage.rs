//! Provider of [`MatrixStrage`].

use crate::aliases::{Pos, Size};
use crate::iters::NzIter;
use crate::mat_util::MatSize;
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
    fn nz_iter(&self) -> NzIter<'_, T>;

    /// Returns internal data length.
    fn len(&self) -> usize {
        MatSize(self.size()).len()
    }
}