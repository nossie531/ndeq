//! Provider of [`MatrixStrageMut`].

use crate::aliases::Pos;
use crate::iters::NzIterMut;
use crate::parts::strage::MatrixStrage;
use crate::parts::Scalar;

pub trait MatrixStrageMut<T>: MatrixStrage<T>
where 
    T: Scalar
{
    /// Sets value at specified position.
    fn set_value(&mut self, pos: Pos, value: T);

    /// Returns mutable none-zero components iterator.
    fn nz_iter_mut(&mut self) -> NzIterMut<'_, T>;    
}