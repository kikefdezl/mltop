use ratatui::{
    style::{Modifier, Style},
    text::Span,
};

use ratatui::prelude::*;

use crate::config::get_config;

const BRACKET_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);

pub fn percentage_bar<'a>(width: u16, perc: f32, text: &str) -> Vec<Span<'a>> {
    const HIGH_THRESHOLD: f32 = 90.0;
    const MEDIUM_HIGH_THRESHOLD: f32 = 70.0;
    const MEDIUM_THRESHOLD: f32 = 40.0;

    let theme = &get_config().theme;

    let perc = perc.clamp(0.0, 100.0);

    let mut text = text.to_string();
    let mut text_width = text.chars().count();
    if text_width > width as usize {
        text = text.chars().take(width as usize).collect();
        text_width = width as usize;
    }

    let width_usize = width as usize;
    let full_width_usize = ((width_usize as f32) * (perc / 100.0)).round() as usize;
    let bar_width_usize = full_width_usize.min(width_usize.saturating_sub(text_width));

    let color = match perc {
        p if p > HIGH_THRESHOLD => theme.bar_high_use,
        p if p > MEDIUM_HIGH_THRESHOLD => theme.bar_medium_high_use,
        p if p > MEDIUM_THRESHOLD => theme.bar_medium_use,
        _ => theme.bar_low_use,
    };

    let bar = "|".repeat(bar_width_usize);
    let empty_len = width_usize
        .saturating_sub(text_width)
        .saturating_sub(bar_width_usize);
    let empty = " ".repeat(empty_len);

    let colored_text_width = full_width_usize.saturating_sub(bar_width_usize);
    let colored_text = text.chars().take(colored_text_width).collect::<String>();
    let rest_text = text.chars().skip(colored_text_width).collect::<String>();

    let mut spans = vec![Span::styled("[".to_string(), BRACKET_STYLE)];
    spans.push(Span::styled(bar, Style::default().fg(color)));
    spans.push(Span::raw(empty));
    spans.push(Span::styled(colored_text, Style::default().fg(color)));
    spans.push(Span::styled(
        rest_text,
        Style::default().fg(theme.bar_text).dim(),
    ));

    spans.push(Span::styled("]".to_string(), BRACKET_STYLE));
    spans
}
