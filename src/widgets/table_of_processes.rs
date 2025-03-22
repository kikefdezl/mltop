use crate::data::components::processes::{Process, Processes};
use crate::{constants::BYTES_PER_MB, data::components::processes::ProcessType};
use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Cell, Row, Table, Widget},
};

const GPU_COMPUTE_COLOR: Color = Color::Magenta;
const GPU_GRAPHIC_COLOR: Color = Color::Yellow;

const HEADER: [&str; 6] = ["   pid", "type", " CPU%", "  MEM%", "   MEMORY", "Command"];
const CONSTRAINTS: [Constraint; 6] = [
    Constraint::Length(6),
    Constraint::Length(8),
    Constraint::Length(5),
    Constraint::Length(6),
    Constraint::Length(9),
    Constraint::Min(10),
];

pub struct TableOfProcessesWidget<'a> {
    data: &'a Processes,
}

impl Widget for TableOfProcessesWidget<'_> {
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
            .map(|data| Self::create_row(data))
            .collect();

        Table::new(rows, CONSTRAINTS)
            .header(header)
            .render(area, buf);
    }
}

impl TableOfProcessesWidget<'_> {
    pub fn new<'a>(data: &'a Processes) -> TableOfProcessesWidget<'a> {
        TableOfProcessesWidget { data }
    }

    fn create_row<'a>(data: Process) -> Row<'a> {
        let color = match data.type_ {
            ProcessType::GpuGraphic => GPU_GRAPHIC_COLOR,
            ProcessType::GpuCompute => GPU_COMPUTE_COLOR,
            _ => Color::White,
        };

        let cpu_text_color = Self::value_color(data.cpu_usage);
        let mem_text_color = Self::value_color(data.memory_usage);
        let (cmd_prefix, cmd_suffix) = Self::split_command(&data.command);

        Row::new(vec![
            Cell::from(Text::from(data.pid.to_string()).alignment(Alignment::Right)),
            Cell::from(Text::from(data.type_.to_string())),
            Cell::from(
                Line::from(vec![
                    Span::styled(
                        format!("{:.1}", data.cpu_usage)
                            .chars()
                            .take(4)
                            .collect::<String>(),
                        Style::default().fg(cpu_text_color),
                    ),
                    Span::styled("%", Style::default().fg(Color::DarkGray)),
                ])
                .alignment(Alignment::Right),
            ),
            Cell::from(
                Line::from(vec![
                    Span::styled(
                        format!("{:.1}", data.memory_usage),
                        Style::default().fg(mem_text_color),
                    ),
                    Span::styled("%", Style::default().fg(Color::DarkGray)),
                ])
                .alignment(Alignment::Right),
            ),
            Cell::from(
                Line::from(vec![
                    Span::from(format!("{:.0}", data.memory / BYTES_PER_MB)),
                    Span::styled("MiB", Style::default().fg(Color::DarkGray)),
                ])
                .alignment(Alignment::Right),
            ),
            Cell::from(Line::from(vec![
                Span::from(cmd_prefix),
                Span::styled(cmd_suffix, Style::default().fg(Color::Magenta)),
            ])),
        ])
        .style(Style::default().fg(color))
    }

    fn split_command(cmd: &str) -> (String, String) {
        let split_idx = cmd.rfind('/').map_or(0, |i| i + 1);
        let prefix: String = cmd.chars().take(split_idx).collect();
        let suffix: String = cmd.chars().skip(split_idx).collect();
        (prefix, suffix)
    }

    fn value_color(value: f32) -> Color {
        if value > 0.0 {
            Color::default()
        } else {
            Color::DarkGray
        }
    }
}
