use app::App;
use std::io;

mod app;
mod config;
mod constants;
mod data;
mod utils;
mod widgets;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let result = app.run(&mut terminal);
    ratatui::restore();
    result
}
