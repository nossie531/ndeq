//! Provider of [`Matrix`].

use crate::linalg::parts::{MData, Matc, Pos, Scalar, Size};
use std::ops::{AddAssign, Mul};

/// [Matrix].
///
/// [Matrix]: https://en.wikipedia.org/wiki/Matrix_(mathematics)
#[derive(Clone, Debug)]
pub struct Matrix<T> {
    size: Size,
    data: MData<T>,
}

impl<T> Matrix<T>
where
    T: Scalar,
{
    /// Creates a new value.
    #[must_use]
    pub fn new(size: Size, sparse: bool) -> Self {
        let data = MData::new(size, sparse);
        Self { size, data }
    }

    /// Returns `true` if storage format is for sparse matrix.
    #[must_use]
    pub fn is_sparse(&self) -> bool {
        self.data.is_sparse()
    }

    /// Returns row length.
    #[must_use]
    pub fn m(&self) -> usize {
        self.size.0
    }

    /// Returns column length.
    #[must_use]
    pub fn n(&self) -> usize {
        self.size.1
    }

    /// Returns size.
    #[must_use]
    pub fn size(&self) -> Size {
        self.size
    }

    /// Sets value from specified position.
    #[must_use]
    pub fn get(&self, pos: Pos) -> T {
        assert!((0..self.size.0).contains(&pos.0));
        assert!((0..self.size.1).contains(&pos.1));
        self.data.get(self.size, pos)
    }

    /// Clone this matrix with sparse flag.
    #[must_use]
    pub fn clone_sparse(&self, sparse: bool) -> Self {
        let mut ret = Self::new(self.size, sparse);
        for mc in self.none_zeros() {
            ret.set(mc.pos(), self.get(mc.pos()));
        }

        ret
    }

    /// Sets value to specified position.
    pub fn set(&mut self, pos: Pos, value: T) {
        assert!((0..self.size.0).contains(&pos.0));
        assert!((0..self.size.1).contains(&pos.1));
        self.data.set(self.size, pos, value);
    }
}

impl<T> AddAssign<&Self> for Matrix<T>
where
    T: Scalar,
{
    /// Performs the `+=` operation.
    ///
    /// # Panics
    ///
    /// Panics if both operands size are not same.
    fn add_assign(&mut self, rhs: &Self) {
        assert_eq!(self.size(), rhs.size());

        let method = if rhs.is_sparse() {
            Self::add_assign_for_sparse_rhs
        } else {
            Self::add_assign_for_dense_rhs
        };

        method(self, rhs);
    }
}

impl<T> Mul for &Matrix<T>
where
    T: Scalar,
{
    type Output = Matrix<T>;

    /// Performs the `*` operation.
    ///
    /// # Result matrix storage format
    ///
    /// Result matrix storage format is sparse, if and only if both operands
    /// have sparse storage formats.
    ///
    /// # Panics
    ///
    /// Panics if the number of rows of itself and the number of columns on
    /// the right side are not the same.
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.n(), rhs.m());

        let use_sparse_calc = self.is_sparse() || rhs.is_sparse();
        let return_sparse_fmt = self.is_sparse() && rhs.is_sparse();
        let method = if use_sparse_calc {
            Matrix::mul_with_sparse
        } else {
            Matrix::mul_without_sparse
        };

        let ret = method(&self, rhs);
        assert_eq!(ret.is_sparse(), return_sparse_fmt);
        ret
    }
}

impl<T> PartialEq for Matrix<T>
where
    T: Scalar,
{
    /// Tests for `self` and `other` values to be equal, and is used by `==`.
    ///
    /// # Comparison rule
    ///
    /// Comparisons are based only on whether the values of all components match
    /// ([`Self::sparse`] does not affect the result).
    fn eq(&self, other: &Self) -> bool {
        let method = if self.is_sparse() == other.is_sparse() {
            Self::eq_by_inside
        } else {
            Self::eq_by_outside
        };

        method(&self, other)
    }
}

impl<T> Matrix<T>
where
    T: Scalar,
{
    /// Returns none-zero components iterator.
    fn none_zeros(&self) -> Box<dyn Iterator<Item = Matc<T>> + '_> {
        self.data.none_zeros(self.size)
    }

    /// Compare this matrix to other matrix with their inside.
    fn eq_by_inside(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        self.data.eq(&other.data)
    }

    /// Compare this matrix to other matrix with their outside.
    fn eq_by_outside(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        let sm = if self.is_sparse() { self } else { other };
        let dm = if self.is_sparse() { other } else { self };
        for mc in sm.none_zeros() {
            let sv = mc.val();
            let dv = dm.get(mc.pos());
            if sv != dv {
                return false;
            }
        }

        true
    }

    /// Perform add assign with dense matrix on the right-hand side.
    fn add_assign_for_dense_rhs(&mut self, rhs: &Self) {
        for i in 0..rhs.m() {
            for j in 0..rhs.n() {
                let curr = self.get((i, j));
                let addition = rhs.get((i, j));
                self.set((i, j), curr + addition);
            }
        }
    }

    /// Perform add assign with sparse matrix on the right-hand side.
    fn add_assign_for_sparse_rhs(&mut self, rhs: &Self) {
        for mc in rhs.none_zeros() {
            let curr = self.get(mc.pos());
            let addition = mc.val();
            self.set(mc.pos(), curr + addition)
        }
    }

    /// Perform multiple operation without sparse matrix.
    fn mul_without_sparse(&self, rhs: &Self) -> Self {
        let ret_for_sparse = self.data.is_sparse() && rhs.data.is_sparse();
        let mut ret = Matrix::<T>::new((self.m(), rhs.n()), ret_for_sparse);

        for i in 0..ret.m() {
            for j in 0..ret.n() {
                let mut sum = T::default();
                for k in 0..self.n() {
                    sum += self.get((i, k)) * rhs.get((k, j));
                }

                ret.set((i, j), sum);
            }
        }

        ret
    }

    /// Perform multiple operation with sparse matrix.
    fn mul_with_sparse(&self, rhs: &Self) -> Self {
        let ret_for_sparse = self.data.is_sparse() && rhs.data.is_sparse();
        let mut ret = Matrix::<T>::new((self.m(), rhs.n()), ret_for_sparse);
        let mut lhs_iter = self.none_zeros();
        let mut rhs_iter = rhs.none_zeros();
        let mut last_lc = <Option<Matc<T>>>::None;
        let mut last_rc = <Option<Matc<T>>>::None;

        while let Some(lc) = lhs_iter.next() {
            if Some(lc.row()) != last_lc.map(|x| x.row()) {
                last_lc = Some(lc);
                last_rc = None;
                rhs_iter = rhs.none_zeros();
            }

            while let Some(rc) = last_rc.or_else(|| rhs_iter.next()) {
                last_rc = None;

                if rc.row() == lc.col() {
                    let curr = ret.get((lc.row(), rc.col()));
                    ret.set((lc.row(), rc.col()), curr + lc.val() * rc.val());
                }

                if rc.row() > lc.col() {
                    last_rc = Some(rc);
                    break;
                }
            }
        }

        ret
    }
}
