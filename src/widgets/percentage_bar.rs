use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

const BRACKET_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);

pub fn percentage_bar<'a>(width: u16, perc: f32, text: &str) -> Vec<Span<'a>> {
    let mut spans = vec![Span::styled("[".to_string(), BRACKET_STYLE)];

    let color: Color = if perc > 90.0 {
        Color::Red
    } else if perc > 70.0 {
        Color::Rgb(255, 130, 0) // orange
    } else if perc > 40.0 {
        Color::Yellow
    } else {
        Color::Green
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

    let grey_text = text
        .get(colored_text_width as usize..)
        .unwrap_or("")
        .to_string();
    spans.push(Span::styled(
        grey_text,
        Style::default().fg(Color::DarkGray),
    ));

    spans.push(Span::styled("]".to_string(), BRACKET_STYLE));

    spans
}
