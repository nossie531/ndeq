//! Provider of [`NodeVec`]

use std::borrow::{Borrow, BorrowMut};
use std::mem;
use std::ops::{AddAssign, DivAssign, Index, IndexMut, MulAssign, SubAssign};
use ensure_impl::prelude::*;
use ndeq_ode::values::OdeValue;

/// Node values vector.
#[repr(transparent)]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct NodeVec<T>(Vec<T>);

#[ensure_impl]
impl<T> OdeValue for NodeVec<T>
where 
    T: 'static,
    T: Clone,
    T: Default,
    T: PartialEq,
    T: MulAssign<f32>,
    T: for<'a> AddAssign<&'a T>,
    T: for<'a> SubAssign<&'a T>,
{
}

impl<T> NodeVec<T> {
    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if `self` has no elements.
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }
}

impl<T> AsRef<Vec<T>> for NodeVec<T> {
    fn as_ref(&self) -> &Vec<T> {
        self.borrow()
    }
}

impl<T> AsMut<Vec<T>> for NodeVec<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        self.borrow_mut()
    }
}

impl<T> Borrow<Vec<T>> for NodeVec<T> {
    fn borrow(&self) -> &Vec<T> {
        unsafe { mem::transmute(self) }
    }
}

impl<T> BorrowMut<Vec<T>> for NodeVec<T> {
    fn borrow_mut(&mut self) -> &mut Vec<T> {
        unsafe { mem::transmute(self) }
    }
}

impl<T> From<&Vec<T>> for &NodeVec<T> {
    fn from(value: &Vec<T>) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl<T> Index<usize> for NodeVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T> IndexMut<usize> for NodeVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<T, U> MulAssign<U> for NodeVec<T>
where
    T: MulAssign<U>,
    U: Copy,
{
    fn mul_assign(&mut self, rhs: U) {
        for i in 0..self.len() {
            self.0[i] *= rhs;
        }
    }
}

impl<T, U> DivAssign<U> for NodeVec<T>
where
    T: DivAssign<U>,
    U: Copy,
{
    fn div_assign(&mut self, rhs: U) {
        for i in 0..self.len() {
            self.0[i] /= rhs;
        }
    }
}

impl<'a, T> AddAssign<&'a Self> for NodeVec<T>
where
    T: AddAssign<&'a T>,
{
    fn add_assign(&mut self, rhs: &'a Self) {
        assert_eq!(self.len(), rhs.len(), "{}", msg::SIZE_MISSMATCH);
        for i in 0..self.len() {
            self.0[i] += &rhs.0[i];
        }
    }
}

impl<'a, T> SubAssign<&'a Self> for NodeVec<T>
where
    T: SubAssign<&'a T>,
{
    fn sub_assign(&mut self, rhs: &'a Self) {
        assert_eq!(self.len(), rhs.len(), "{}", msg::SIZE_MISSMATCH);
        for i in 0..self.len() {
            self.0[i] -= &rhs.0[i];
        }
    }
}

mod msg {
    pub const SIZE_MISSMATCH: &str = "Left and right size missmatch.";
}
