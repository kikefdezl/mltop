use crate::widgets::percentage_bar::percentage_bar;
use crate::{constants::BYTES_PER_GB, data::memory::MemorySnapshot};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

pub const MEMORY_WIDGET_HEIGHT: u16 = 1;

pub struct MemoryWidget<'a> {
    pub data: &'a MemorySnapshot,
}

impl<'a> Widget for MemoryWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // memory
        let mut spans = vec![Span::styled("  Memory", Style::default().fg(Color::Yellow))];
        let percentage = self.data.used as f32 / self.data.total as f32 * 100.0;
        let text = format!(
            "{:.1}G/{:.1}G",
            self.data.used as f32 / BYTES_PER_GB as f32,
            self.data.total as f32 / BYTES_PER_GB as f32
        );
        let mem_bar_width = (area.width / 2) - 10;
        spans.extend(percentage_bar(mem_bar_width, percentage, &text));

        // swap
        spans.extend(vec![Span::styled(
            " Swp",
            Style::default().fg(Color::Yellow),
        )]);
        let percentage = self.data.used_swap as f32 / self.data.total_swap as f32 * 100.0;
        let text = format!(
            "{:.1}G/{:.1}G",
            self.data.used_swap as f32 / BYTES_PER_GB as f32,
            self.data.total_swap as f32 / BYTES_PER_GB as f32
        );
        let swap_bar_width = area.width - mem_bar_width - 22;
        spans.extend(percentage_bar(swap_bar_width, percentage, &text));

        Paragraph::new(Line::from(spans))
            .left_aligned()
            .render(area, buf);
    }
}
