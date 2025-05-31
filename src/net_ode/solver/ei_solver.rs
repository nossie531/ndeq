//! Provider of [`EiSolver`].

use crate::linear::Matrix;
use crate::net_ode::solver::NetOdeSolver;
use crate::ode::solver::OdeSolver;
use crate::ode::values::VArr;
use crate::parts::NdeqNet;

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
    fn create<'a>(&self, net: &'a dyn NdeqNet<V>) -> Box<dyn OdeSolver<'a, T, VArr<V>> + 'a> {
        todo!()
        // - net から [各ノード値のベクトル] と [ラプラシアン行列] を取得する。
        // - [各ノード値のベクトル] と [ラプラシアン行列] から行列指数関数を実行する。
    }
}
