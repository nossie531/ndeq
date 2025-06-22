//! Crate's utility.

use std::convert::TryFrom;
use std::ops::{BitAnd, Shr, Sub};

/// Returns size of a type in bits.
pub const fn bit_size<T>() -> usize {
    size_of::<T>() * u8::BITS as usize
}

/// Returns exponential part of floating number bits.
///
/// See [IEEE 754][1] for bit representations.
///
/// [1]: https://en.wikipedia.org/wiki/IEEE_754.
pub fn exponent<B, const M: u32>(bits: B) -> i32
where
    B: Sub<Output = B> + BitAnd<Output = B> + Shr<u32, Output = B> + From<u32>,
    i32: TryFrom<B>,
{
    let mask = const { !(!0 << bit_size::<B>() as u32 - M) };
    let bias = const { !(!0 << bit_size::<B>() as u32 - M) / 2 };
    let raw = bits >> M - 1 & B::from(mask);
    i32::try_from(raw).ok().unwrap() - bias
}
