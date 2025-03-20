use app::App;
use std::io;
use tokio;

mod app;
mod config;
mod constants;
mod data;
mod event;
mod utils;
mod widgets;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut app = App::new();
    let result = app.run().await;
    ratatui::restore();
    result
}
