//! Constant values for test.

/// Standard matrix edge length 1.
pub const M: usize = 4;

/// Standard matrix edge length 2.
pub const N: usize = 5;

/// Test scale.
///
/// ⚠️ Large value increase the number of tests.
/// So it reduce test leaks, but increase test time.
pub static TEST_SCALE: usize = 100;
