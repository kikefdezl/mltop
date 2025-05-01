use crate::data::cpu::CpuSnapshot;
use crate::utils::fast_int_sqrt;
use crate::widgets::percentage_bar::percentage_bar;

use ratatui::style::{Color, Style};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget},
};

pub struct CpuWidget {}

impl CpuWidget {
    pub fn new() -> CpuWidget {
        CpuWidget {}
    }

    // returns the dimensions of a grid to fit all cpu cores
    // in a pseudo-rectangular way (Rows, Cols)
    pub fn grid_dimensions(&self, cpu_snapshot: &CpuSnapshot) -> (u16, u16) {
        let cores = cpu_snapshot.cores.len();
        if cores <= 3 {
            return (cores as u16, 1);
        }
        let cpu_rows = fast_int_sqrt(cores) as u16;
        let mut cpu_cols: u16 = 0;
        while ((cpu_cols * cpu_rows) as usize) < cores {
            cpu_cols += 1;
        }
        (cpu_rows, cpu_cols)
    }
}

impl CpuWidget {
    pub fn render(&self, area: Rect, buf: &mut Buffer, data: &CpuSnapshot) {
        let (cpu_rows, cpu_cols) = self.grid_dimensions(data);

        let mut spans = vec![Span::styled(" Total ", Style::default().fg(Color::Cyan))];

        let usage = data.usage;
        let text = format!("{:.1}%", usage);
        let total_bar = percentage_bar(area.width - 16, usage, &text);
        spans.extend(total_bar);

        let mut lines = vec![Line::from(spans).left_aligned()];

        let total_width = area.width - 6;
        let core_width: u16 = total_width / cpu_cols;

        for r in 0..cpu_rows {
            let mut spans = vec![Span::raw("    ")];

            // this block makes the core bars wider to ensure that they
            // occupy the full required width to be well aligned
            let mut widths = vec![core_width; cpu_cols as usize];
            let remain: u16 = total_width - widths.iter().sum::<u16>();
            for i in 0..remain as usize {
                widths[i] += 1;
            }

            for c in 0..cpu_cols {
                let i = (c * cpu_rows + r) as usize;

                spans.push(Span::styled(
                    format!(" {:>2}", i),
                    Style::default().fg(Color::Cyan),
                )); // cpu number

                let text = format!("{:.1}%", data.cores[i].usage);
                let bar = percentage_bar(widths[c as usize] - 10, data.cores[i].usage, &text);
                spans.extend(bar);

                let (temp_str, color) = if data.cores[i].temp == 0.0 {
                    (" N/A ".to_string(), Color::DarkGray)
                } else {
                    let temp_str = format!("{:>3.0}°C", data.cores[i].temp);
                    if data.cores[i].temp > 90.0 {
                        (temp_str, Color::Red)
                    } else if data.cores[i].temp > 80.0 {
                        (temp_str, Color::Rgb(255, 130, 0)) // orange
                    } else if data.cores[i].temp > 70.0 {
                        (temp_str, Color::Yellow)
                    } else {
                        (temp_str, Color::White)
                    }
                };
                spans.push(Span::styled(temp_str, Style::default().fg(color)));
            }
            lines.push(Line::from(spans));
        }

        let content = Text::from(lines);

        Paragraph::new(content).left_aligned().render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::CpuSnapshot;
    use super::CpuWidget;
    use crate::data::cpu::CoreSnapshot;

    fn cpu_snap(cores: usize) -> CpuSnapshot {
        CpuSnapshot {
            usage: 0.0,
            cores: vec![
                CoreSnapshot {
                    usage: 0.0,
                    temp: 0.0
                };
                cores
            ],
        }
    }

    #[test]
    fn test_grid_dimensions() {
        let widget = CpuWidget {};

        let data = cpu_snap(1);
        assert_eq!(widget.grid_dimensions(&data), (1, 1));

        let data = cpu_snap(2);
        assert_eq!(widget.grid_dimensions(&data), (2, 1));

        let data = cpu_snap(3);
        assert_eq!(widget.grid_dimensions(&data), (3, 1));

        let data = cpu_snap(4);
        assert_eq!(widget.grid_dimensions(&data), (2, 2));

        let data = cpu_snap(5);
        assert_eq!(widget.grid_dimensions(&data), (2, 3));

        let data = cpu_snap(12);
        assert_eq!(widget.grid_dimensions(&data), (3, 4));

        let data = cpu_snap(16);
        assert_eq!(widget.grid_dimensions(&data), (4, 4));
    }
}
