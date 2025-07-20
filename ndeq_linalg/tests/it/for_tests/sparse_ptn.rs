//! Sparse pattern calculations.

use crate::for_tests::consts;
use ndeq_linalg::aliases::Size;

/// Calculates the number of sparse patterns from matrix sizes.
///
/// First, the product of the area for each `sizes` is calculated, and then
/// the exponent is calculated to the power of 2. This is the ideal number
/// of test trials. But this becomes huge quickly. Therefore, the increase
/// gradient should be adjusted. We adjust increase gradient to 0.5 around
/// the value of [`consts::TEST_SCALE`].
pub fn calc<const N: usize>(sizes: [Size; N]) -> usize {
    let too_big = sizes.iter().map(area_pow).product();
    let scale = consts::TEST_SCALE as f32;
    adjust(too_big, scale).ceil() as usize
}

/// Returns 2^(`size.0 * size.1`).
fn area_pow(size: &Size) -> f32 {
    2usize.pow((size.0 * size.1) as u32) as f32
}

/// Returns adjusted value.
fn adjust(x: f32, half_gard_at: f32) -> f32 {
    (1.0 + x / half_gard_at).ln() * half_gard_at
}
