use crate::data::components::processes::Processes;
use crate::{constants::BYTES_PER_MB, data::components::processes::ProcessType};
use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Cell, Row, Table, Widget},
};

const GPU_COMPUTE_COLOR: Color = Color::Magenta;
const GPU_GRAPHIC_COLOR: Color = Color::Yellow;

const HEADER: [&str; 6] = ["   pid", "type", "  CPU%", "  MEM%", "      MEM", "Command"];
const CONSTRAINTS: [Constraint; 6] = [
    Constraint::Length(6),
    Constraint::Length(8),
    Constraint::Length(6),
    Constraint::Length(6),
    Constraint::Length(9),
    Constraint::Min(10),
];

pub struct ProcessesWidget<'a> {
    data: &'a Processes,
}

impl ProcessesWidget<'_> {
    pub fn new<'a>(data: &'a Processes) -> ProcessesWidget<'a> {
        ProcessesWidget { data }
    }
}

impl Widget for ProcessesWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header_style = Style::default().fg(Color::Black).bg(Color::White);

        let header = HEADER
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let mut data = self.data.0.clone();
        data.sort_by_key(|p| (p.cpu_usage * 1000.0) as u32);
        let rows: Vec<Row> = self
            .data
            .into_iter()
            .sorted_by(|a, b| b.cpu_usage.total_cmp(&a.cpu_usage)) // TODO: allow user what to sort by
            .map(|data| {
                let color = match data.type_ {
                    ProcessType::GpuGraphic => GPU_GRAPHIC_COLOR,
                    ProcessType::GpuCompute => GPU_COMPUTE_COLOR,
                    _ => Color::White,
                };
                Row::new(vec![
                    Cell::from(Text::from(data.pid.to_string()).alignment(Alignment::Right)),
                    Cell::from(Text::from(data.type_.to_string())),
                    Cell::from(
                        Text::from(format!("{:.1}%", data.cpu_usage)).alignment(Alignment::Right),
                    ),
                    Cell::from(
                        Text::from(format!("{:.1}%", data.memory_usage))
                            .alignment(Alignment::Right),
                    ),
                    Cell::from(
                        Text::from(format!("{:.0}MiB", data.memory / BYTES_PER_MB))
                            .alignment(Alignment::Right),
                    ),
                    Cell::from(Text::from(data.command)),
                ])
                .style(Style::default().fg(color))
            })
            .collect::<Vec<Row>>();

        Table::new(rows, CONSTRAINTS)
            .header(header)
            .render(area, buf);
    }
}
