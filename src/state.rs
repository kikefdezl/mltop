use ratatui::widgets::TableState;

pub struct State {
    pub processes: TableState,
}

impl State {
    pub fn new() -> State {
        State {
            processes: TableState::default(),
        }
    }

    pub fn active(&self) -> bool {
        self.processes.selected().is_some()
    }

    pub fn activate(&mut self) {
        self.processes.select(Some(0))
    }

    pub fn deactivate(&mut self) {
        self.processes.select(None)
    }

    pub fn move_down(&mut self) {
        match self.processes.selected() {
            None => self.activate(),
            Some(s) => self.processes.select(Some(s + 1)),
        };
    }

    pub fn move_up(&mut self) {
        match self.processes.selected() {
            None => self.activate(),
            Some(s) => {
                if s <= 0 {
                    self.processes.select(Some(s))
                } else {
                    self.processes.select(Some(s - 1))
                }
            }
        }
    }
}
