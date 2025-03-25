use ratatui::widgets::TableState;

pub struct State {
    pub table_of_processes: TableState,
}

impl State {
    pub fn new() -> State {
        State {
            table_of_processes: TableState::default(),
        }
    }

    pub fn table_is_active(&self) -> bool {
        self.table_of_processes.selected().is_some()
    }

    pub fn activate_table(&mut self) {
        self.table_of_processes.select(Some(0))
    }

    pub fn deactivate_table(&mut self) {
        self.table_of_processes.select(None)
    }

    pub fn move_down(&mut self) {
        match self.table_of_processes.selected() {
            None => self.activate_table(),
            Some(s) => self.table_of_processes.select(Some(s + 1)),
        };
    }

    pub fn move_up(&mut self) {
        match self.table_of_processes.selected() {
            None => self.activate_table(),
            Some(s) => {
                if s <= 0 {
                    self.table_of_processes.select(Some(s))
                } else {
                    self.table_of_processes.select(Some(s - 1))
                }
            }
        }
    }
}
