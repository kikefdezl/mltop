use crate::constants::BYTES_PER_MB;
use crate::data::components::processes::Processes;
use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Cell, Row, Table, Widget},
};

pub struct ProcessesWidget {
    data: Processes,
}

impl ProcessesWidget {
    pub fn new(data: Processes) -> ProcessesWidget {
        ProcessesWidget { data }
    }
}

impl Widget for ProcessesWidget {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let header_style = Style::default().fg(Color::Black).bg(Color::White);

        let header = ["   pid", "type", " CPU%", "  MEM%", "      MEM", "Command"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        self.data.0.sort_by_key(|p| (p.cpu_usage * 1000.0) as u32);
        let rows: Vec<Row> = self
            .data
            .into_iter()
            .sorted_by(|a, b| b.cpu_usage.total_cmp(&a.cpu_usage))
            .map(|data| {
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
            })
            .collect::<Vec<Row>>();

        Table::new(
            rows,
            [
                Constraint::Length(6),
                Constraint::Length(8),
                Constraint::Length(5),
                Constraint::Length(6),
                Constraint::Length(9),
                Constraint::Min(10),
            ],
        )
        .header(header)
        .render(area, buf);
    }
}
