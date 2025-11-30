use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::config::get_config;

const FOOTER: [(&str, &str); 4] = [
    ("F4", "Filter"),
    ("F5", "Threads"),
    ("F6", "SortBy"),
    ("F9", "Kill"),
];

pub struct ActionBarWidget<'a> {
    pub message: Option<&'a str>,
    pub filter_by: Option<&'a str>,
}

impl<'a> Widget for ActionBarWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = &get_config().theme;

        let key_style: Style = Style::new()
            .bg(theme.action_bar_key_bg)
            .fg(theme.action_bar_key_fg);
        let cmd_style: Style = Style::new()
            .bg(theme.action_bar_cmd_bg)
            .fg(theme.action_bar_cmd_fg);

        let mut spans: Vec<Span> = FOOTER
            .iter()
            .flat_map(|f| {
                vec![
                    Span::styled(format!(" {}", f.0), key_style),
                    Span::styled(f.1, cmd_style),
                ]
            })
            .collect();

        if let Some(s) = self.filter_by {
            spans.push(Span::raw(format!(" Filter: {} ", s)));
        };

        let used_width: usize = spans.iter().map(|s| s.content.len()).sum();
        let message_width: usize = match self.message {
            None => 0,
            Some(m) => m.len() + 2,
        };
        let fill_width = area
            .width
            .saturating_sub(used_width as u16)
            .saturating_sub(message_width as u16);
        spans.push(Span::styled(" ".repeat(fill_width as usize), cmd_style));
        if let Some(m) = self.message {
            spans.push(Span::styled(
                format!(" {} ", m),
                Style::new()
                    .bg(theme.action_bar_msg_bg)
                    .fg(theme.action_bar_msg_fg),
            ));
        }

        Paragraph::new(Line::from(spans))
            .block(Block::default())
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}
