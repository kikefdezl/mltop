use crate::widgets::state::process_table::ProcessTableState;

pub struct State {
    pub process_table: ProcessTableState,
}

impl State {
    pub fn new() -> State {
        State {
            process_table: ProcessTableState::default(),
        }
    }

    pub fn table_is_active(&self) -> bool {
        self.process_table.is_active()
    }

    pub fn deactivate_table(&mut self) {
        self.process_table.deactivate()
    }

    pub fn move_down(&mut self) {
        self.process_table.move_down()
    }

    pub fn move_up(&mut self) {
        self.process_table.move_up()
    }

    pub fn selected_row(&self) -> Option<usize> {
        self.process_table.selected_row()
    }

    pub fn toggle_sort_by(&mut self) {
        self.process_table.toggle_sort_by();
    }

    pub fn toggle_show_threads(&mut self) {
        self.process_table.toggle_show_threads();
    }
}
