use crate::data::components::processes::Processes;
use crate::constants::BYTES_PER_MB;

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
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header_style = Style::default()
            .fg(Color::Black)
            .bg(Color::White);

        let header = ["   pid", "type", " CPU%", " MEM%", "      MEM", "Command"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let rows: Vec<Row> = self
            .data
            .into_iter()
            .map(|data| {
                Row::new(vec![
                    Cell::from(Text::from(data.pid.to_string()).alignment(Alignment::Right)),
                    Cell::from(Text::from(data.type_.to_string())),
                    Cell::from(
                        Text::from(format!("{:.0}%", data.cpu_usage)).alignment(Alignment::Right),
                    ),
                    Cell::from(
                        Text::from(format!("{:.0}%", data.memory_usage))
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
                Constraint::Length(5),
                Constraint::Length(9),
                Constraint::Min(10),
            ],
        )
        .header(header)
        .render(area, buf);
    }
}
