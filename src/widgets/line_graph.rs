use crate::config::GRAPH_X_AXIS_WINDOW_IN_SECONDS;
use crate::data::gpu::GpuSnapshot;
use crate::data::DataStore;
use ratatui::layout::Constraint;

use ratatui::style::{Color, Style};
use ratatui::widgets::{GraphType, Widget};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols,
    widgets::{Axis, Block, BorderType, Chart, Dataset},
};

pub struct LineGraphWidget {}

impl LineGraphWidget {
    pub fn new() -> LineGraphWidget {
        LineGraphWidget {}
    }
}

impl LineGraphWidget {
    pub fn render(&self, area: Rect, buf: &mut Buffer, data: &DataStore) {
        let mut datasets = vec![];

        let data = data
            .snapshots
            .iter()
            .rev()
            .take(GRAPH_X_AXIS_WINDOW_IN_SECONDS)
            .rev();

        // TODO: See if we can avoid cloning here
        let gpu_data: Vec<&GpuSnapshot> = data.clone().filter_map(|s| s.gpu.as_ref()).collect();
        let gpu_mem_data: Vec<(f64, f64)> = gpu_data
            .iter()
            .map(|gpu| (gpu.used_memory as f64 / gpu.max_memory as f64) * 100.0)
            .enumerate()
            .map(|(t, g)| (t as f64, g))
            .collect();
        let gpu_use_data: Vec<(f64, f64)> = gpu_data
            .iter()
            .enumerate()
            .map(|(t, gpu)| (t as f64, gpu.utilization as f64))
            .collect();
        if !gpu_data.is_empty() {
            datasets.push(
                Dataset::default()
                    .name("GPU MEM%")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Yellow))
                    .graph_type(GraphType::Line)
                    .data(&gpu_mem_data),
            );

            datasets.push(
                Dataset::default()
                    .name("GPU %")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Blue))
                    .graph_type(GraphType::Line)
                    .data(&gpu_use_data),
            );
        }

        let cpu_data: Vec<(f64, f64)> = data
            .enumerate()
            .map(|(t, s)| (t as f64, s.cpu.as_ref().unwrap().usage as f64))
            .collect();

        datasets.push(
            Dataset::default()
                .name("CPU %")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Rgb(255, 105, 97)))
                .graph_type(GraphType::Line)
                .data(&cpu_data),
        );

        Chart::new(datasets)
            .block(Block::bordered().border_type(BorderType::Rounded))
            .x_axis(
                Axis::default()
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 120.0])
                    .labels(["0", "120"]),
            )
            .y_axis(
                Axis::default()
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 100.0])
                    .labels(["0", "100"]),
            )
            .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)))
            .render(area, buf);
    }
}
