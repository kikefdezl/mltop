pub fn red_text(input: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", input)
}

pub fn yellow_text(input: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", input)
}

pub fn cyan_text(input: &str) -> String {
    format!("\x1b[36m{}\x1b[0m", input)
}

pub fn orange_text(input: &str) -> String {
    format!("\x1b[38;5;214m{}\x1b[0m", input)
}

pub fn gray_text(input: &str) -> String {
    format!("\x1b[90m{}\x1b[0m", input)
}
