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

pub struct CpuWidget<'a> {
    pub data: &'a CpuSnapshot,
}

impl<'a> CpuWidget<'a> {
    // returns the dimensions of a grid to fit all cpu cores
    // in a pseudo-rectangular way (Rows, Cols)
    pub fn grid_dimensions(&self) -> (u16, u16) {
        let cores = self.data.cores.len();
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

    pub fn grid_height(&self) -> u16 {
        self.grid_dimensions().0
    }
}

impl<'a> Widget for CpuWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (cpu_rows, cpu_cols) = self.grid_dimensions();

        let mut spans = vec![Span::styled("  Total ", Style::default().fg(Color::Cyan))];

        let usage = self.data.usage;
        let text = format!("{:.1}%", usage);
        let total_bar = percentage_bar(area.width - 16, usage, &text);
        spans.extend(total_bar);

        let cores_len = self.data.cores.len();

        let mut lines = vec![Line::from(spans).left_aligned()];

        let total_width = area.width.saturating_sub(10);
        let core_width: u16 = if cpu_cols > 0 {
            total_width / cpu_cols
        } else {
            0
        };

        for r in 0..cpu_rows {
            let mut spans = vec![Span::raw("    ")];

            // this block makes the core bars wider to ensure that they
            // occupy the full required width to be well aligned
            let mut widths = vec![core_width; cpu_cols as usize];
            let remain: u16 = total_width - widths.iter().sum::<u16>();
            for width in widths.iter_mut().take(remain as usize) {
                *width += 1;
            }

            'inner: for c in 0..cpu_cols {
                let i = (c * cpu_rows + r) as usize;

                if i >= cores_len {
                    break 'inner;
                }

                // cpu number
                spans.push(Span::styled(
                    format!("  {:>2}", i),
                    Style::default().fg(Color::Cyan),
                ));

                // bar
                // width includes: label (4 chars) + brackets (2 chars) + bar content
                // we want total width of this column to be `widths[c]`
                // label takes 4 chars ("  XX")
                // percentage_bar adds 2 chars for brackets "[]"
                // so we need bar content width = widths[c] - 4 - 2 = widths[c] - 6
                let width = widths[c as usize].saturating_sub(6);
                let usage = self.data.cores[i].usage;
                let text = format!("{:.1}%{:>3.0}°C", usage, self.data.cores[i].temp);
                let bar = percentage_bar(width, usage, &text);
                spans.extend(bar);

                // TODO: Show something red again if the core temperature gets too high.
                // Maybe the sidebars of the core itself (the white brackets).
                //
                // temperature with color
                // let (temp_str, color) = if self.data.cores[i].temp == 0.0 {
                // (" N/A ".to_string(), Color::DarkGray)
                // } else {
                // let temp_str = format!("{:>3.0}°C", self.data.cores[i].temp);
                // if self.data.cores[i].temp > 90.0 {
                // (temp_str, Color::Red)
                // } else if self.data.cores[i].temp > 80.0 {
                // (temp_str, Color::Rgb(255, 130, 0)) // orange
                // } else if self.data.cores[i].temp > 70.0 {
                // (temp_str, Color::Yellow)
                // } else {
                // (temp_str, Color::White)
                // }
                // };
                // spans.push(Span::styled(temp_str, Style::default().fg(color)));
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
        let widget = CpuWidget { data: &cpu_snap(1) };
        assert_eq!(widget.grid_dimensions(), (1, 1));

        let widget = CpuWidget { data: &cpu_snap(2) };
        assert_eq!(widget.grid_dimensions(), (2, 1));

        let widget = CpuWidget { data: &cpu_snap(3) };
        assert_eq!(widget.grid_dimensions(), (3, 1));

        let widget = CpuWidget { data: &cpu_snap(4) };
        assert_eq!(widget.grid_dimensions(), (2, 2));

        let widget = CpuWidget { data: &cpu_snap(5) };
        assert_eq!(widget.grid_dimensions(), (2, 3));

        let widget = CpuWidget {
            data: &cpu_snap(12),
        };
        assert_eq!(widget.grid_dimensions(), (3, 4));

        let widget = CpuWidget {
            data: &cpu_snap(16),
        };
        assert_eq!(widget.grid_dimensions(), (4, 4));

        let widget = CpuWidget {
            data: &cpu_snap(32),
        };
        assert_eq!(widget.grid_dimensions(), (5, 7));
    }
}
