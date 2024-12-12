mod net;
mod net_binder;
mod node;
mod plotters_util;
mod sample;

use plotters::coord::Shift;
use plotters::prelude::*;
use plotters_util::{PointMarker, SeriesHeader, C2};
use sample::Sample;
use std::error::Error;
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let mut sample = Sample::new();
    sample.run_simulation();

    let out = get_out_path()?;
    let root = SVGBackend::new(&*out, (600, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = create_chart(&root)?;
    let series_vec = sample.series_vec();
    for (i, series) in series_vec.iter().enumerate() {
        let label = format!("n{}", i + 1);
        chart.draw_series(vec![SeriesHeader::new(label, series[0])])?;
        chart.draw_series(LineSeries::new(series.clone(), &BLACK))?;
        chart.draw_series(series.iter().cloned().map(PointMarker::new))?;
    }

    Ok(())
}

fn get_out_path() -> std::io::Result<PathBuf> {
    let example_name = env!("CARGO_BIN_NAME");
    let path = format!("example_pkgs/{example_name}/out/out.svg");
    Ok(env::current_dir()?.join(path.as_str()))
}

fn create_chart<'a, DB>(
    area: &DrawingArea<DB, Shift>,
) -> Result<ChartContext<'a, DB, C2>, DrawingAreaErrorKind<DB::ErrorType>>
where
    DB: DrawingBackend,
{
    let mut builder = ChartBuilder::on(area);
    let mut ret = builder
        .margin(20)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(sample::T_RANGE, sample::V_RANGE)?;

    ret.configure_mesh()
        .axis_desc_style(("serif", 15))
        .x_desc("Time")
        .y_desc("Value")
        .light_line_style(&WHITE)
        .draw()?;

    Ok(ret)
}
