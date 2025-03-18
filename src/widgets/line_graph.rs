use crate::config::GRAPH_X_AXIS_WINDOW_IN_SECONDS;
use crate::data::components::cpu::Cpu;
use crate::data::components::gpu::Gpu;
use ratatui::layout::Constraint;

use ratatui::style::{Color, Style};
use ratatui::widgets::{GraphType, Widget};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols,
    widgets::{Axis, Block, BorderType, Chart, Dataset},
};

pub struct LineGraphWidget<'a> {
    cpu_data: &'a Cpu,
    gpu_data: &'a Option<Gpu>,
}

impl LineGraphWidget<'_> {
    pub fn new<'a>(cpu_data: &'a Cpu, gpu_data: &'a Option<Gpu>) -> LineGraphWidget<'a> {
        LineGraphWidget { cpu_data, gpu_data }
    }
}

impl Widget for LineGraphWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut datasets = vec![];

        let gpu_mem_data: Option<Vec<(f64, f64)>> = self.gpu_data.as_ref().map(|gpu| {
            gpu.used_memory
                .iter()
                .rev()
                .take(GRAPH_X_AXIS_WINDOW_IN_SECONDS)
                .rev()
                .enumerate()
                .map(|(t, &x)| (t as f64, (x as f64 / gpu.max_memory as f64) * 100.0))
                .collect()
        });
        if let Some(ref gpu_mem_data) = gpu_mem_data {
            datasets.push(
                Dataset::default()
                    .name("GPU MEM%")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Yellow))
                    .graph_type(GraphType::Line)
                    .data(gpu_mem_data),
            );
        }

        let gpu_use_data: Option<Vec<(f64, f64)>> = self.gpu_data.as_ref().map(|gpu| {
            gpu.utilization
                .iter()
                .rev()
                .take(GRAPH_X_AXIS_WINDOW_IN_SECONDS)
                .rev()
                .enumerate()
                .map(|(t, &x)| (t as f64, x as f64))
                .collect()
        });
        if let Some(ref gpu_data) = gpu_use_data {
            datasets.push(
                Dataset::default()
                    .name("GPU %")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Blue))
                    .graph_type(GraphType::Line)
                    .data(gpu_data),
            );
        }

        let cpu_data: Vec<(f64, f64)> = self
            .cpu_data
            .usage
            .iter()
            .rev()
            .take(GRAPH_X_AXIS_WINDOW_IN_SECONDS)
            .rev()
            .enumerate()
            .map(|(t, &x)| (t as f64, x as f64))
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
