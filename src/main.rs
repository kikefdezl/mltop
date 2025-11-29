use std::io;

use mltop::tui::Tui;
use mltop::config::init_config;

fn main() -> io::Result<()> {
    init_config();
    let mut app = Tui::new();
    let result = app.run();
    ratatui::restore();
    app.render();
    result
}
