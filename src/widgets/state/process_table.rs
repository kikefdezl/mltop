use ratatui::widgets::TableState;

#[derive(Clone)]
pub enum ProcessesSortBy {
    CPU,
    MEM,
}

impl ProcessesSortBy {
    fn default() -> ProcessesSortBy {
        Self::CPU
    }
}

pub struct ProcessTableState {
    pub sort_by: ProcessesSortBy,
    pub show_threads: bool,
    pub ratatui_table_state: TableState,
}

impl ProcessTableState {
    pub fn default() -> ProcessTableState {
        ProcessTableState {
            sort_by: ProcessesSortBy::default(),
            show_threads: false,
            ratatui_table_state: TableState::default(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.ratatui_table_state.selected().is_some()
    }

    pub fn activate(&mut self) {
        self.ratatui_table_state.select(Some(0))
    }

    pub fn deactivate(&mut self) {
        self.ratatui_table_state.select(None)
    }

    pub fn move_down(&mut self) {
        match self.ratatui_table_state.selected() {
            None => self.activate(),
            Some(s) => self.ratatui_table_state.select(Some(s + 1)),
        };
    }

    pub fn move_up(&mut self) {
        match self.ratatui_table_state.selected() {
            None => self.activate(),
            Some(s) => {
                if s <= 0 {
                    self.ratatui_table_state.select(Some(s))
                } else {
                    self.ratatui_table_state.select(Some(s - 1))
                }
            }
        }
    }

    pub fn selected_row(&self) -> Option<usize> {
        self.ratatui_table_state.selected()
    }

    pub fn toggle_sort_by(&mut self) {
        self.sort_by = match self.sort_by {
            ProcessesSortBy::CPU => ProcessesSortBy::MEM,
            ProcessesSortBy::MEM => ProcessesSortBy::CPU,
        }
    }

    pub fn toggle_show_threads(&mut self) {
        self.show_threads = !self.show_threads;
    }
}
