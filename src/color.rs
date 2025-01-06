pub fn black_text(input: &str) -> String {
    format!("\x1b[30m{}\x1b[0m", input)
}

pub fn red_text(input: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", input)
}

pub fn green_text(input: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", input)
}

pub fn yellow_text(input: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", input)
}

pub fn blue_text(input: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", input)
}

pub fn purple_text(input: &str) -> String {
    format!("\x1b[35m{}\x1b[0m", input)
}

pub fn cyan_text(input: &str) -> String {
    format!("\x1b[36m{}\x1b[0m", input)
}

pub fn white_text(input: &str) -> String {
    format!("\x1b[37m{}\x1b[0m", input)
}

pub fn orange_text(input: &str) -> String {
    format!("\x1b[38;5;214m{}\x1b[0m", input)
}
