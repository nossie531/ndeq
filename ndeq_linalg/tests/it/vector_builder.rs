use crate::for_tests::consts::*;
use ndeq_linalg::prelude::*;
use test_panic::prelude::*;

#[test]
fn build() {
    let target = DVector::<f32>::make(N);
    let result = target.build();
    assert!(!result.is_sparse());
    assert!(result.is_zero());
    assert_eq!(result.len(), N);
}

#[test]
fn values() {
    with_empty();
    with_values();
    with_overwrite();
    with_out_of_range();

    fn with_empty() {
        let target = DVector::<f32>::make(N);
        let result = target.values([]);
        let matrix = result.build();
        assert!(matrix.is_zero());
    }

    fn with_values() {
        let target = DVector::<f32>::make(N);
        let result = target.values([(0, 1.0), (N - 1, 1.0)]);
        let matrix = result.build();
        assert_eq!(matrix[0], 1.0);
        assert_eq!(matrix[N - 1], 1.0);
        assert_eq!(matrix[N / 2], 0.0);
    }

    fn with_overwrite() {
        let target = DVector::<f32>::make(N);
        let target = target.values([(0, 1.0)]);
        let result = test_panic(|| target.values([]));
        assert!(result.is_panic());
    }

    fn with_out_of_range() {
        let target = DVector::<f32>::make(N);
        let result = test_panic(|| target.values([(N, 1.0)]));
        assert!(result.is_panic());
    }
}
