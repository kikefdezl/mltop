use crate::constants::BYTES_PER_GB_FLOAT;
use crate::data::gpu::GpuSnapshot;
use crate::widgets::percentage_bar::percentage_bar;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
};

pub const GPU_WIDGET_HEIGHT: u16 = 4;

pub struct GpuWidget<'a> {
    pub data: &'a GpuSnapshot,
}

impl<'a> Widget for GpuWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(format!(" {}", self.data.name.clone()))
            .border_type(BorderType::Rounded)
            .padding(Padding::new(1, 1, 0, 0));

        let mut spans = vec![Span::styled("TEMP:", Style::default().fg(Color::Cyan))];
        spans.push(Span::raw(format!(" {}°C", self.data.temperature)));

        spans.push(Span::styled("   POW:", Style::default().fg(Color::Cyan)));
        spans.push(Span::raw(format!(
            " {} W / {} W",
            self.data.power_usage / 1000,
            self.data.max_power / 1000
        )));

        spans.push(Span::styled("   FAN:", Style::default().fg(Color::Cyan)));
        match self.data.fan_speed {
            Some(fan_speed) => spans.push(Span::raw(format!("  {:.0}%", fan_speed))),
            None => spans.push(Span::raw("  N/A")),
        }

        let mut lines = vec![Line::from(spans).alignment(Alignment::Left)];

        let mut spans = vec![Span::styled("GPU", Style::default().fg(Color::Cyan))];
        spans.extend(percentage_bar(
            area.width / 3 - 5,
            self.data.utilization as f32,
            &format!("{}%", self.data.utilization),
        ));

        spans.push(Span::styled(" MEM", Style::default().fg(Color::Cyan)));
        let mem_perc: f32 = (self.data.used_memory as f32 / self.data.max_memory as f32) * 100.0;
        spans.extend(percentage_bar(
            area.width / 3 - 5,
            mem_perc,
            &format!(
                "{:.2}Gi/{:.2}Gi",
                (self.data.used_memory as f32) / BYTES_PER_GB_FLOAT,
                (self.data.max_memory as f32) / BYTES_PER_GB_FLOAT
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
