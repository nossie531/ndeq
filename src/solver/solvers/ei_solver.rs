//! Provider of [`EiSolver`].

use crate::prelude::*;
use ndeq_linalg::prelude::*;

/// ODE solver for network with [Exponential Integrator].
///
/// [Exponential Integrator]: https://en.wikipedia.org/wiki/Exponential_integrator
pub struct EiSolver<T, V> {
    h: T,
    // Node values.
    vec: DMatrix<V>,
    // Laplacian matrix.
    laplacian: DMatrix<V>,
}

impl<'a, T, V> NetOdeSolver<'a, T, V> for EiSolver<T, V> {
    // - net から [各ノード値のベクトル] と [ラプラシアン行列] を取得する。
    // - [各ノード値のベクトル] と [ラプラシアン行列] から行列指数関数を実行する。
    fn new_values(&self) -> &[V] {
        todo!()
    }

    fn set_flow(&mut self, value: &'a dyn NetFlow<V>) {
        todo!()
    }

    fn run(&mut self, t: T) {
        todo!()
    }
}
