use ndeq_num::prelude::*;
use std::ops::MulAssign;

/// Node value.
pub trait NodeVal<T>: Float + MulAssign<f32> + MulAssign<T> {}

impl<X, T> NodeVal<T> for X where X: Float + MulAssign<f32> + MulAssign<T> {}
