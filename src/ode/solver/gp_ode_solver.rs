use crate::ode::Slope;
use crate::ode::solver::OdeSolver;
use crate::ode::values::{Time, Value};
use std::ops::MulAssign;
use std::rc::Rc;

/// General purpose ODE solver.
pub trait GpOdeSolver<'a, T, V>: OdeSolver<'a, T, V>
where
    T: Time,
    V: Value + MulAssign<T>,
{
    /// Sets slope of this instance.
    fn set_slope(&mut self, value: Rc<Slope<'a, V>>);
}
