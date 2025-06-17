use std::ops::{BitAnd, Shr, Sub};

/// Returns exponential part of floating number bits.
/// 
/// See [IEEE 754][1] for bit representations.
/// 
/// [1]: https://en.wikipedia.org/wiki/IEEE_754.
pub fn exponent<B, const D: u32>(bits: B) -> i32
where
    B: Sub<Output = B>
    + BitAnd<Output = B>
    + Shr<u32, Output = B>
    + From<u32>,
    i32: TryFrom<B>,
{
    let shift = const { size_of::<B>() as u32 * u8::BITS - D - 1 };
    let mask = const { !(!0 << D) };
    let bias = const { !(!0 << D) / 2 };
    let raw = bits >> shift & B::from(mask);
    i32::try_from(raw).ok().unwrap() - bias
}