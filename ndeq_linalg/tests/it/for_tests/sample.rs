//! Provider of [`Sample`].

use crate::for_tests::consts;
use ndeq_linalg::aliases::{Pos, Size};
use ndeq_linalg::mat_util::MatSize;
use ndeq_linalg::prelude::*;
use rand::seq::index::sample;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::collections::BTreeSet;

/// Pseudo random sequence seed.
static SEED: u64 = 0;

/// Random matrix generator.
pub struct Sample {
    rng: Pcg32,
}

impl Sample {
    /// Sample square matrix size.
    pub const SQ_SIZE: Size = (consts::N, consts::N);

    /// Sample row vector size.
    pub const ROW_VEC_SIZE: Size = (1, consts::N);

    /// Sample column vector size.
    pub const COL_VEC_SIZE: Size = (consts::N, 1);

    /// Creates a new value.
    pub fn new() -> Self {
        Self {
            rng: Pcg32::seed_from_u64(SEED),
        }
    }

    /// Creates a standard row vector.
    pub fn create_row_vector(&mut self) -> DMatrix<f32> {
        let mut ret = DMatrix::new(Self::ROW_VEC_SIZE);
        for i in 0..ret.rn() {
            *ret.cell((0, i)) = self.random_in_open01();
        }

        ret
    }

    /// Creates a standard column vector.
    pub fn create_col_vector(&mut self) -> DMatrix<f32> {
        let mut ret = DMatrix::new(Self::COL_VEC_SIZE);
        for i in 0..ret.rn() {
            *ret.cell((i, 0)) = self.random_in_open01();
        }

        ret
    }

    /// Creates a standard square matrix.
    pub fn create_sq_matrix(&mut self, sparse: bool) -> DMatrix<f32> {
        let mut ret = DMatrix::make(Self::SQ_SIZE).sparse(sparse).build();
        let nnz_ratio = self.rng.random::<f32>();
        let zero_poss = self.random_shot(Self::SQ_SIZE, 1.0 - nnz_ratio);

        for i in 0..ret.rn() {
            for j in 0..ret.cn() {
                let is_zero = zero_poss.contains(&(i, j));
                *ret.cell((i, j)) = if is_zero {
                    0.0
                } else {
                    self.random_in_open01()
                };
            }
        }

        ret
    }
}

impl Sample {
    /// Generates random set in two dimension index.
    fn random_shot(&mut self, size: Size, ratio: f32) -> BTreeSet<Pos> {
        let cmps_cnt = MatSize(size).len();
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
