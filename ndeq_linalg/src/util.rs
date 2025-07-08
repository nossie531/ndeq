//! Crate's utility.

use crate::aliases::Size;

/// Returns whether sparse matrix format is recommended.
///
/// If `nnz` (The Number of Non Zero) is much small than `size` (Matrix size),
/// sparse storage format is suggested. However, if `size` is too small, the
/// trend is lost and dense matrix format is suggested. This is because the
/// sparse matrix format has large memory overhead and poor memory locality.
#[must_use]
pub fn is_sparse_prefered(nnz: usize, size: Size) -> bool {
    static OFFSET: usize = 1024;
    static DENSITY_LIMIT: f32 = 0.1;
    let len = size.0 * size.1;
    let ntr = (nnz + OFFSET) as f32;
    let dtr = (len + OFFSET) as f32;
    ntr / dtr < DENSITY_LIMIT
}