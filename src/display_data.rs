use terminal_size::{terminal_size, Height, Width};

pub struct DisplayData {
    pub width: u16,
    pub height: u16,
}

impl DisplayData {
    pub fn get() -> DisplayData {
        let size = terminal_size();
        match size {
            None => DisplayData {
                width: 80,
                height: 24,
            },
            Some((Width(w), Height(h))) => DisplayData {
                width: w,
                height: h,
            },
        }
    }
}
