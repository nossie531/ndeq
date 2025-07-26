use crate::aliases::{Pos, Size};
use crate::parts::Scalar;
use crate::prelude::*;
use crate::parts::strage::{MatrixStrage, OwnStrage};
use std::marker::PhantomData;

/// Matrix builder.
pub struct MatrixBuilder<T, R, C> {
    /// Matrix size.
    size: Size,

    /// Matrix internal data.
    mdata: OwnStrage<T>,

    /// Phantom data.
    pd: PhantomData<(R, C)>,
}

impl<T, R, C> MatrixBuilder<T, R, C>
where
    T: Scalar,
{
    /// Creates a new value.
    pub(crate) fn new(size: Size) -> Self {
        Self {
            size,
            mdata: OwnStrage::new(size, false),
            pd: Default::default(),
        }
    }

    /// Builds matrix.
    #[must_use]
    pub fn build(self) -> Matrix<T, R, C> {
        Matrix::<T, R, C>::new_internal(self.mdata)
    }

    /// Sets sparse flag.
    ///
    /// # Panics
    ///
    /// Panics if `self` has data already.
    #[must_use]
    pub fn sparse(mut self, value: bool) -> Self {
        assert!(self.mdata.nz_iter().next().is_none());
        self.mdata = OwnStrage::new(self.size, value);
        self
    }

    /// Sets entries.
    ///
    /// # Panics
    ///
    /// Panic if any of the following occurs.
    ///
    /// * `self` has data already.
    /// * `values` item is out of range.
    #[must_use]
    pub fn entries<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = (Pos, T)>,
    {
        assert!(self.mdata.nz_iter().next().is_none());
        for (p, v) in values {
            assert!(p.0 < self.size.0 && p.1 < self.size.1);
            self.mdata.set_value(p, v);
        }

        self
    }
}
