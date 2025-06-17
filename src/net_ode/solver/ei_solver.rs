//! Provider of [`EiSolver`].

use crate::net_ode::solver::NetOdeSolver;
use crate::parts::NdeqNet;
use ndeq_linalg::Matrix;
use ndeq_ode::solver::OdeSolver;
use ndeq_ode::values::VArr;

/// ODE solver for network with [Exponential Integrator].
///
/// [Exponential Integrator]: https://en.wikipedia.org/wiki/Exponential_integrator
pub struct EiSolver<T, V> {
    h: T,
    // Node values.
    vec: Matrix<V>,
    // Laplacian matrix.
    laplacian: Matrix<V>,
}

impl<T, V> NetOdeSolver<T, V> for EiSolver<T, V> {
    fn create<'a>(&self, net: &'a dyn NdeqNet<V>) -> Box<dyn OdeSolver<T, VArr<V>> + 'a> {
        todo!()
        // - net から [各ノード値のベクトル] と [ラプラシアン行列] を取得する。
        // - [各ノード値のベクトル] と [ラプラシアン行列] から行列指数関数を実行する。
    }
}
