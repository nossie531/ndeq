mod for_tests;

use for_tests::{Sample, sparse_ptn};
use ndeq_linalg::parts::Scalar;
use ndeq_linalg::prelude::*;
use ndeq_linalg::util::mutil;
use test_panic::prelude::*;

const M: usize = 4;
const N: usize = 5;

#[test]
fn new() {
    with_svector();
    with_dvector();
    with_smatrix();
    with_dmatrix();

    fn with_svector() {
        let result = SVector::<f32, N>::new();
        assert_eq!(result.size(), (N, 1));
        assert_eq!(result.is_sparse(), false);
        assert!(result.is_zero());
    }

    fn with_dvector() {
        let result = DVector::<f32>::new(N);
        assert_eq!(result.size(), (N, 1));
        assert_eq!(result.is_sparse(), false);
        assert!(result.is_zero());
    }

    fn with_smatrix() {
        let result = SMatrix::<f32, M, N>::new();
        assert_eq!(result.size(), (M, N));
        assert_eq!(result.is_sparse(), false);
        assert!(result.is_zero());
    }

    fn with_dmatrix() {
        let result = DMatrix::<f32>::new((M, N));
        assert_eq!(result.size(), (M, N));
        assert_eq!(result.is_sparse(), false);
        assert!(result.is_zero());
    }
}

#[test]
fn sparse() {
    with_smatrix();
    with_dmatrix();

    fn with_smatrix() {
        let result = SMatrix::<f32, M, N>::sparse();
        assert_eq!(result.size(), (M, N));
        assert_eq!(result.is_sparse(), true);
        assert!(result.is_zero());
    }

    fn with_dmatrix() {
        let result = DMatrix::<f32>::sparse((M, N));
        assert_eq!(result.size(), (M, N));
        assert_eq!(result.is_sparse(), true);
        assert!(result.is_zero());
    }
}

#[test]
fn identity() {
    with_smatrix();
    with_dmatrix();

    fn with_smatrix() {
        let result = SMatrix::<f32, N, N>::identity();
        assert_eq!(result.size(), (N, N));
        assert_eq!(result.is_sparse(), mutil::is_sparse_prefered(N, (N, N)));
        assert!(result.is_identity());
    }

    fn with_dmatrix() {
        let result = DMatrix::<f32>::identity(N);
        assert_eq!(result.size(), (N, N));
        assert_eq!(result.is_sparse(), mutil::is_sparse_prefered(N, (N, N)));
        assert!(result.is_identity());
    }
}

#[test]
fn len() {
    let target = DMatrix::<f32>::new((M, N));
    assert_eq!(target.len(), M * N);
}

#[test]
fn index() {
    with_err_index();
    with_normal();
    with_vector();

    fn with_err_index() {
        let target = DMatrix::<f32>::new((M, N));
        let result = test_panic(|| _ = target[(M / 2, N)]);
        assert!(result.is_panic());
    }

    fn with_normal() {
        let pos = (M / 2, N / 2);
        let val = 3.0;
        let target = DMatrix::<f32>::make((M, N)).entries([(pos, val)]).build();
        let result = target[pos];
        assert_eq!(result, val);
    }

    fn with_vector() {
        let pos = M / 2;
        let val = 3.0;
        let target = DVector::make(N).entries([(pos, val)]).build();
        let result = target[pos];
        assert_eq!(result, val);
    }
}

#[test]
fn val() {
    with_err_index();
    with_normal();

    fn with_err_index() {
        let mut target = DVector::<f32>::new(N);
        let result = test_panic(|| *target.val(N) = 0.42);
        assert!(result.is_panic());
    }

    fn with_normal() {
        let mut target = DVector::<f32>::new(N);
        let index = N / 2;
        let val = 3.0;
        *target.val(index) = val;
        assert_eq!(target[index], val);
    }
}

#[test]
fn cell() {
    with_err_index();
    with_normal();

    fn with_err_index() {
        let mut target = DMatrix::<f32>::new((M, N));
        let result = test_panic(|| *target.cell((M / 2, N)) = 0.42);
        assert!(result.is_panic());
    }

    fn with_normal() {
        let mut target = DMatrix::<f32>::new((M, N));
        let pos = (M / 2, N / 2);
        let val = 3.0;
        *target.cell(pos) = val;
        assert_eq!(target[pos], val);
    }
}

#[test]
fn exp_mul_f32() {
    with_err_size();

    fn with_err_size() {}
}

#[test]
fn exp_mul_f64() {
    // todo!();
}

#[test]
fn eq() {
    with_diff_size();
    with_normal_false();
    with_normal_true();
    with_diff_strage_format();

    fn with_diff_size() {
        let target_x = DMatrix::<f32>::new((M, N));
        let target_y = DMatrix::<f32>::new((N, M));
        assert!(target_x != target_y);
    }

    fn with_normal_false() {
        let sample = &mut Sample::new();
        let target_x = sample.create_sq_matrix(false);
        let mut target_y = target_x.clone();
        *target_y.cell((0, 0)) += 1.0;
        assert!(target_x != target_y);
    }

    fn with_normal_true() {
        let sample = &mut Sample::new();
        let target_x = sample.create_sq_matrix(false);
        let target_y = target_x.clone();
        assert!(target_x == target_y);
    }

    fn with_diff_strage_format() {
        let sample = &mut Sample::new();
        let target_x = sample.create_sq_matrix(false);
        let target_y = target_x.clone_with_sparse(true);
        assert!(target_x == target_y);
    }
}

#[test]
fn add_assign() {
    with_diff_size();
    with_sparse_rhs();

    fn with_diff_size() {
        let mut lhs = DMatrix::<f32>::new((M, N));
        let rhs = DMatrix::<f32>::new((N, M));
        let result = test_panic(|| lhs += &rhs);
        assert!(result.is_panic());
    }

    fn with_sparse_rhs() {
        let sample = &mut Sample::new();
        let lhs = sample.create_sq_matrix(false);
        let rhs = sample.create_sq_matrix(true);
        let mut target = lhs.clone();
        target += &rhs;
        assert_eq!(target, expected(lhs, &rhs));
    }

    fn expected(lhs: DMatrix<f32>, rhs: &DMatrix<f32>) -> DMatrix<f32> {
        let mut lhs = lhs.clone();
        let rhs = rhs.clone_with_sparse(false);
        lhs += &rhs;
        lhs
    }
}

#[test]
fn mul() {
    with_err_size();
    with_scalar_vs_dense();
    with_scalar_vs_sparse();
    with_vec_vs_sparse();
    with_sparse_vs_vec();
    with_dense_vs_sparse();
    with_sparse_vs_sparse();
    with_sparse_vs_dense();

    fn with_err_size() {
        let lhs = DMatrix::<f32>::new((M, N));
        let rhs = DMatrix::<f32>::new((M, M));
        let result = test_panic(|| _ = &lhs * &rhs);
        assert!(result.is_panic());
    }

    fn with_scalar_vs_dense() {
        let sample = &mut Sample::new();
        let target = &sample.create_sq_matrix(false);
        let rhs = 2.0;
        let result = target * rhs;
        assert_eq!(result, expected_vs_scalar(target, rhs));
    }

    fn with_scalar_vs_sparse() {
        let sample = &mut Sample::new();
        let target = &sample.create_sq_matrix(true);
        let rhs = 2.0;
        let result = target * rhs;
        assert_eq!(result, expected_vs_scalar(target, rhs));
    }

    fn with_vec_vs_sparse() {
        let sample = &mut Sample::new();
        for _ in 0..sparse_ptn::calc([Sample::SQ_SIZE]) {
            let lhs = sample.create_row_vector();
            let rhs = sample.create_sq_matrix(true);
            let result = &lhs * &rhs;
            assert!(!result.is_sparse());
            assert_eq!(result, expected_vs_matrix(&lhs, &rhs));
        }
    }

    fn with_sparse_vs_vec() {
        let sample = &mut Sample::new();
        for _ in 0..sparse_ptn::calc([Sample::SQ_SIZE]) {
            let lhs = sample.create_sq_matrix(true);
            let rhs = sample.create_col_vector();
            let result = &lhs * &rhs;
            assert!(!result.is_sparse());
            assert_eq!(result, expected_vs_matrix(&lhs, &rhs));
        }
    }

    fn with_dense_vs_sparse() {
        let sample = &mut Sample::new();
        for _ in 0..sparse_ptn::calc([Sample::SQ_SIZE]) {
            let lhs = sample.create_sq_matrix(false);
            let rhs = sample.create_sq_matrix(true);
            let result = &lhs * &rhs;
            assert!(!result.is_sparse());
            assert_eq!(result, expected_vs_matrix(&lhs, &rhs));
        }
    }

    fn with_sparse_vs_dense() {
        let sample = &mut Sample::new();
        for _ in 0..sparse_ptn::calc([Sample::SQ_SIZE]) {
            let lhs = sample.create_sq_matrix(true);
            let rhs = sample.create_sq_matrix(false);
            let result = &lhs * &rhs;
            assert!(!result.is_sparse());
            assert_eq!(result, expected_vs_matrix(&lhs, &rhs));
        }
    }

    fn with_sparse_vs_sparse() {
        let sample = &mut Sample::new();
        for _ in 0..sparse_ptn::calc([Sample::SQ_SIZE, Sample::SQ_SIZE]) {
            let lhs = sample.create_sq_matrix(true);
            let rhs = sample.create_sq_matrix(true);
            let result = &lhs * &rhs;
            assert!(result.is_sparse());
            assert_eq!(result, expected_vs_matrix(&lhs, &rhs));
        }
    }

    fn expected_vs_scalar<T: Scalar>(lhs: &DMatrix<T>, rhs: T) -> DMatrix<T> {
        let mut ret = DMatrix::new(lhs.size());
        for i in 0..ret.m() {
            for j in 0..ret.n() {
                *ret.cell((i, j)) = lhs[(i, j)] * rhs;
            }
        }

        ret
    }

    fn expected_vs_matrix(lhs: &DMatrix<f32>, rhs: &DMatrix<f32>) -> DMatrix<f32> {
        let lhs = lhs.clone_with_sparse(false);
        let rhs = rhs.clone_with_sparse(false);
        &lhs * &rhs
    }
}

#[test]
fn mul_assign() {
    with_dense();
    with_sparse();

    fn with_dense() {
        let sample = &mut Sample::new();
        let target = &mut sample.create_sq_matrix(false);
        let original = &target.clone();
        *target *= 2.0;
        assert_eq!(target, &(original * 2.0))
    }

    fn with_sparse() {
        let sample = &mut Sample::new();
        let target = &mut sample.create_sq_matrix(true);
        let original = &target.clone();
        *target *= 2.0;
        assert_eq!(target, &(original * 2.0))
    }
}
