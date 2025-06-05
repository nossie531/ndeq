//! Type aliases.

use crate::Matrix;

/// Size of matrix.
pub type Size = (usize, usize);

/// Position in matrix.
pub type Pos = (usize, usize);

/// Single column or single row matrix.
pub type Vector<T> = Matrix<T>;
