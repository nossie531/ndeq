use crate::aliases::{Pos, Size};
use crate::iters::NzIter;
use crate::parts::Scalar;
use crate::parts::strage::MatrixStrage;
use crate::mat_util::{MatPos, MatSize};

/// External referencing matrix strage with vector.
pub struct RefVecStrage<'a, T> {
    vec: &'a Vec<T>,
    size: Size
}

impl<'a, T> RefVecStrage<'a, T> {
    pub fn new(vec: &'a Vec<T>, size: Size) -> Self {
        assert_eq!(vec.len(), MatSize(size).len());
        Self {vec, size}
    }
}

impl<'a, T> MatrixStrage<T> for RefVecStrage<'a, T>
where 
    T: Scalar,
{
    fn is_sparse(&self) -> bool {
        false
    }

    fn size(&self) -> Size {
        self.size
    }

    fn value(&self, pos: Pos) -> &T {
        &self.vec[MatPos(pos, self.size).index()]
    }

    fn nz_iter(&self) -> NzIter<'_, T> {
        todo!()
    }
}