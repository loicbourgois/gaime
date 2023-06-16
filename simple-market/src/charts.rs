use crate::Config;
use plotters::prelude::full_palette::GREY;
use plotters::prelude::*;

pub fn write_chart_1(
    c: &Config,
    data: &[Vec<f64>],
    best_score: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let dim = 1024;
    let root = BitMapBackend::new("latest/chart_1.png", (dim, dim)).into_drawing_area();
    root.fill(&GREY)?;
    let xmin = 0f64;
    let xmax = c.generations as f64;
    let ymin = 0f64;
    let ymax = f64::from(best_score).sqrt();
    let mut chart = ChartBuilder::on(&root)
        .margin((f64::from(dim)) * 0.05)
        .build_cartesian_2d(xmin..xmax, ymin..ymax)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(
        data.iter()
            .map(|d| Circle::new((d[0], d[1].sqrt()), 1, YELLOW.filled())),
    )?;
    chart.configure_series_labels().draw()?;
    root.present()?;
    Ok(())
}

pub fn write_chart_2(data: &[Vec<f64>], best_score: f32) -> Result<(), Box<dyn std::error::Error>> {
    let dim = 1024;
    let root = BitMapBackend::new("latest/chart_2.png", (dim, dim)).into_drawing_area();
    root.fill(&GREY)?;
    let xmin = 0f64;
    let xmax = 100f64;
    let ymin = 0f64;
    let ymax = f64::from(best_score).sqrt();
    let mut chart = ChartBuilder::on(&root)
        .margin((f64::from(dim)) * 0.05)
        .build_cartesian_2d(xmin..xmax, ymin..ymax)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(
        data.iter()
            .map(|d| Circle::new((d[2], d[1].sqrt()), 1, YELLOW.filled())),
    )?;
    chart.configure_series_labels().draw()?;
    root.present()?;
    Ok(())
}
