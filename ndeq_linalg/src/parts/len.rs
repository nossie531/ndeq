//! Matrix edge length.

/// Singe edge length.
pub struct Single();

/// Fixed matrix edge length.
pub struct Fixed<const N: usize>();

/// Variable matrix edge length.
pub struct Var();
