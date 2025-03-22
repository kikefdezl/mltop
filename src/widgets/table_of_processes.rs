use crate::data::components::processes::{Process, Processes};
use crate::{constants::BYTES_PER_MB, data::components::processes::ProcessType};
use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Modifier, Style},
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
        let (cmd_path, cmd_bin, cmd_args) = Self::split_command(&data.command);

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
                Span::from(cmd_path),
                Span::styled(
                    cmd_bin,
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::from(" "),
                Span::from(cmd_args),
            ])),
        ])
        .style(Style::default().fg(color))
    }

    /// splits a command into three parts
    /// - prefix: the path to the binary
    /// - bin: the binary name
    /// - suffix: command arguments
    ///
    /// Example:
    ///     `/usr/bin/mltop --help`
    /// would return:
    ///     (/usr/bin, mltop, --help)
    fn split_command(cmd: &str) -> (String, String, String) {
        let split_idx0 = cmd.rfind('/').map_or(0, |i| i + 1);
        let path: String = cmd.chars().take(split_idx0).collect();

        let split_idx1 = cmd.find(' ').unwrap_or(cmd.len());
        let bin: String = cmd[..split_idx1].chars().skip(split_idx0).collect();

        let args: String = cmd.chars().skip(split_idx1 + 1).collect();
        (path, bin, args)
    }

    fn value_color(value: f32) -> Color {
        if value > 0.0 {
            Color::default()
        } else {
            Color::DarkGray
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TableOfProcessesWidget;

    #[test]
    fn test_split_command() {
        let (path, bin, args) = TableOfProcessesWidget::split_command("/usr/bin/mltop --help");
        assert_eq!(path, "/usr/bin/");
        assert_eq!(bin, "mltop");
        assert_eq!(args, "--help");

        let (path, bin, args) = TableOfProcessesWidget::split_command("mltop");
        assert_eq!(path, "");
        assert_eq!(bin, "mltop");
        assert_eq!(args, "");

        let (path, bin, args) = TableOfProcessesWidget::split_command("/bin/bash -c 'echo hello'");
        assert_eq!(path, "/bin/");
        assert_eq!(bin, "bash");
        assert_eq!(args, "-c 'echo hello'");

        let (path, bin, args) = TableOfProcessesWidget::split_command("/usr/local/bin/python3");
        assert_eq!(path, "/usr/local/bin/");
        assert_eq!(bin, "python3");
        assert_eq!(args, "");
    }
}
