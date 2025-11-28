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

#[derive(Default)]
pub struct MemoryWidget {}

impl MemoryWidget {
    pub fn new() -> MemoryWidget {
        MemoryWidget::default()
    }
}

impl MemoryWidget {
    pub fn render(&self, area: Rect, buf: &mut Buffer, data: &MemorySnapshot) {
        // memory
        let mut spans = vec![Span::styled(" Memory", Style::default().fg(Color::Yellow))];
        let percentage = data.used as f32 / data.total as f32 * 100.0;
        let text = format!(
            "{:.1}G/{:.1}G",
            data.used as f32 / BYTES_PER_GB as f32,
            data.total as f32 / BYTES_PER_GB as f32
        );
        let mem_bar_width = (area.width / 2) - 12;
        spans.extend(percentage_bar(mem_bar_width, percentage, &text));

        // swap
        spans.extend(vec![Span::styled(
            "    Swap",
            Style::default().fg(Color::Yellow),
        )]);
        let percentage = data.used_swap as f32 / data.total_swap as f32 * 100.0;
        let text = format!(
            "{:.1}G/{:.1}G",
            data.used_swap as f32 / BYTES_PER_GB as f32,
            data.total_swap as f32 / BYTES_PER_GB as f32
        );
        let swap_bar_width = area.width - mem_bar_width - 26;
        spans.extend(percentage_bar(swap_bar_width, percentage, &text));

        Paragraph::new(Line::from(spans))
            .left_aligned()
            .render(area, buf);
    }
}
