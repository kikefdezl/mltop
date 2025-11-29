use std::io;

use mltop::config::init_config;
use mltop::tui::Tui;

fn main() -> io::Result<()> {
    init_config();
    let mut app = Tui::new();
    let result = app.run();
    ratatui::restore();
    app.render();
    result
}
