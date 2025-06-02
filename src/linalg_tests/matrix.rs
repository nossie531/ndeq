use crate::linalg::Matrix;
use crate::linalg::parts::Size;
use crate::linalg_tests::sample::Sample;
use std::usize;
use test_panic::test_panic;

/// Pseudo random sequence seed.
static SEED: u64 = 0;

/// Test scale.
///
/// ⚠️ Small increase of this value causes large increase of test time.
static TEST_SCALE: f32 = 1.25;

#[test]
fn new() {
    let size = (3, 4);
    let sparse = false;
    let ret = Matrix::<f32>::new(size, false);

    assert_eq!(ret.size(), size);
    assert_eq!(ret.is_sparse(), sparse);
    for i in 0..ret.m() {
        for j in 0..ret.n() {
            assert_eq!(ret.get((i, j)), 0.0);
        }
    }
}

#[test]
fn get() {
    with_err_index();
    with_normal();

    fn with_err_index() {
        let target = Matrix::<f32>::new((3, 4), false);
        let result = test_panic(|| _ = target.get((2, 4)));

        assert!(result.is_panic());
    }

    fn with_normal() {
        let mut target = Matrix::<f32>::new((3, 4), false);
        let pos = (1, 2);
        let val = 3.0;
        target.set(pos, val);

        let ret = target.get(pos);
        assert_eq!(ret, val);
    }
}

#[test]
fn set() {
    with_err_index();
    with_normal();

    fn with_err_index() {
        let mut target = Matrix::<f32>::new((3, 4), false);
        let result = test_panic(|| _ = target.set((2, 4), 0.42));

        assert!(result.is_panic());
    }

    fn with_normal() {
        let mut target = Matrix::<f32>::new((3, 4), false);
        let pos = (1, 2);
        let val = 3.0;
        target.set(pos, val);

        let ret = target.get(pos);
        assert_eq!(ret, val);
    }
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
        let sample = &mut Sample::new(SEED);
        let target_x = sample.create_sq_matrix(false);
        let mut target_y = target_x.clone();
        target_y.set((0, 0), target_y.get((0, 0)) + 1.0);
        assert!(target_x != target_y);
    }

    fn with_normal_true() {
        let sample = &mut Sample::new(SEED);
        let target_x = sample.create_sq_matrix(false);
        let target_y = target_x.clone();
        assert!(target_x == target_y);
    }

    fn with_diff_strage_format() {
        let sample = &mut Sample::new(SEED);
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
        let sample = &mut Sample::new(SEED);
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
        let sample = &mut Sample::new(SEED);
        for _ in 0..calc_test_counts([Sample::SQ_SIZE]) {
            let lhs = sample.create_row_vector();
            let rhs = sample.create_sq_matrix(true);
            let ret = &lhs * &rhs;
            assert!(!ret.is_sparse());
            assert_eq!(ret, expected(&lhs, &rhs));
        }
    }

    fn with_sparse_vs_vec() {
        let sample = &mut Sample::new(SEED);
        for _ in 0..calc_test_counts([Sample::SQ_SIZE]) {
            let lhs = sample.create_sq_matrix(true);
            let rhs = sample.create_col_vector();
            let ret = &lhs * &rhs;
            assert!(!ret.is_sparse());
            assert_eq!(ret, expected(&lhs, &rhs));
        }
    }

    fn with_dense_vs_sparse() {
        let sample = &mut Sample::new(SEED);
        for _ in 0..calc_test_counts([Sample::SQ_SIZE]) {
            let lhs = sample.create_sq_matrix(false);
            let rhs = sample.create_sq_matrix(true);
            let ret = &lhs * &rhs;
            assert!(!ret.is_sparse());
            assert_eq!(ret, expected(&lhs, &rhs));
        }
    }

    fn with_sparse_vs_dense() {
        let sample = &mut Sample::new(SEED);
        for _ in 0..calc_test_counts([Sample::SQ_SIZE]) {
            let lhs = sample.create_sq_matrix(true);
            let rhs = sample.create_sq_matrix(false);
            let ret = &lhs * &rhs;
            assert!(!ret.is_sparse());
            assert_eq!(ret, expected(&lhs, &rhs));
        }
    }

    fn with_sparse_vs_sparse() {
        let sample = &mut Sample::new(SEED);
        for _ in 0..calc_test_counts([Sample::SQ_SIZE, Sample::SQ_SIZE]) {
            let lhs = sample.create_sq_matrix(true);
            let rhs = sample.create_sq_matrix(true);
            let ret = &lhs * &rhs;
            let expected = expected(&lhs, &rhs);
            assert!(ret.is_sparse());
            assert_eq!(ret, expected);
        }
    }

    fn expected(lhs: &Matrix<f32>, rhs: &Matrix<f32>) -> Matrix<f32> {
        let lhs = lhs.clone_sparse(false);
        let rhs = rhs.clone_sparse(false);
        &lhs * &rhs
    }
}

fn calc_test_counts<const N: usize>(sizes: [Size; N]) -> usize {
    sizes
        .iter()
        .fold(1, |acc, size| acc * calc_test_count(*size))
}

fn calc_test_count(size: Size) -> usize {
    (((size.0 * size.1) as f32).powi(2) * TEST_SCALE) as usize
}
