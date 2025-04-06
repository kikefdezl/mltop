use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

const FOOTER: [(&str, &str); 3] = [("F6", "SortBy"), ("F9", "SIGKILL"), ("F12", "SIGTERM")];

pub struct ActionBarWidget {}

impl Widget for ActionBarWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let highlight_style = Style::new().bg(Color::White).fg(Color::Black);
        let mut spans: Vec<Span> = FOOTER
            .iter()
            .flat_map(|f| {
                vec![
                    Span::raw(format!(" {}", f.0)),
                    Span::styled(f.1, highlight_style),
                ]
            })
            .collect();

        let used_width: usize = spans.iter().map(|s| s.content.len()).sum();
        let remaining_width = area.width.saturating_sub(used_width as u16);
        spans.push(Span::styled(
            " ".repeat(remaining_width as usize),
            highlight_style,
        ));

        <Paragraph as ratatui::widgets::Widget>::render(
            Paragraph::new(Line::from(spans))
                .block(Block::default())
                .alignment(Alignment::Left),
            area,
            buf,
        );
    }
}

impl ActionBarWidget {
    pub fn new() -> ActionBarWidget {
        ActionBarWidget {}
    }
}
