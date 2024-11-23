use plotters::element::{Drawable, PointCollection};
use plotters::prelude::*;
use plotters_backend::{DrawingBackend, DrawingErrorKind};
use std::iter::{self, Once};

pub struct PointMarker {
    p: (f32, f32),
}

impl PointMarker {
    pub fn new(p: (f32, f32)) -> Self {
        Self { p }
    }
}

impl<'a> PointCollection<'a, (f32, f32)> for &'a PointMarker {
    type Point = &'a (f32, f32);
    type IntoIter = Once<&'a (f32, f32)>;
    fn point_iter(self) -> Self::IntoIter {
        iter::once(&self.p)
    }
}

impl<DB: DrawingBackend> Drawable<DB> for PointMarker {
    fn draw<I: Iterator<Item = (i32, i32)>>(
        &self,
        mut pos: I,
        backend: &mut DB,
        _: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        let tgt_pos = pos.next().unwrap();
        backend.draw_circle(tgt_pos, 3, &BLACK, true)
    }
}
