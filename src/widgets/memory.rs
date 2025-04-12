use crate::constants::BYTES_PER_GB;
use crate::data::models::memory::Memory;
use crate::widgets::percentage_bar::percentage_bar;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

pub const MEMORY_WIDGET_HEIGHT: u16 = 1;

pub struct MemoryWidget<'a> {
    data: &'a Memory,
}

impl MemoryWidget<'_> {
    pub fn new<'a>(data: &'a Memory) -> MemoryWidget<'a> {
        MemoryWidget { data }
    }
}

impl Widget for MemoryWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // memory
        let mut spans = vec![Span::styled(" Memory", Style::default().fg(Color::Yellow))];
        let percentage = self.data.used as f32 / self.data.total as f32 * 100.0;
        let text = format!(
            "{:.1}G/{:.1}G",
            self.data.used as f32 / BYTES_PER_GB as f32,
            self.data.total as f32 / BYTES_PER_GB as f32
        );
        spans.extend(percentage_bar((area.width / 2) - 16, percentage, &text));

        // swap
        spans.extend(vec![Span::styled(
            "      Swap",
            Style::default().fg(Color::Yellow),
        )]);
        let percentage = self.data.used_swap as f32 / self.data.total_swap as f32 * 100.0;
        let text = format!(
            "{:.1}G/{:.1}G",
            self.data.used_swap as f32 / BYTES_PER_GB as f32,
            self.data.total_swap as f32 / BYTES_PER_GB as f32
        );
        spans.extend(percentage_bar((area.width / 2) - 16, percentage, &text));

        Paragraph::new(Line::from(spans))
            .left_aligned()
            .render(area, buf);
    }
}
