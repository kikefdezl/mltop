use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

const FOOTER: [(&str, &str); 4] = [
    ("F4", "Filter"),
    ("F5", "Threads"),
    ("F6", "SortBy"),
    ("F9", "Kill"),
];
const HIGHLIGHT_STYLE: Style = Style::new().bg(Color::White).fg(Color::Black);
const MESSAGE_STYLE: Style = Style::new().bg(Color::Red).fg(Color::Black);

pub struct ActionBarWidget {}

impl ActionBarWidget {
    pub fn new() -> ActionBarWidget {
        ActionBarWidget {}
    }

    pub fn render(
        &self,
        area: Rect,
        buf: &mut Buffer,
        message: Option<&str>,
        filter_by: Option<&str>,
    ) {
        let mut spans: Vec<Span> = FOOTER
            .iter()
            .flat_map(|f| {
                vec![
                    Span::raw(format!(" {}", f.0)),
                    Span::styled(f.1, HIGHLIGHT_STYLE),
                ]
            })
            .collect();

        if let Some(s) = filter_by {
            spans.push(Span::raw(format!(" Filter: {}", s)));
        };

        let used_width: usize = spans.iter().map(|s| s.content.len()).sum();
        let message_width: usize = match message {
            None => 0,
            Some(m) => m.len() + 2,
        };
        let fill_width = area
            .width
            .saturating_sub(used_width as u16)
            .saturating_sub(message_width as u16);
        spans.push(Span::styled(
            " ".repeat(fill_width as usize),
            HIGHLIGHT_STYLE,
        ));
        if let Some(m) = message {
            spans.push(Span::styled(format!(" {} ", m), MESSAGE_STYLE));
        }

        Paragraph::new(Line::from(spans))
            .block(Block::default())
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}
