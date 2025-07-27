//! Provider of [`Matrix`].

use crate::aliases::{Pos, Size};
use crate::builders::{MatrixBuilder, VectorBuilder};
use crate::iters::{MCell, NzIter, NzIterMut};
use crate::parts::len::{Fixed, Single, Var};
use crate::parts::{Editor, Scalar};
use crate::parts::strage::{MatrixStrage, MatrixStrageMut, OwnStrage, OwnVecStrage};
use crate::mat_util::{self, MatPos};
use iter_chunks_ext::prelude::*;
use ndeq_num::prelude::*;
use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use std::mem;
use std::ops::{AddAssign, Index, Mul, MulAssign};

/// [Matrix].
///
/// [Matrix]: https://en.wikipedia.org/wiki/Matrix_(mathematics)
pub struct Matrix<T, R, C, S = OwnStrage<T>>
where 
    T: Scalar,
    S: MatrixStrage<T>
{
    /// Internal data.
    data: S,
    /// Phantom data.
    pd: PhantomData<(T, R, C)>,
}

/// Vector (Single column matrix).
pub type Vector<T, N> = Matrix<T, N, Single>;

/// Static size vector.
pub type SVector<T, const N: usize> = Vector<T, Fixed<N>>;

/// Dynamic size vector.
pub type DVector<T> = Vector<T, Var>;

/// Static size matrix.
pub type SMatrix<T, const R: usize, const C: usize> = Matrix<T, Fixed<R>, Fixed<C>>;

/// Dynamic size matrix.
pub type DMatrix<T> = Matrix<T, Var, Var>;

impl<T, const N: usize> SVector<T, N>
where
    T: Scalar,
{
    /// Creates a new vector.
    #[must_use]
    pub fn new() -> Self {
        let ret = DMatrix::make((N, 1)).build();
        unsafe { ret.mimic() }
    }

    /// Creates a vector builder.
    #[must_use]
    pub fn make() -> VectorBuilder<T, Fixed<N>> {
        VectorBuilder::new(N)
    }
}

impl<T> DVector<T>
where
    T: Scalar,
{
    /// Creates a new vector.
    #[must_use]
    pub fn new(len: usize) -> Self {
        let ret = DMatrix::make((len, 1)).build();
        unsafe { ret.mimic() }
    }

    /// Creates a vector builder.
    #[must_use]
    pub fn make(len: usize) -> VectorBuilder<T, Var> {
        VectorBuilder::new(len)
    }
}

impl<T, N> Vector<T, N>
where
    T: Scalar,
{
    /// Returns mutable reference of vector component.
    ///
    /// # Panics
    ///
    /// Panics if `pos` is out of range.
    #[must_use]
    pub fn val(&mut self, index: usize) -> Editor<'_, T> {
        assert!(index < self.rn());
        Editor::new(self, (index, 0))
    }
}

impl<T, const N: usize> SMatrix<T, N, N>
where
    T: Scalar,
{
    /// Creates an identity matrix.
    #[must_use]
    pub fn new_identity() -> Self {
        let ret = DMatrix::new_identity(N);
        unsafe { ret.mimic() }
    }
}

impl<T, const R: usize, const C: usize> SMatrix<T, R, C>
where
    T: Scalar,
{
    /// Creates a new matrix.
    #[must_use]
    pub fn new() -> Self {
        Self::make().build()
    }

    /// Creates a new sparse matrix.
    #[must_use]
    pub fn new_sparse() -> Self {
        Self::make().sparse(true).build()
    }

    /// Creates a matrix builder.
    #[must_use]
    pub fn make() -> MatrixBuilder<T, Fixed<R>, Fixed<C>> {
        MatrixBuilder::new((R, C))
    }
}

impl<T> DMatrix<T>
where
    T: Scalar,
{
    /// Creates a new matrix.
    #[must_use]
    pub fn new(size: Size) -> Self {
        Self::make(size).build()
    }

    /// Creates a new sparse matrix.
    #[must_use]
    pub fn new_sparse(size: Size) -> Self {
        Self::make(size).sparse(true).build()
    }

    /// Creates an identity matrix.
    #[must_use]
    pub fn new_identity(len: usize) -> Self {
        let nnz = len;
        let size = (len, len);
        let sparse = mat_util::is_sparse_prefered(nnz, size);
        let mut ret = Self::make(size).sparse(sparse).build();
        for i in 0..len {
            for j in 0..len {
                let val = if i == j { T::one() } else { T::zero() };
                *ret.cell((i, j)) = *val;
            }
        }

        ret
    }

    /// Creates a matrix builder.
    #[must_use]
    pub fn make(size: Size) -> MatrixBuilder<T, Var, Var> {
        MatrixBuilder::new(size)
    }
}

impl<T, R, C> Matrix<T, R, C>
where
    T: Scalar,
{
    /// Returns `true` if storage format is for sparse matrix.
    #[must_use]
    pub fn is_sparse(&self) -> bool {
        self.data.is_sparse()
    }

    #[must_use]
    pub fn is_square(&self) -> bool {
        self.size().0 == self.size().1
    }

    /// Returns `true` if this is zero matrix.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.nz_iter().next().is_none()
    }

    /// Returns `true` if this is identity matrix.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        if !self.is_square() {
            return false;
        }

        let x_iter = self.nz_iter().map(|x| (x.pos(), x.val()));
        let y_iter = (0..self.cn()).map(|x| ((x, x), T::one()));
        x_iter.eq(y_iter)
    }

    /// Returns row length.
    #[must_use]
    pub fn rn(&self) -> usize {
        self.size().0
    }

    /// Returns column length.
    #[must_use]
    pub fn cn(&self) -> usize {
        self.size().1
    }

    /// Returns the number of components.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns size.
    #[must_use]
    pub fn size(&self) -> Size {
        self.data.size()
    }

    /// Clone this matrix size.
    #[must_use]
    pub fn clone_size(&self) -> Self {
        let ret = DMatrix::make(self.size()).sparse(self.is_sparse()).build();
        unsafe { ret.mimic() }
    }

    /// Clone this matrix with sparse flag.
    #[must_use]
    pub fn clone_with_sparse(&self, sparse: bool) -> Self {
        let mut ret = DMatrix::make(self.size()).sparse(sparse).build();
        for mc in self.nz_iter() {
            *ret.cell(mc.pos()) = self[mc.pos()];
        }

        unsafe { ret.mimic() }
    }

    /// Calculate matrix exponential of 'self' on 'vec'.
    ///
    /// # Panics
    ///
    /// Panics if `self` is not square matrix.
    #[must_use]
    pub fn expmv(&self, vec: &Vector<T, R>) -> Vector<T, R> {
        assert!(self.is_square());
        assert_eq!(vec.len(), self.cn());
        let mut ret = vec.clone_size();
        let mut work = vec.clone_size();
        let mut term = vec.clone();
        // TODO: 1..10 はものすごく適当…。
        // let s = self.max_scale_nrom();
        // let scale = T::Real::from(s as f32).exp2();
        // let matrix = self * T::from(T::Real::from(1.) / scale);

        for i in 1..10 {
            self.mul_to(&term, &mut work);
            mem::swap(&mut term, &mut work);
            term *= T::from(1.0 / i as f32);
            ret += &term;
        }

        ret
    }

    /// Returns mutable reference of matrix component.
    ///
    /// # Panics
    ///
    /// Panics if `pos` is out of range.
    #[must_use]
    pub fn cell(&mut self, pos: Pos) -> Editor<'_, T> {
        assert!(MatPos(pos).is_in(self.size()));
        Editor::new(self, pos)
    }

    /// Cast matrix type.
    ///
    /// # Panics
    ///
    /// Panics if `self` and `other` size is different.
    #[must_use]
    pub fn cast<R2, C2>(self, mut other: Matrix<T, R2, C2>) -> Matrix<T, R2, C2> {
        assert_eq!(self.size(), other.size());

        let mut newtype = unsafe { mem::transmute(self) };
        mem::swap(&mut newtype, &mut other);
        other
    }

    /// Perform multiple operation and store its result to `out`.
    ///
    /// # Panics
    ///
    /// Panics if any of the following occurs.
    ///
    /// * `out` size missmatch to result size.
    /// * `self` columns count and `rhs` rows count do not match.
    pub fn mul_to<R2, C2>(&self, rhs: &Matrix<T, R2, C2>, out: &mut Matrix<T, R, C2>) {
        assert_eq!(self.cn(), rhs.rn());

        let use_sparse_calc = self.is_sparse() || rhs.is_sparse();
        let method = if use_sparse_calc {
            Matrix::mul_with_sparse_to
        } else {
            Matrix::mul_without_sparse_to
        };

        method(&self, rhs, out);
    }
}

impl<T, R, C> Clone for Matrix<T, R, C>
where
    T: Scalar,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            pd: self.pd.clone(),
        }
    }
}

impl<T, R, C> Debug for Matrix<T, R, C>
where
    T: Scalar,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Matrix")
            .field("data", &self.data)
            .finish()
    }
}

impl<T, R, C> PartialEq for Matrix<T, R, C>
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
        if self.size() != other.size() {
            return false;
        }

        if self.is_sparse() == other.is_sparse() {
            return self.data.eq(&other.data);
        }

        let sm = if self.is_sparse() { self } else { other };
        let dm = if self.is_sparse() { other } else { self };
        for mc in sm.nz_iter() {
            let sv = *mc.val();
            let dv = dm[mc.pos()];
            if sv != dv {
                return false;
            }
        }

        true
    }
}

impl<T, N> Index<usize> for Vector<T, N>
where
    T: Scalar,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.rn());
        self.data.value((index, 0))
    }
}

impl<T, R, C> Index<Pos> for Matrix<T, R, C>
where
    T: Scalar,
{
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        assert!(MatPos(index).is_in(self.size()));
        self.data.value(index)
    }
}

impl<T, R, C> AddAssign<&Self> for Matrix<T, R, C>
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

impl<T, R, C> Mul<T> for &Matrix<T, R, C>
where
    T: Scalar,
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut ret = DMatrix::make(self.size()).sparse(self.is_sparse()).build();
        for mc in self.nz_iter() {
            *ret.cell(mc.pos()) = *mc.val() * rhs;
        }

        unsafe { ret.mimic() }
    }
}

impl<T, R, C, R2, C2> Mul<&Matrix<T, R2, C2>> for &Matrix<T, R, C>
where
    T: Scalar,
{
    type Output = Matrix<T, R, C2>;

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
    fn mul(self, rhs: &Matrix<T, R2, C2>) -> Self::Output {
        assert_eq!(self.cn(), rhs.rn());
        let size = (self.rn(), rhs.cn());
        let sparse = self.is_sparse() && rhs.is_sparse();
        let ret = DMatrix::make(size).sparse(sparse).build();
        let mut ret = unsafe { ret.mimic() };
        self.mul_to(&rhs, &mut ret);
        ret
    }
}

impl<T, R, C> MulAssign<T> for Matrix<T, R, C>
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

impl<T, R, C> Matrix<T, R, C>
where
    T: Scalar,
{
    /// Returns mutable reference of internal data.
    pub(crate) fn mdata_mut(&mut self) -> &mut OwnStrage<T> {
        &mut self.data
    }

    /// Cast matrix strongly.
    pub(crate) unsafe fn mimic<R2, C2>(self) -> Matrix<T, R2, C2> {
        let mut newtype = unsafe { mem::transmute(self) };
        let mut other = Matrix::empty();
        mem::swap(&mut newtype, &mut other);
        other
    }

    /// Creates a new matrix with internal data.
    pub(crate) fn new_internal(data: OwnStrage<T>) -> Self {
        Self {
            data,
            pd: Default::default(),
        }
    }

    /// Creates a dummy empty matrix.
    fn empty() -> Self {
        Matrix {
            data: OwnStrage::Dense(OwnVecStrage::new((0, 0))),
            pd: Default::default(),
        }
    }

    /// Returns max scale norm.
    ///
    /// "Max scale norm" is coined term by author.
    ///
    /// This value is calculated by the following steps.
    ///
    /// 1. For each row, sum all components absolute values and get it max digits in binary.
    /// 2. Select the one with the largest absolute value of them.
    fn max_scale_nrom(&self) -> i32 {
        let rows = self.nz_iter().chunks(|x| x.row());
        let sums = rows.map(|r| r.map(|c| c.val().abs()).sum::<T::Real>());
        let digits = sums.map(|x| x.exponent());
        digits.max().unwrap_or(0)
    }

    /// Returns none-zero components iterator.
    fn nz_iter(&self) -> NzIter<'_, T> {
        self.data.nz_iter()
    }

    /// Perform add assign with dense matrix on the right-hand side.
    fn add_assign_for_dense_rhs(&mut self, rhs: &Self) {
        assert_eq!(self.size(), rhs.size());
        for i in 0..rhs.rn() {
            for j in 0..rhs.cn() {
                *self.cell((i, j)) += rhs[(i, j)];
            }
        }
    }

    /// Perform add assign with sparse matrix on the right-hand side.
    fn add_assign_for_sparse_rhs(&mut self, rhs: &Self) {
        assert_eq!(self.size(), rhs.size());
        for mc in rhs.nz_iter() {
            *self.cell(mc.pos()) += *mc.val();
        }
    }

    /// Perform multiple operation without sparse matrix.
    fn mul_without_sparse_to<R2, C2>(&self, rhs: &Matrix<T, R2, C2>, out: &mut Matrix<T, R, C2>) {
        assert_eq!(self.cn(), rhs.rn());
        assert_eq!((self.rn(), rhs.cn()), out.size());

        for i in 0..out.rn() {
            for j in 0..out.cn() {
                let mut sum = *T::zero();
                for k in 0..self.cn() {
                    sum += self[(i, k)] * rhs[(k, j)];
                }

                *out.cell((i, j)) = sum;
            }
        }
    }

    /// Perform multiple operation with sparse matrix.
    fn mul_with_sparse_to<R2, C2>(&self, rhs: &Matrix<T, R2, C2>, out: &mut Matrix<T, R, C2>) {
        assert_eq!(self.cn(), rhs.rn());
        assert_eq!((self.rn(), rhs.cn()), out.size());

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
                    *out.cell((lc.row(), rc.col())) += *lc.val() * *rc.val();
                }

                if rc.row() > lc.col() {
                    last_rc = Some(rc);
                    break;
                }
            }
        }
    }
}

impl<T, R, C, S> Matrix<T, R, C, S>
where
    T: Scalar,
    S: MatrixStrageMut<T>,
{
    /// Returns mutable none-zero components iterator.
    fn nz_iter_mut(&mut self) -> NzIterMut<'_, T> {
        self.data.nz_iter_mut()
    }
}
