use crate::constants::BYTES_PER_GB_FLOAT;
use crate::data::components::gpu::Gpu;
use crate::widgets::percentage_bar::percentage_bar;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
};

pub const GPU_WIDGET_HEIGHT: u16 = 4;

pub struct GpuWidget {
    data: Option<Gpu>,
}

impl GpuWidget {
    pub fn new(data: Option<Gpu>) -> GpuWidget {
        GpuWidget { data }
    }
}

impl Widget for GpuWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match &self.data {
            None => {
                let block = Block::bordered()
                    .border_type(BorderType::Rounded)
                    .padding(Padding::new(1, 1, 0, 0));
                Paragraph::new(Text::from("No GPU found."))
                    .centered()
                    .block(block)
                    .render(area, buf);
            }
            Some(gpu) => {
                let block = Block::bordered()
                    .title(format!(" {}", gpu.name.clone()))
                    .border_type(BorderType::Rounded)
                    .padding(Padding::new(1, 1, 0, 0));

                let mut spans = vec![Span::styled("TEMP:", Style::default().fg(Color::Cyan))];
                spans.push(Span::raw(format!(" {}°C", gpu.temperature)));

                spans.push(Span::styled("   POW:", Style::default().fg(Color::Cyan)));
                spans.push(Span::raw(format!(
                    " {} W / {} W",
                    gpu.power_usage / 1000,
                    gpu.max_power / 1000
                )));

                spans.push(Span::styled("   FAN:", Style::default().fg(Color::Cyan)));
                spans.push(Span::raw(format!("  {:.0}%", gpu.fan_speed)));

                let mut lines = vec![Line::from(spans).alignment(Alignment::Left)];

                let mut spans = vec![Span::styled("GPU", Style::default().fg(Color::Cyan))];
                let utilization = gpu.utilization.last().unwrap();
                spans.extend(percentage_bar(
                    area.width / 3 - 5,
                    *utilization as f32,
                    &format!("{}%", utilization),
                ));

                spans.push(Span::styled(" MEM", Style::default().fg(Color::Cyan)));
                let used: u64 = *gpu.used_memory.last().unwrap();
                let mem_perc: f32 = (used as f32 / gpu.max_memory as f32) * 100.0;
                spans.extend(percentage_bar(
                    area.width / 3 - 5,
                    mem_perc,
                    &format!(
                        "{:.2}Gi/{:.2}Gi",
                        (used as f32) / BYTES_PER_GB_FLOAT,
                        (gpu.max_memory as f32) / BYTES_PER_GB_FLOAT
                    ),
                ));
                lines.push(Line::from(spans).alignment(Alignment::Left));

                let content = Text::from(lines);
                Paragraph::new(content)
                    .centered()
                    .block(block)
                    .render(area, buf);
            }
        }
    }
}
