//! Provider of [`RF32`].

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Newtype of `f32` for operator right-hand side.
///
/// This type has a value of type [`f32`]. And some arithmetic operators can be
/// used with this type on the right-hand side. Here, the left-hand side can be
/// `f32` or some type with bigger range (`f32` and `f64` in the current stable
/// Rust 2025, `f128` in the nightly version may be added in the future). Then
/// of course, the result type is aligned to the left-hand side.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct RF32(pub f32);

macro_rules! def_add {
    ($ty:ty, $self:ident, $rhs:ident, $block:block) => {
        impl Add<RF32> for $ty {
            type Output = $ty;
            fn add($self, $rhs: RF32) -> Self::Output {
                $block
            }
        }

        impl Add<RF32> for &$ty {
            type Output = $ty;
            fn add($self, $rhs: RF32) -> Self::Output {
                $block
            }
        }

        impl Add<&RF32> for $ty {
            type Output = $ty;
            fn add($self, $rhs: &RF32) -> Self::Output {
                $block
            }
        }

        impl Add<&RF32> for &$ty {
            type Output = $ty;
            fn add($self, $rhs: &RF32) -> Self::Output {
                $block
            }
        }
    };
}

macro_rules! def_sub {
    ($ty:ty, $self:ident, $rhs:ident, $block:block) => {
        impl Sub<RF32> for $ty {
            type Output = $ty;
            fn sub($self, $rhs: RF32) -> Self::Output {
                $block
            }
        }

        impl Sub<RF32> for &$ty {
            type Output = $ty;
            fn sub($self, $rhs: RF32) -> Self::Output {
                $block
            }
        }

        impl Sub<&RF32> for $ty {
            type Output = $ty;
            fn sub($self, $rhs: &RF32) -> Self::Output {
                $block
            }
        }

        impl Sub<&RF32> for &$ty {
            type Output = $ty;
            fn sub($self, $rhs: &RF32) -> Self::Output {
                $block
            }
        }
    };
}

macro_rules! def_mul {
    ($ty:ty, $self:ident, $rhs:ident, $block:block) => {
        impl Mul<RF32> for $ty {
            type Output = $ty;
            fn mul($self, $rhs: RF32) -> Self::Output {
                $block
            }
        }

        impl Mul<RF32> for &$ty {
            type Output = $ty;
            fn mul($self, $rhs: RF32) -> Self::Output {
                $block
            }
        }

        impl Mul<&RF32> for $ty {
            type Output = $ty;
            fn mul($self, $rhs: &RF32) -> Self::Output {
                $block
            }
        }

        impl Mul<&RF32> for &$ty {
            type Output = $ty;
            fn mul($self, $rhs: &RF32) -> Self::Output {
                $block
            }
        }
    };
}

macro_rules! def_div {
    ($ty:ty, $self:ident, $rhs:ident, $block:block) => {
        impl Div<RF32> for $ty {
            type Output = $ty;
            fn div($self, $rhs: RF32) -> Self::Output {
                $block
            }
        }

        impl Div<RF32> for &$ty {
            type Output = $ty;
            fn div($self, $rhs: RF32) -> Self::Output {
                $block
            }
        }

        impl Div<&RF32> for $ty {
            type Output = $ty;
            fn div($self, $rhs: &RF32) -> Self::Output {
                $block
            }
        }

        impl Div<&RF32> for &$ty {
            type Output = $ty;
            fn div($self, $rhs: &RF32) -> Self::Output {
                $block
            }
        }
    };
}

macro_rules! def_add_assign {
    ($ty: ty, $self:ident, $rhs:ident, $block:block) => {
        impl AddAssign<RF32> for $ty {
            fn add_assign(&mut $self, $rhs: RF32) {
                $block
            }
        }

        impl AddAssign<&RF32> for $ty {
            fn add_assign(&mut $self, $rhs: &RF32) {
                $block
            }
        }
    };
}

macro_rules! def_sub_assign {
    ($ty: ty, $self:ident, $rhs:ident, $block:block) => {
        impl SubAssign<RF32> for $ty {
            fn sub_assign(&mut $self, $rhs: RF32) {
                $block
            }
        }

        impl SubAssign<&RF32> for $ty {
            fn sub_assign(&mut $self, $rhs: &RF32) {
                $block
            }
        }
    };
}

macro_rules! def_mul_assign {
    ($ty: ty, $self:ident, $rhs:ident, $block:block) => {
        impl MulAssign<RF32> for $ty {
            fn mul_assign(&mut $self, $rhs: RF32) {
                $block
            }
        }

        impl MulAssign<&RF32> for $ty {
            fn mul_assign(&mut $self, $rhs: &RF32) {
                $block
            }
        }
    };
}

macro_rules! def_div_assign {
    ($ty: ty, $self:ident, $rhs:ident, $block:block) => {
        impl DivAssign<RF32> for $ty {
            fn div_assign(&mut $self, $rhs: RF32) {
                $block
            }
        }

        impl DivAssign<&RF32> for $ty {
            fn div_assign(&mut $self, $rhs: &RF32) {
                $block
            }
        }
    };
}

def_add!(f32, self, rhs, { self + rhs.0 });
def_sub!(f32, self, rhs, { self - rhs.0 });
def_mul!(f32, self, rhs, { self * rhs.0 });
def_div!(f32, self, rhs, { self / rhs.0 });
def_add!(f64, self, rhs, { self + f64::from(rhs.0) });
def_sub!(f64, self, rhs, { self - f64::from(rhs.0) });
def_mul!(f64, self, rhs, { self * f64::from(rhs.0) });
def_div!(f64, self, rhs, { self / f64::from(rhs.0) });
def_add_assign!(f32, self, rhs, { *self += rhs.0 });
def_sub_assign!(f32, self, rhs, { *self -= rhs.0 });
def_mul_assign!(f32, self, rhs, { *self *= rhs.0 });
def_div_assign!(f32, self, rhs, { *self /= rhs.0 });
def_add_assign!(f64, self, rhs, { *self += f64::from(rhs.0) });
def_sub_assign!(f64, self, rhs, { *self -= f64::from(rhs.0) });
def_mul_assign!(f64, self, rhs, { *self *= f64::from(rhs.0) });
def_div_assign!(f64, self, rhs, { *self /= f64::from(rhs.0) });
