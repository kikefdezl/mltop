use std::io;

use mltop::tui::Tui;

fn main() -> io::Result<()> {
    let mut app = Tui::new();
    let result = app.run();
    ratatui::restore();
    app.render();
    result
}
