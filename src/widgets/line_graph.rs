use crate::config::{get_config, GRAPH_X_AXIS_WINDOW_IN_SECONDS};
use crate::data::store::{DataStore, StoredSnapshot};
use ratatui::layout::Constraint;

use ratatui::style::{Color, Style};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols,
    widgets::{Axis, Block, BorderType, Chart, Dataset, GraphType, Widget},
};

pub struct LineGraphWidget<'a> {
    pub data: &'a DataStore,
    pub max_gpu_mem: Option<u64>,
}

impl<'a> Widget for LineGraphWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut datasets = vec![];

        let colors = &get_config().colors;

        let data: Vec<&StoredSnapshot> = self
            .data
            .snapshots
            .iter()
            .rev()
            .take(GRAPH_X_AXIS_WINDOW_IN_SECONDS)
            .rev()
            .collect();

        // GPU USE %
        let gpu_use_data: Vec<(f64, f64)> = data
            .iter()
            .filter_map(|s| s.gpu_use)
            .enumerate()
            .map(|(t, g)| (t as f64, g as f64))
            .collect();
        if !gpu_use_data.is_empty() {
            datasets.push(
                Dataset::default()
                    .name("GPU %")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(colors.line_graph_gpu_use))
                    .graph_type(GraphType::Line)
                    .data(&gpu_use_data),
            );
        }

        // GPU MEM %
        let gpu_mem_data: Vec<(f64, f64)> = data
            .iter()
            .filter_map(|s| s.gpu_mem_use)
            .enumerate()
            .map(|(t, g)| {
                (
                    t as f64,
                    (g as f64 / self.max_gpu_mem.unwrap() as f64 * 100.0),
                )
            })
            .collect();
        if !gpu_mem_data.is_empty() {
            datasets.push(
                Dataset::default()
                    .name("GPU MEM%")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(colors.line_graph_gpu_mem))
                    .graph_type(GraphType::Line)
                    .data(&gpu_mem_data),
            );
        }

        // CPU %
        let cpu_data: Vec<(f64, f64)> = data
            .iter()
            .enumerate()
            .map(|(t, s)| (t as f64, s.cpu_use as f64))
            .collect();
        datasets.push(
            Dataset::default()
                .name("CPU %")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(colors.line_graph_cpu))
                .graph_type(GraphType::Line)
                .data(&cpu_data),
        );

        // MEM %
        let mem_data: Vec<(f64, f64)> = data
            .iter()
            .enumerate()
            .map(|(t, s)| (t as f64, s.mem_use * 100.0))
            .collect();
        datasets.push(
            Dataset::default()
                .name("MEM %")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(colors.line_graph_mem))
                .graph_type(GraphType::Line)
                .data(&mem_data),
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
