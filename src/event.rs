use crossterm::event::Event as CrosstermEvent;

pub enum Event {
    Crossterm(CrosstermEvent),
    Render,
}
