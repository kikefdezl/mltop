use terminal_size::{terminal_size, Height, Width};

pub struct TerminalData {
    pub width: u16,
    pub height: u16,
}

impl TerminalData {
    pub fn get() -> TerminalData {
        let size = terminal_size();
        match size {
            None => TerminalData {
                width: 80,
                height: 24,
            },
            Some((Width(w), Height(h))) => TerminalData {
                width: w,
                height: h,
            },
        }
    }
}
