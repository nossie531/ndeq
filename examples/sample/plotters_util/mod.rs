mod point_marker;
mod series_header;

pub use point_marker::*;
pub use series_header::*;

use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;

pub type C2 = Cartesian2d<RangedCoordf32, RangedCoordf32>;
