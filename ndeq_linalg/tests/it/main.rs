mod for_tests;

use for_tests::{Sample, scale};
use ndeq_linalg::parts::Scalar;
use ndeq_linalg::prelude::*;
use test_panic::prelude::*;

#[test]
fn new() {
    let size = (3, 4);
    let sparse = false;
    let result = Matrix::<f32>::new(size, false);

    assert_eq!(result.size(), size);
    assert_eq!(result.is_sparse(), sparse);
    for i in 0..result.m() {
        for j in 0..result.n() {
            assert_eq!(result[(i, j)], 0.0);
        }
    }
}

#[test]
fn identity() {
    let result = Matrix::<f32>::identity(5);
    for i in 0..result.m() {
        for j in 0..result.n() {
            let val = if i == j { 1.0 } else { 0.0 };
            assert_eq!(result[(i, j)], val);
        }
    }
}

#[test]
fn get() {
    with_err_index();
    with_normal();

    fn with_err_index() {
        let target = Matrix::<f32>::new((3, 4), false);
        let result = test_panic(|| _ = target[(2, 4)]);

        assert!(result.is_panic());
    }

    fn with_normal() {
        let mut target = Matrix::<f32>::new((3, 4), false);
        let pos = (1, 2);
        let val = 3.0;
        *target.cell(pos) = val;

        let result = target[pos];
        assert_eq!(result, val);
    }
}

#[test]
fn cell() {
    with_err_index();
    with_normal();

    fn with_err_index() {
        let mut target = Matrix::<f32>::new((3, 4), false);
        let result = test_panic(|| *target.cell((2, 4)) = 0.42);

        assert!(result.is_panic());
    }

    fn with_normal() {
        let mut target = Matrix::<f32>::new((3, 4), false);
        let pos = (1, 2);
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
        let target_x = Matrix::<f32>::new((3, 4), false);
        let target_y = Matrix::<f32>::new((4, 3), false);
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
        let target_y = target_x.clone_sparse(true);
        assert!(target_x == target_y);
    }
}

#[test]
fn add_assign() {
    with_err_size();
    with_sparse_rhs();

    fn with_err_size() {
        let mut lhs = Matrix::<f32>::new((3, 4), false);
        let rhs = Matrix::<f32>::new((3, 3), false);
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

    fn expected(lhs: Matrix<f32>, rhs: &Matrix<f32>) -> Matrix<f32> {
        let mut lhs = lhs.clone();
        let rhs = rhs.clone_sparse(false);
        lhs += &rhs;
        lhs
    }
}

#[test]
fn mul() {
    with_err_size();
    with_vec_vs_sparse();
    with_sparse_vs_vec();
    with_dense_vs_sparse();
    with_sparse_vs_sparse();
    with_sparse_vs_dense();

    fn with_err_size() {
        let lhs = Matrix::<f32>::new((3, 4), false);
        let rhs = Matrix::<f32>::new((3, 3), false);
        let result = test_panic(|| _ = &lhs * &rhs);
        assert!(result.is_panic());
    }

    fn with_vec_vs_sparse() {
        let sample = &mut Sample::new();
        for _ in 0..scale::calc([Sample::SQ_SIZE]) {
            let lhs = sample.create_row_vector();
            let rhs = sample.create_sq_matrix(true);
            let result = &lhs * &rhs;
            assert!(!result.is_sparse());
            assert_eq!(result, expected(&lhs, &rhs));
        }
    }

    fn with_sparse_vs_vec() {
        let sample = &mut Sample::new();
        for _ in 0..scale::calc([Sample::SQ_SIZE]) {
            let lhs = sample.create_sq_matrix(true);
            let rhs = sample.create_col_vector();
            let result = &lhs * &rhs;
            assert!(!result.is_sparse());
            assert_eq!(result, expected(&lhs, &rhs));
        }
    }

    fn with_dense_vs_sparse() {
        let sample = &mut Sample::new();
        for _ in 0..scale::calc([Sample::SQ_SIZE]) {
            let lhs = sample.create_sq_matrix(false);
            let rhs = sample.create_sq_matrix(true);
            let result = &lhs * &rhs;
            assert!(!result.is_sparse());
            assert_eq!(result, expected(&lhs, &rhs));
        }
    }

    fn with_sparse_vs_dense() {
        let sample = &mut Sample::new();
        for _ in 0..scale::calc([Sample::SQ_SIZE]) {
            let lhs = sample.create_sq_matrix(true);
            let rhs = sample.create_sq_matrix(false);
            let result = &lhs * &rhs;
            assert!(!result.is_sparse());
            assert_eq!(result, expected(&lhs, &rhs));
        }
    }

    fn with_sparse_vs_sparse() {
        let sample = &mut Sample::new();
        for _ in 0..scale::calc([Sample::SQ_SIZE, Sample::SQ_SIZE]) {
            let lhs = sample.create_sq_matrix(true);
            let rhs = sample.create_sq_matrix(true);
            let result = &lhs * &rhs;
            assert!(result.is_sparse());
            assert_eq!(result, expected(&lhs, &rhs));
        }
    }

    fn expected(lhs: &Matrix<f32>, rhs: &Matrix<f32>) -> Matrix<f32> {
        let lhs = lhs.clone_sparse(false);
        let rhs = rhs.clone_sparse(false);
        &lhs * &rhs
    }
}

#[test]
fn mul_scalar() {
    with_dense();
    with_sparse();

    fn with_dense() {
        let sample = &mut Sample::new();
        let target = &sample.create_sq_matrix(false);
        let rhs = 2.0;
        let result = target * rhs;
        assert_eq!(result, expected(target, rhs))
    }

    fn with_sparse() {
        let sample = &mut Sample::new();
        let target = &sample.create_sq_matrix(true);
        let rhs = 2.0;
        let result = target * rhs;
        assert_eq!(result, expected(target, rhs))
    }

    fn expected<T: Scalar>(lhs: &Matrix<T>, rhs: T) -> Matrix<T> {
        let mut ret = Matrix::new(lhs.size(), false);
        for i in 0..ret.m() {
            for j in 0..ret.n() {
                *ret.cell((i, j)) = lhs[(i, j)] * rhs;
            }
        }

        ret
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
