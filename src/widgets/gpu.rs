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

pub struct GpuWidget {}

impl GpuWidget {
    pub fn new() -> GpuWidget {
        GpuWidget {}
    }
}

impl GpuWidget {
    pub fn render(&self, area: Rect, buf: &mut Buffer, data: &GpuSnapshot) {
        let block = Block::bordered()
            .title(format!(" {}", data.name.clone()))
            .border_type(BorderType::Rounded)
            .padding(Padding::new(1, 1, 0, 0));

        let mut spans = vec![Span::styled("TEMP:", Style::default().fg(Color::Cyan))];
        spans.push(Span::raw(format!(" {}°C", data.temperature)));

        spans.push(Span::styled("   POW:", Style::default().fg(Color::Cyan)));
        spans.push(Span::raw(format!(
            " {} W / {} W",
            data.power_usage / 1000,
            data.max_power / 1000
        )));

        spans.push(Span::styled("   FAN:", Style::default().fg(Color::Cyan)));
        spans.push(Span::raw(format!("  {:.0}%", data.fan_speed)));

        let mut lines = vec![Line::from(spans).alignment(Alignment::Left)];

        let mut spans = vec![Span::styled("GPU", Style::default().fg(Color::Cyan))];
        spans.extend(percentage_bar(
            area.width / 3 - 5,
            data.utilization as f32,
            &format!("{}%", data.utilization),
        ));

        spans.push(Span::styled(" MEM", Style::default().fg(Color::Cyan)));
        let mem_perc: f32 = (data.used_memory as f32 / data.max_memory as f32) * 100.0;
        spans.extend(percentage_bar(
            area.width / 3 - 5,
            mem_perc,
            &format!(
                "{:.2}Gi/{:.2}Gi",
                (data.used_memory as f32) / BYTES_PER_GB_FLOAT,
                (data.max_memory as f32) / BYTES_PER_GB_FLOAT
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
