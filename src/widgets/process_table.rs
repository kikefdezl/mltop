use crate::config::get_config;
use crate::constants::BYTES_PER_MB;
use crate::data::processes::{Process, ProcessType, ProcessesSnapshot};
use crate::widgets::state::process_table::{ProcessTableState, ProcessesSortBy};
use ratatui::widgets::StatefulWidget;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Cell, Row, Table},
};

const CONSTRAINTS: [Constraint; 6] = [
    Constraint::Length(6),
    Constraint::Length(8),
    Constraint::Length(5),
    Constraint::Length(6),
    Constraint::Length(9),
    Constraint::Min(10),
];

pub struct ProcessTableWidget<'a> {
    pub data: &'a ProcessesSnapshot,
    pub filter_by: Option<&'a str>,
}

impl<'a> StatefulWidget for ProcessTableWidget<'a> {
    type State = ProcessTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ProcessTableState) {
        let header = self.create_header(state);

        let processes = self.get_processes(state);

        let rows: Vec<Row> = processes
            .into_iter()
            .map(|d| Self::create_row(d, self.filter_by))
            .collect();

        Table::new(rows, CONSTRAINTS)
            .header(header)
            .row_highlight_style(Style::new().reversed())
            .render(area, buf, &mut state.ratatui_table_state);
    }
}

impl<'a> ProcessTableWidget<'a> {
    fn create_header(&self, state: &ProcessTableState) -> Row<'static> {
        let colors = &get_config().colors;
        let header_style = Style::default()
            .fg(colors.processes_header_fg)
            .bg(colors.processes_header_bg);
        let (cpu, mem) = match &state.sort_by {
            ProcessesSortBy::CPU => ("▽CPU%", "  MEM%"),
            ProcessesSortBy::MEM => (" CPU%", " ▽MEM%"),
        };

        ["   pid", "type", cpu, mem, "   MEMORY", "Command"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1)
    }

    fn create_row<'b>(data: Process, filter_by: Option<&'b str>) -> Row<'b> {
        let colors = &get_config().colors;
        let color = match data.type_ {
            ProcessType::GpuGraphic => colors.processes_gpu_graphic,
            ProcessType::GpuCompute => colors.processes_gpu_compute,
            ProcessType::UserThread => colors.processes_thread,
            ProcessType::KernelThread => colors.processes_thread,
            _ => colors.processes_cpu,
        };

        let cpu_text_color = if data.cpu_usage < 0.05 {
            Color::DarkGray
        } else {
            color
        };
        let mem_text_color = if data.memory_usage < 0.05 {
            Color::DarkGray
        } else {
            color
        };

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
            Self::create_cmd_cell(data.command, color, filter_by),
        ])
        .style(Style::default().fg(color))
    }

    // creates a Cell with the process command:
    // - highlights the `bin` part of the command with Magenta text
    // - highlights the `filter_by` matching string with a green background
    fn create_cmd_cell(cmd: String, color: Color, filter_by: Option<&str>) -> Cell<'_> {
        let bin_start = cmd.rfind('/').map(|i| i + 1).unwrap_or(0);
        let bin_end = cmd.find(' ').unwrap_or(cmd.len());

        let (match_start, match_end) = match filter_by {
            Some(s) => match cmd.find(s) {
                Some(m) => (m, m + s.len()),
                None => (0, 0),
            },
            None => (0, 0),
        };

        let mut cuts = vec![0, bin_start, bin_end, match_start, match_end, cmd.len()];
        cuts.sort_unstable();
        cuts.dedup();

        let spans: Vec<Span> = cuts
            .windows(2)
            .filter(|w| w[0] != w[1])
            .map(|w| {
                let (s, e) = (w[0], w[1]);
                let text = cmd[s..e].to_string();
                let mut style = Style::default().fg(color);

                // Apply magenta/bold for bin section
                if s < bin_end && e > bin_start {
                    style = style
                        .fg(get_config().colors.processes_bin_name)
                        .add_modifier(Modifier::BOLD);
                }

                // Apply green background for filter match
                if s < match_end && e > match_start {
                    style = style.fg(Color::Black).bg(Color::Green);
                }

                Span::styled(text, style)
            })
            .collect();

        Cell::from(Line::from(spans))
    }

    pub fn get_processes(&'a self, state: &mut ProcessTableState) -> Vec<Process> {
        let mut processes = Self::filter_processes(self.data.processes.clone(), self.filter_by);
        processes = Self::sort_processes(processes, &state.sort_by);
        processes = Self::filter_threads(processes, state.show_threads);
        processes
    }

    pub fn filter_processes(processes: Vec<Process>, filter_by: Option<&str>) -> Vec<Process> {
        match filter_by {
            Some(s) => processes
                .into_iter()
                .filter(|p| p.command.contains(s))
                .collect(),
            None => processes,
        }
    }

    pub fn sort_processes(mut processes: Vec<Process>, sort_by: &ProcessesSortBy) -> Vec<Process> {
        match sort_by {
            ProcessesSortBy::CPU => {
                processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap())
            }
            ProcessesSortBy::MEM => {
                processes.sort_by(|a, b| b.memory_usage.partial_cmp(&a.memory_usage).unwrap())
            }
        };
        processes
    }

    pub fn filter_threads(processes: Vec<Process>, show_threads: bool) -> Vec<Process> {
        if show_threads {
            processes
        } else {
            processes
                .into_iter()
                .filter(|p| !p.is_thread())
                .collect::<Vec<Process>>()
        }
    }

    pub fn get_nth_pid(&self, n: usize, state: &mut ProcessTableState) -> Option<u32> {
        Some(self.get_processes(state).get(n)?.pid)
    }
}
