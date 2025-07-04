//! Test scale calculations.

use crate::for_tests::consts;
use ndeq_linalg::aliases::Size;

/// Calculates test scale from matrix sizes.
///
/// First, the product of the area for each `sizes` is calculated, and then
/// the exponent is calculated to the power of 2. Although this is the ideal
/// number of test trials, this quickly becomes huge. Therefore, its value is
/// calculated using logarithms to control the rate of increase. At this time,
/// the rate of increase is halved around the value of [`consts::TEST_SCALE`].
pub fn calc<const N: usize>(sizes: [Size; N]) -> usize {
    let raw_size = sizes.iter().map(area).product::<usize>();
    ln_adjusted(raw_size as f32, 1.0, consts::TEST_SCALE as f32).ceil() as usize
}

/// Calculates area from size.
fn area(size: &Size) -> usize {
    (2usize.pow((size.0 * size.1) as u32) as f32) as usize
}

/// Returns natural logarithm of `x` with some adjust.
fn ln_adjusted(x: f32, shift: f32, scale: f32) -> f32 {
    (shift + x / scale).ln() * scale
}
