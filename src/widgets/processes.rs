use crate::data::components::processes::Processes;

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
        let header_style = Style::default().fg(Color::Black).bg(Color::LightGreen);

        let header = ["   pid", "type", "Command"]
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
                    Cell::from(Text::from(data.command)),
                ])
            })
            .collect::<Vec<Row>>();

        Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(6),
                Constraint::Length(8),
                Constraint::Min(10),
            ],
        )
        .header(header)
        .render(area, buf);
    }
}
