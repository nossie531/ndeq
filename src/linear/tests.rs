#![cfg(test)]
use std::ops::Mul;
use super::Matrix;

#[test]
fn test() {
    let mut mx = Matrix::<f32>::new(3, 3, true);
    let mut my = Matrix::<f32>::new(3, 3, true);
    mx.set((0, 0), 2.0);
    mx.set((1, 1), 3.0);
    mx.set((2, 2), 4.0);
    my.set((0, 0), 2.0);
    my.set((1, 1), 3.0);
    my.set((2, 2), 4.0);
    
    let ret = mx.mul(my);
    assert_eq!(ret.m(), 3);
    assert_eq!(ret.n(), 3);
    assert_eq!(ret.get((0, 0)), 4.0);
}
