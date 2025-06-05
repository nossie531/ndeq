//! Provider of [`Matrix`].

use crate::aliases::{Pos, Size, Vector};
use crate::iters::{MCell, MCells, MCellsMut};
use crate::parts::{MData, Scalar, Real};
use std::mem;
use std::ops::{AddAssign, Mul, MulAssign};

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

    /// Creates an identity matrix.
    #[must_use]
    pub fn identity(len: usize) -> Self {
        let sparse = Self::sparse_prefered(len, (len, len));
        let mut ret = Self::new((len, len), sparse);
        for i in 0..len {
            for j in 0..len {
                let val = if i == j { T::one() } else { T::zero() };
                ret.set((i, j), val);
            }
        }

        ret
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
        for mc in self.nz_iter() {
            ret.set(mc.pos(), self.get(mc.pos()));
        }

        ret
    }

    /// Calculate matrix exponential of 'self' on 'vec'.
    ///
    /// # Panics
    ///
    /// Panics if `self` is not square matrix.
    pub fn expmv(&self, vec: &Vector<T>) -> Vector<T> {
        assert_eq!(self.size.0, self.size.1);
        assert_eq!(vec.size.0, self.size.0);
        assert_eq!(vec.size.1, 1);
        let mut ret = Matrix::new(vec.size, false);
        let mut term = vec.clone();
        let mut work = Matrix::new(vec.size, false);
        // TODO: 1..10 はものすごく適当…。
        // max_scale_nrom を使用予定。
        todo!();
        for i in 1..10 {
            self.mul_to(&term, &mut work);
            mem::swap(&mut term, &mut work);
            term *= T::from(1.0 / i as f32);
            ret += &term;
        }

        ret
    }

    /// Sets value to specified position.
    pub fn set(&mut self, pos: Pos, value: T) {
        assert!((0..self.size.0).contains(&pos.0));
        assert!((0..self.size.1).contains(&pos.1));
        self.data.set(self.size, pos, value);
    }

    /// Perform multiple operation and store its result to `out`.
    ///
    /// # Panics
    ///
    /// Panics if any of the following occurs.
    ///
    /// * `work` size missmatch to result size.
    /// * `self` columns count and `rhs` rows count do not match.
    fn mul_to(&self, rhs: &Self, out: &mut Self) {
        assert_eq!(self.n(), rhs.m());

        let use_sparse_calc = self.is_sparse() || rhs.is_sparse();
        let method = if use_sparse_calc {
            Matrix::mul_with_sparse_to
        } else {
            Matrix::mul_without_sparse_to
        };

        method(&self, rhs, out);
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
        if self.size != other.size {
            return false;
        }

        if self.is_sparse() == other.is_sparse() {
            return self.data.eq(&other.data);
        }

        let sm = if self.is_sparse() { self } else { other };
        let dm = if self.is_sparse() { other } else { self };
        for mc in sm.nz_iter() {
            let sv = *mc.val();
            let dv = dm.get(mc.pos());
            if sv != dv {
                return false;
            }
        }

        true
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
    /// Panics if `self` columns count and `rhs` rows count do not match.
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.n(), rhs.m());
        let sparse = self.is_sparse() && rhs.is_sparse();
        let mut ret = Matrix::<T>::new((self.m(), rhs.n()), sparse);
        self.mul_to(rhs, &mut ret);
        ret
    }
}

impl<T> Mul<T> for &Matrix<T>
where
    T: Scalar,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut ret = Matrix::new(self.size, self.is_sparse());
        for mc in self.nz_iter() {
            ret.set(mc.pos(), *mc.val() * rhs);
        }

        ret
    }
}

impl<T> MulAssign<T> for Matrix<T>
where
    T: Scalar,
{
    fn mul_assign(&mut self, rhs: T) {
        for mut mc in self.nz_iter_mut() {
            let val = *mc.val();
            *mc.val_mut() = val * rhs;
        }
    }
}

impl<T> Matrix<T>
where
    T: Scalar,
{
    /// Returns whether sparse matrix format is recommended.
    ///
    /// If `nnz` (The Number of Non Zero) is much small than `size` (Matrix size),
    /// sparse storage format is suggested. However, if `size` is too small, the
    /// trend is lost and dense matrix format is suggested. This is because the
    /// sparse matrix format has large memory overhead and poor memory locality.
    fn sparse_prefered(nnz: usize, size: Size) -> bool {
        static OFFSET: usize = 1024;
        static DENSITY_LIMIT: f32 = 0.1;
        let len = size.0 * size.1;
        let ntr = (nnz + OFFSET) as f32;
        let dtr = (len + OFFSET) as f32;
        ntr / dtr < DENSITY_LIMIT
    }

    /// Returns max scale norm.
    ///
    /// "Max scale norm" is coined term by author.
    /// This value is calculated by the following steps.
    /// 1. For each row, sum absolute of all components and take log base 2 of it.
    /// 2. Select the one with the largest absolute value of them.
    fn max_scale_nrom(&self) -> T::Real {
        let mut ret = T::Real::zero();
        let mut row = T::Real::zero();
        for mc in self.nz_iter() {
            todo!()
        }

        ret
    }

    /// Returns none-zero components iterator.
    fn nz_iter(&self) -> MCells<'_, T> {
        self.data.nz_iter(self.size)
    }

    /// Perform add assign with dense matrix on the right-hand side.
    fn add_assign_for_dense_rhs(&mut self, rhs: &Self) {
        assert_eq!(self.size(), rhs.size());
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
        assert_eq!(self.size(), rhs.size());
        for mc in rhs.nz_iter() {
            let curr = self.get(mc.pos());
            let addition = *mc.val();
            self.set(mc.pos(), curr + addition)
        }
    }

    /// Perform multiple operation without sparse matrix.
    fn mul_without_sparse_to(&self, rhs: &Self, out: &mut Self) {
        assert_eq!(self.n(), rhs.m());
        assert_eq!((self.m(), rhs.n()), out.size);

        for i in 0..out.m() {
            for j in 0..out.n() {
                let mut sum = T::zero();
                for k in 0..self.n() {
                    sum += self.get((i, k)) * rhs.get((k, j));
                }

                out.set((i, j), sum);
            }
        }
    }

    /// Perform multiple operation with sparse matrix.
    fn mul_with_sparse_to(&self, rhs: &Self, out: &mut Self) {
        assert_eq!(self.n(), rhs.m());
        assert_eq!((self.m(), rhs.n()), out.size);

        let mut lhs_iter = self.nz_iter();
        let mut rhs_iter = rhs.nz_iter();
        let mut last_lc = <Option<MCell<T>>>::None;
        let mut last_rc = <Option<MCell<T>>>::None;

        while let Some(lc) = lhs_iter.next() {
            if Some(lc.row()) != last_lc.map(|x| x.row()) {
                last_lc = Some(lc);
                last_rc = None;
                rhs_iter = rhs.nz_iter();
            }

            while let Some(rc) = last_rc.or_else(|| rhs_iter.next()) {
                last_rc = None;

                if rc.row() == lc.col() {
                    let curr = out.get((lc.row(), rc.col()));
                    out.set((lc.row(), rc.col()), curr + *lc.val() * *rc.val());
                }

                if rc.row() > lc.col() {
                    last_rc = Some(rc);
                    break;
                }
            }
        }
    }

    /// Returns mutable none-zero components iterator.
    fn nz_iter_mut(&mut self) -> MCellsMut<'_, T> {
        self.data.nz_iter_mut(self.size)
    }
}
