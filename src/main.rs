use app::App;
use std::io;

mod app;
mod config;
mod constants;
mod data;
mod event;
mod utils;
mod widgets;

fn main() -> io::Result<()> {
    let mut app = App::new();
    let result = app.run();
    ratatui::restore();
    result
}
