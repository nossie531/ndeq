//! Provider of [`Rmg`].

use crate::linalg::Matrix;
use crate::linalg::parts::{Pos, Size};
use rand::seq::index::sample;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::collections::BTreeSet;

/// Random matrix generator.
pub struct Sample {
    rng: Pcg32,
}

impl Sample {
    /// Standard vector length.
    pub const STD_LEN: usize = 3;

    /// Sample square matrix size.
    pub const SQ_SIZE: Size = (Self::STD_LEN, Self::STD_LEN);

    /// Sample row vector size.
    pub const ROW_VEC_SIZE: Size = (1, Self::STD_LEN);

    /// Sample column vector size.
    pub const COL_VEC_SIZE: Size = (Self::STD_LEN, 1);

    /// Creates a new value.
    pub fn new(seed: u64) -> Self {
        Self {
            rng: Pcg32::seed_from_u64(seed),
        }
    }

    /// Creates a standard row vector.
    pub fn create_row_vector(&mut self) -> Matrix<f32> {
        let mut ret = Matrix::new(Self::ROW_VEC_SIZE, false);
        for i in 0..ret.m() {
            ret.set((0, i), self.random_in_open01());
        }

        ret
    }

    /// Creates a standard column vector.
    pub fn create_col_vector(&mut self) -> Matrix<f32> {
        let mut ret = Matrix::new(Self::COL_VEC_SIZE, false);
        for i in 0..ret.m() {
            ret.set((i, 0), self.random_in_open01());
        }

        ret
    }

    /// Creates a standard square matrix.
    pub fn create_sq_matrix(&mut self, sparse: bool) -> Matrix<f32> {
        let mut ret = Matrix::new(Self::SQ_SIZE, sparse);
        let nnz_ratio = self.rng.random::<f32>();
        let zero_poss = self.random_shot(Self::SQ_SIZE, 1.0 - nnz_ratio);

        for i in 0..ret.m() {
            for j in 0..ret.n() {
                let is_zero = zero_poss.contains(&(i, j));
                let val = if is_zero {
                    0.0
                } else {
                    self.random_in_open01()
                };
                ret.set((i, j), val);
            }
        }

        ret
    }
}

impl Sample {
    /// Generates random set in two dimension index.
    fn random_shot(&mut self, size: Size, ratio: f32) -> BTreeSet<Pos> {
        let cmps_cnt = size.0 * size.1;
        let zero_cnt = (cmps_cnt as f32 * ratio).round() as usize;
        let zero_idxs = sample(&mut self.rng, cmps_cnt, zero_cnt);
        let zero_poss = zero_idxs.iter().map(|idx| (idx / size.1, idx % size.1));
        BTreeSet::from_iter(zero_poss)
    }

    /// Generates random value in open range 0 to 1.
    fn random_in_open01(&mut self) -> f32 {
        let rand_in_01 = self.rng.random();
        if rand_in_01 == 0.0 {
            f32::MIN_POSITIVE
        } else {
            rand_in_01
        }
    }
}
