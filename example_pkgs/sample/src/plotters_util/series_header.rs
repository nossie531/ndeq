use plotters::element::{Drawable, PointCollection};
use plotters::style::IntoTextStyle;
use plotters_backend::{DrawingBackend, DrawingErrorKind, FontStyle};
use std::iter::{self, Once};

pub struct SeriesHeader {
    text: String,
    p: (f32, f32),
}

impl SeriesHeader {
    pub fn new(text: String, p: (f32, f32)) -> Self {
        Self { text, p }
    }
}

impl<'a> PointCollection<'a, (f32, f32)> for &'a SeriesHeader {
    type Point = &'a (f32, f32);
    type IntoIter = Once<&'a (f32, f32)>;
    fn point_iter(self) -> Self::IntoIter {
        iter::once(&self.p)
    }
}

impl<DB: DrawingBackend> Drawable<DB> for SeriesHeader {
    fn draw<I: Iterator<Item = (i32, i32)>>(
        &self,
        mut pos: I,
        backend: &mut DB,
        _: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        let tgt_pos = pos.next().unwrap();
        let adj_pos = (tgt_pos.0 + 10, tgt_pos.1 - 5);
        let font_prop = ("serif", 20, FontStyle::Bold);
        let text_style = font_prop.into_text_style(&backend.get_size());
        backend.draw_text(self.text.as_str(), &text_style, adj_pos)
    }
}
