use crate::data::models::cpu::Cpu;
use crate::utils::fast_int_sqrt;
use crate::widgets::percentage_bar::percentage_bar;

use ratatui::style::{Color, Style};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget},
};

pub struct CpuWidget<'a> {
    data: &'a Cpu,
}

impl CpuWidget<'_> {
    pub fn new(data: &Cpu) -> CpuWidget {
        CpuWidget { data }
    }

    // returns the dimensions of a grid to fit all cpu cores
    // in a pseudo-rectangular way (Rows, Cols)
    pub fn grid_dimensions(&self) -> (u16, u16) {
        let cores = self.data.cores.len();
        let cpu_cols = fast_int_sqrt(cores) as u16;
        let mut cpu_rows: u16 = 0;
        while ((cpu_rows * cpu_cols) as usize) < cores {
            cpu_rows += 1;
        }
        (cpu_rows, cpu_cols)
    }
}

impl Widget for CpuWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut spans = vec![Span::styled(" Total ", Style::default().fg(Color::Cyan))];

        let usage = self.data.usage.last().unwrap();
        let text = format!("{:.1}%", usage);
        let total_bar = percentage_bar(area.width - 16, *usage, &text);
        spans.extend(total_bar);

        let mut lines = vec![Line::from(spans).left_aligned()];

        let (cpu_rows, cpu_cols) = self.grid_dimensions();

        let total_width = area.width - 7;
        let core_width: u16 = total_width / cpu_cols;

        for r in 0..cpu_rows {
            let mut spans = vec![Span::raw("     ")];

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
                    format!("{:>2}", i),
                    Style::default().fg(Color::Cyan),
                )); // cpu number

                let text = format!("{:.1}%", self.data.cores[i].usage);
                let bar = percentage_bar(widths[c as usize] - 9, self.data.cores[i].usage, &text);
                spans.extend(bar);

                let (temp_str, color) = if self.data.cores[i].temp == 0.0 {
                    (" N/A ".to_string(), Color::DarkGray)
                } else {
                    let temp_str = format!("{:>3.0}°C", self.data.cores[i].temp);
                    if self.data.cores[i].temp > 90.0 {
                        (temp_str, Color::Red)
                    } else if self.data.cores[i].temp > 80.0 {
                        (temp_str, Color::Rgb(255, 130, 0)) // orange
                    } else if self.data.cores[i].temp > 70.0 {
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
    use super::Cpu;
    use super::CpuWidget;
    use crate::data::models::cpu::Core;

    fn fake_cpu(cores: usize) -> Cpu {
        Cpu {
            usage: vec![0.0],
            cores: vec![
                Core {
                    usage: 0.0,
                    temp: 0.0
                };
                cores
            ],
        }
    }

    #[test]
    fn test_grid_dimensions() {
        let widget = CpuWidget { data: &fake_cpu(1) };
        assert_eq!(widget.grid_dimensions(), (1, 1));

        let widget = CpuWidget { data: &fake_cpu(2) };
        assert_eq!(widget.grid_dimensions(), (2, 1));

        let widget = CpuWidget { data: &fake_cpu(3) };
        assert_eq!(widget.grid_dimensions(), (3, 1));

        let widget = CpuWidget { data: &fake_cpu(4) };
        assert_eq!(widget.grid_dimensions(), (2, 2));

        let widget = CpuWidget { data: &fake_cpu(5) };
        assert_eq!(widget.grid_dimensions(), (3, 2));

        let widget = CpuWidget {
            data: &fake_cpu(12),
        };
        assert_eq!(widget.grid_dimensions(), (4, 3));

        let widget = CpuWidget {
            data: &fake_cpu(16),
        };
        assert_eq!(widget.grid_dimensions(), (4, 4));
    }
}
