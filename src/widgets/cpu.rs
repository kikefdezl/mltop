use crate::data::components::cpu::Cpu;
use crate::utils::fast_int_sqrt;
use crate::widgets::percentage_bar::percentage_bar;

use ratatui::style::{Color, Style};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget},
};

pub struct CpuWidget {
    data: Cpu,
}

impl CpuWidget {
    pub fn new(data: Cpu) -> CpuWidget {
        CpuWidget { data }
    }
}

impl Widget for CpuWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut spans = vec![Span::styled("Total ", Style::default().fg(Color::Cyan))];

        let usage = self.data.usage.last().unwrap();
        let text = format!("{:.1}%", usage);
        let total_bar = percentage_bar(area.width - 10, *usage, &text);
        spans.extend(total_bar);

        let mut lines = vec![Line::from(spans)];

        let num_cpus = self.data.cores.len();
        let cpu_rows = fast_int_sqrt(num_cpus);
        let mut cpu_cols = 0;
        while cpu_cols * cpu_rows < num_cpus {
            cpu_cols += 1;
        }

        let core_width = area.width as usize / cpu_cols;

        for r in 0..cpu_rows {
            let mut spans = vec![];
            for c in 0..cpu_cols {
                let i = c * cpu_rows + r;

                spans.push(Span::styled(
                    format!(" {:>2}", i),
                    Style::default().fg(Color::Cyan),
                )); // cpu number

                let text = format!("{:.1}%", self.data.cores[i].usage);
                let bar = percentage_bar(core_width as u16 - 14, self.data.cores[i].usage, &text);
                spans.extend(bar);

                let (temp_str, color) = if self.data.cores[i].temp == 0.0 {
                    (" N/A   ".to_string(), Color::White)
                } else {
                    let temp_str = format!("{:>5.1}°C", self.data.cores[i].temp);
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

        Paragraph::new(content).centered().render(area, buf);
    }
}
