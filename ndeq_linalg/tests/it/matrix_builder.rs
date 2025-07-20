use crate::for_tests::consts::*;
use ndeq_linalg::prelude::*;
use test_panic::prelude::*;

#[test]
fn build() {
    let target = DMatrix::<f32>::make((M, N));
    let result = target.build();
    assert!(!result.is_sparse());
    assert!(result.is_zero());
    assert_eq!(result.size(), (M, N));
}

#[test]
fn sparse() {
    with_false();
    with_true();
    with_data();
    with_overwrite();

    fn with_false() {
        let target = DMatrix::<f32>::make((M, N));
        let result = target.sparse(false);
        let matrix = result.build();
        assert!(!matrix.is_sparse());
        assert!(matrix.is_zero());
        assert_eq!(matrix.size(), (M, N));
    }

    fn with_true() {
        let target = DMatrix::<f32>::make((M, N));
        let result = target.sparse(true);
        let matrix = result.build();
        assert!(matrix.is_sparse());
        assert!(matrix.is_zero());
        assert_eq!(matrix.size(), (M, N));
    }

    fn with_overwrite() {
        let target = DMatrix::<f32>::make((M, N));
        let target = target.sparse(true);
        let result = target.sparse(false);
        let matrix = result.build();
        assert!(!matrix.is_sparse());
        assert!(matrix.is_zero());
        assert_eq!(matrix.size(), (M, N));
    }

    fn with_data() {
        let target = DMatrix::<f32>::make((M, N));
        let target = target.entries(vec![((0, 0), 1.0)]);
        let result = test_panic(|| target.sparse(false));
        assert!(result.is_panic());
    }
}

#[test]
fn entries() {
    with_empty();
    with_values();
    with_overwrite();
    with_out_of_range();

    fn with_empty() {
        let target = DMatrix::<f32>::make((M, N));
        let result = target.entries([]);
        let matrix = result.build();
        assert!(matrix.is_zero());
    }

    fn with_values() {
        let target = DMatrix::<f32>::make((M, N));
        let result = target.entries([((0, 0), 1.0), ((M - 1, N - 1), 1.0)]);
        let matrix = result.build();
        assert_eq!(matrix[(0, 0)], 1.0);
        assert_eq!(matrix[(M - 1, N - 1)], 1.0);
        assert_eq!(matrix[(M / 2, N / 2)], 0.0);
    }

    fn with_overwrite() {
        let target = DMatrix::<f32>::make((M, N));
        let target = target.entries([((0, 0), 1.0)]);
        let result = test_panic(|| target.entries([]));
        assert!(result.is_panic());
    }

    fn with_out_of_range() {
        let target = DMatrix::<f32>::make((M, N));
        let result = test_panic(|| target.entries([((M, N), 1.0)]));
        assert!(result.is_panic());
    }
}
