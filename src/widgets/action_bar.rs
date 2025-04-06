use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

const FOOTER: [(&str, &str); 3] = [("F6", "SortBy"), ("F9", "SIGKILL"), ("F12", "SIGTERM")];
const HIGHLIGHT_STYLE: Style = Style::new().bg(Color::White).fg(Color::Black);
const MESSAGE_STYLE: Style = Style::new().bg(Color::Red).fg(Color::Black);

pub struct ActionBarWidget<'a> {
    message: Option<&'a str>,
}

impl Widget for ActionBarWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut spans: Vec<Span> = FOOTER
            .iter()
            .flat_map(|f| {
                vec![
                    Span::raw(format!(" {}", f.0)),
                    Span::styled(f.1, HIGHLIGHT_STYLE),
                ]
            })
            .collect();

        let used_width: usize = spans.iter().map(|s| s.content.len()).sum();
        let message_width: usize = match self.message {
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
        if let Some(m) = self.message {
            spans.push(Span::styled(format!(" {} ", m), MESSAGE_STYLE));
        }

        Paragraph::new(Line::from(spans))
            .block(Block::default())
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}

impl ActionBarWidget<'_> {
    pub fn new<'a>(message: Option<&'a str>) -> ActionBarWidget<'a> {
        ActionBarWidget { message }
    }
}
