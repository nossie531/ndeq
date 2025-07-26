use crate::parts::Scalar;
use crate::prelude::*;
use crate::parts::strage::{MatrixStrage, MatrixStrageMut, OwnStrage};
use std::marker::PhantomData;

/// Vector builder.
pub struct VectorBuilder<T, N> {
    /// Vector length.
    len: usize,

    /// Vector internal data.
    mdata: OwnStrage<T>,

    /// Phantom data.
    pd: PhantomData<N>,
}

impl<T, N> VectorBuilder<T, N>
where
    T: Scalar,
{
    /// Creates a new value.
    pub(crate) fn new(len: usize) -> Self {
        Self {
            len,
            mdata: OwnStrage::new((len, 1), false),
            pd: Default::default(),
        }
    }

    /// Builds matrix.
    #[must_use]
    pub fn build(self) -> Vector<T, N> {
        Vector::<T, N>::new_internal(self.mdata)
    }

    /// Sets values.
    ///
    /// # Panics
    ///
    /// Panic if any of the following occurs.
    ///
    /// * `self` has data already.
    /// * `values` item is out of range.
    #[must_use]
    pub fn values<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = (usize, T)>,
    {
        assert!(self.mdata.nz_iter().next().is_none());
        for (i, v) in values {
            assert!(i < self.len);
            self.mdata.set_value((i, 0), v);
        }

        self
    }
}
