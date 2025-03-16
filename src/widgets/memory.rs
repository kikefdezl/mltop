use crate::constants::BYTES_PER_GB;
use crate::data::components::memory::Memory;
use crate::widgets::percentage_bar::percentage_bar;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget},
};

pub const MEMORY_WIDGET_HEIGHT: u16 = 2;

pub struct MemoryWidget {
    data: Memory,
}

impl MemoryWidget {
    pub fn new(data: Memory) -> MemoryWidget {
        MemoryWidget { data }
    }
}

impl Widget for MemoryWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut spans = vec![Span::styled("Memory", Style::default().fg(Color::Cyan))];
        let percentage = self.data.used as f32 / self.data.total as f32 * 100.0;
        let text = format!(
            "{:.1}G/{:.1}G",
            self.data.used as f32 / BYTES_PER_GB as f32,
            self.data.total as f32 / BYTES_PER_GB as f32
        );
        spans.extend(percentage_bar(area.width - 11, percentage, &text));

        let mut lines = vec![Line::from(spans)];

        let mut spans = vec![Span::styled("  Swap", Style::default().fg(Color::Cyan))];
        let percentage = self.data.used_swap as f32 / self.data.total_swap as f32 * 100.0;
        let text = format!(
            "{:.1}G/{:.1}G",
            self.data.used_swap as f32 / BYTES_PER_GB as f32,
            self.data.total_swap as f32 / BYTES_PER_GB as f32
        );
        spans.extend(percentage_bar(area.width - 11, percentage, &text));

        lines.push(Line::from(spans));

        let content = Text::from(lines);
        Paragraph::new(content).centered().render(area, buf);
    }
}
