//! Provider of [`SMatrix`].

use std::ops::{Add, AddAssign, Mul};

use super::{
    mds::MData, Scalar
};

/// [Matrix].
///
/// [Matrix]: https://en.wikipedia.org/wiki/Matrix_(mathematics)
#[derive(Clone)]
pub struct Matrix<T> {
    size: (usize, usize),
    data: MData<T>,
}

impl<T> Matrix<T>
where
    T: Scalar,
{
    pub fn new(m: usize, n: usize, sparse: bool) -> Self {
        let size = (m, n);
        let data = MData::new(m, n, sparse);
        Self { size, data }
    }

    pub fn is_sparse(&self) -> bool {
        self.data.is_sparse()
    }

    pub fn m(&self) -> usize {
        self.size.0
    }

    pub fn n(&self) -> usize {
        self.size.1
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn get(&self, pos: (usize, usize)) -> T {
        self.data.get(self.size, pos)
    }

    pub fn set(&mut self, pos: (usize, usize), value: T) {
        self.data.set(self.size, pos, value);
    }
}

impl<T> AddAssign for Matrix<T>
where
    T: Scalar,
{
    /// Performs the `+=` operation.
    ///
    /// # Panics
    ///
    /// Panics if both operands size are not same.
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.size(), rhs.size());

        let method = if rhs.is_sparse() {
            Self::add_assign_for_sparse_rhs
        } else {
            Self::add_assign_for_dense_rhs
        };

        method(self, rhs);
    }
}

impl<T> Mul for Matrix<T>
where
    T: Scalar,
{
    type Output = Self;

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
            Self::mul_with_sparse
        } else {
            Self::mul_without_sparse
        };

        let ret = method(&self, rhs);
        assert_eq!(ret.is_sparse(), return_sparse_fmt);
        ret
    }
}

impl<T> PartialEq for Matrix<T>
where 
    T: Scalar
{
    /// Tests for `self` and `other` values to be equal, and is used by `==`.
    /// 
    /// # Comparison rule
    /// 
    /// Comparisons are based only on whether the values of all components match
    /// ([`Self::sparse`] does not affect the result).
    fn eq(&self, other: &Self) -> bool {
        let method = if self.is_sparse() == other.is_sparse() {
            Self::eq_for_homo
        } else {
            Self::eq_for_hetelo
        };

        method(&self, other)
    }
}

impl<T> Matrix<T>
where
    T: Scalar,
{
    fn eq_for_homo(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        self.data.eq(&other.data)
    }

    fn eq_for_hetelo(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        let sm = if self.is_sparse() { self } else { other };
        let dm = if self.is_sparse() { other } else { self };
        for mc in sm.data.none_zeros(self.size()) {
            let sv = mc.val();
            let dv = dm.get(mc.pos());
            if sv != dv {
                return false;
            }
        }

        true
    }

    fn add_assign_for_dense_rhs(&mut self, rhs: Self)  {
        for i in 0..rhs.m() {
            for j in 0..rhs.n() {
                let curr = self.get((i, j));
                let addition = rhs.get((i, j));
                self.set((i, j), curr + addition);
            }
        }
    }

    fn add_assign_for_sparse_rhs(&mut self, rhs: Self) {
        for mc in rhs.data.none_zeros(self.size()) {
            let curr = self.get(mc.pos());
            let addition = mc.val();
            self.set(mc.pos(), curr + addition)
        }
    }

    fn mul_without_sparse(&self, rhs: Self) -> Self {
        let ret_for_sparse = self.data.is_sparse() && rhs.data.is_sparse();
        let mut ret = Matrix::<T>::new(self.m(), rhs.n(), ret_for_sparse);

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

    fn mul_with_sparse(&self, rhs: Self) -> Self {
        let ret_for_sparse = self.data.is_sparse() && rhs.data.is_sparse();
        let size = self.size();
        let mut ret = Matrix::<T>::new(self.m(), rhs.n(), ret_for_sparse);
        let mut l_iter = self.data.none_zeros(size);
        let mut r_iter = rhs.data.none_zeros(size);
        let mut l_cell = l_iter.next();
        let mut last_row = None as Option<usize>;

        while l_cell.is_some() {
            let l_row = l_cell.as_ref().unwrap().row();
            let l_val = l_cell.as_ref().unwrap().val();
            if Some(l_row) != last_row {
                r_iter = rhs.data.none_zeros(size);
            }

            let mut r_cell = r_iter.next();
            while r_cell.is_some() && r_cell.as_ref().unwrap().row() == l_row {
                let r_col = r_cell.as_ref().unwrap().col();
                let r_val = r_cell.as_ref().unwrap().val();
                let curr = ret.get((l_row, r_col));
                ret.set((l_row, r_col), curr + l_val * r_val);
                r_cell = r_iter.next();
            }

            l_cell = l_iter.next();
            last_row = Some(l_row);
        }

        ret
    }
}