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
        // We want the total used width to be area.width - 6, to match the CPU widget.
        // We split this into two sections: Memory (left half) and Swap (remainder).
        let total_width = area.width.saturating_sub(6);
        let mem_section_width = area.width / 2;
        let swap_section_width = total_width.saturating_sub(mem_section_width);

        // memory
        let mut spans = vec![Span::styled("  Memory", Style::default().fg(Color::Yellow))];
        let percentage = self.data.used as f32 / self.data.total as f32 * 100.0;
        let text = format!(
            "{:.1}G/{:.1}G",
            self.data.used as f32 / BYTES_PER_GB as f32,
            self.data.total as f32 / BYTES_PER_GB as f32
        );
        // width: section - label (8) - brackets (2) = section - 10
        let mem_bar_width = mem_section_width.saturating_sub(10);
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
        // width: section - label (4) - brackets (2) = section - 6
        let swap_bar_width = swap_section_width.saturating_sub(6);
        spans.extend(percentage_bar(swap_bar_width, percentage, &text));

        Paragraph::new(Line::from(spans))
            .left_aligned()
            .render(area, buf);
    }
}
