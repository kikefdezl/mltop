use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::config::get_config;

const BRACKET_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);

pub fn percentage_bar<'a>(width: u16, perc: f32, text: &str) -> Vec<Span<'a>> {
    let theme = &get_config().theme;

    let mut spans = vec![Span::styled("[".to_string(), BRACKET_STYLE)];

    let color: Color = if perc > 90.0 {
        theme.bar_high_use
    } else if perc > 70.0 {
        theme.bar_medium_high_use
    } else if perc > 40.0 {
        theme.bar_medium_use
    } else {
        theme.bar_low_use
    };

    let full_width = (width as f32 * (perc / 100.0)).round() as u16;
    let text_width = text.chars().count() as u16;
    let bar_width = std::cmp::min(full_width, width - text_width);

    let bar = (0..bar_width).map(|_| "|").collect::<String>();
    spans.push(Span::styled(bar, Style::default().fg(color)));
    let empty = (bar_width..(width - text_width))
        .map(|_| " ")
        .collect::<String>();
    spans.push(Span::raw(empty));

    let colored_text_width = text_width.saturating_sub(width - full_width);
    let colored_text = text
        .get(..colored_text_width as usize)
        .unwrap_or("")
        .to_string();
    spans.push(Span::styled(colored_text, Style::default().fg(color)));

    let rest_text = text
        .get(colored_text_width as usize..)
        .unwrap_or("")
        .to_string();
    spans.push(Span::styled(
        rest_text,
        Style::default()
            .fg(get_config().theme.bar_text)
            .add_modifier(Modifier::DIM),
    ));

    spans.push(Span::styled("]".to_string(), BRACKET_STYLE));

    spans
}
