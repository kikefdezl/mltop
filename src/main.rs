use crate::system::System;
use std::io;
use tui::Tui;

mod config;
mod constants;
mod data;
mod event;
mod message_bus;
mod state;
mod system;
mod tui;
mod utils;
mod widgets;

fn main() -> io::Result<()> {
    let mut app = Tui::<System>::new();
    let result = app.run();
    ratatui::restore();
    result
}
