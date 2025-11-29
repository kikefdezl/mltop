use std::io::Stdout;
use std::sync::mpsc::{self, Sender};
use std::time::Duration;
use std::{io, thread};

use crossterm::event::{
    self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use ratatui::prelude::{Backend, CrosstermBackend};
use ratatui::widgets::{StatefulWidget, Widget};
use ratatui::{
    backend::TestBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use crate::config::REFRESH_RATE_MILLIS;
use crate::data::store::{DataStore, StoredSnapshot};
use crate::data::system_data::SystemData;
use crate::data::update_kind::DataUpdateKind;
use crate::event::Event;
use crate::message_bus::MessageBus;
use crate::state::{Mode, State};
use crate::system::{FakeSystem, RealSystem, SystemMonitor};
use crate::widgets::action_bar::ActionBarWidget;
use crate::widgets::cpu::CpuWidget;
use crate::widgets::gpu::{GpuWidget, GPU_WIDGET_HEIGHT};
use crate::widgets::line_graph::LineGraphWidget;
use crate::widgets::memory::MemoryWidget;
use crate::widgets::memory::MEMORY_WIDGET_HEIGHT;
use crate::widgets::process_table::ProcessTableWidget;

pub struct Tui<S: SystemMonitor, B: Backend> {
    system: S,
    data: SystemData,
    data_store: DataStore,
    exit: bool,
    message_bus: MessageBus,
    refresh_rate_ms: u64,
    state: State,
    terminal: Terminal<B>,
}

impl Tui<RealSystem, CrosstermBackend<Stdout>> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Tui<RealSystem, CrosstermBackend<Stdout>> {
    fn default() -> Self {
        let mut message_bus = MessageBus::new();

        let mut system = RealSystem::default();
        let data = SystemData::new_from_snapshot(system.collect_snapshot(&DataUpdateKind::all()));

        if !system.gpu_available() {
            message_bus.send("No GPU found.".to_string())
        }

        Tui {
            system,
            data,
            data_store: DataStore::new(),
            exit: false,
            message_bus,
            refresh_rate_ms: REFRESH_RATE_MILLIS,
            state: State::new(),
            terminal: ratatui::init(),
        }
    }
}

impl<S: SystemMonitor, B: Backend> Tui<S, B> {
    pub fn run(&mut self) -> io::Result<()> {
        self.render();
        let (tx, rx) = mpsc::channel();

        Self::spawn_crossterm_event_thread(tx.clone(), 300)?;
        Self::spawn_render_event_thread(tx.clone(), self.refresh_rate_ms)?;

        while !self.exit {
            match rx.recv().unwrap() {
                Event::Crossterm(evt) => self.handle_crossterm_event(evt)?,
                Event::Render => self.handle_render_event()?,
            }
        }
        Ok(())
    }

    /// Spawns a thread that captures CrosstermEvents and re-emits them
    /// to the mpsc channel, wrapped into the Crossterm variant of the Event enum:
    /// Event::Crossterm<CrosstermEvent>
    fn spawn_crossterm_event_thread(tx: Sender<Event>, poll_rate: u64) -> io::Result<()> {
        thread::spawn(move || loop {
            if event::poll(Duration::from_millis(poll_rate)).unwrap() {
                tx.send(Event::Crossterm(event::read().unwrap())).unwrap();
            }
        });
        Ok(())
    }

    /// Spawns a thread that sends an Event::Render to the mpsc channel
    fn spawn_render_event_thread(tx: Sender<Event>, render_rate: u64) -> io::Result<()> {
        let duration = Duration::from_millis(render_rate);
        let custom_tx = tx.clone();
        thread::spawn(move || loop {
            thread::sleep(duration);
            custom_tx.send(Event::Render).unwrap();
        });
        Ok(())
    }

    fn handle_crossterm_event(&mut self, cross_evt: CrosstermEvent) -> io::Result<()> {
        match cross_evt {
            CrosstermEvent::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match self.state.mode {
                    Mode::Normal => self.handle_key_event_normal_mode(key_event),
                    Mode::Filter => self.handle_key_event_filter_mode(key_event),
                }
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event_normal_mode(&mut self, key_event: KeyEvent) {
        match key_event.modifiers {
            KeyModifiers::NONE => match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char('t') => self.toggle_threads(),
                KeyCode::Down | KeyCode::Char('j') => self.move_down(),
                KeyCode::Up | KeyCode::Char('k') => self.move_up(),
                KeyCode::F(4) | KeyCode::Char('/') => self.enter_filter_mode(),
                KeyCode::Esc => self.deactivate(),
                KeyCode::F(5) => self.toggle_threads(),
                KeyCode::F(6) => self.toggle_sort_by(),
                KeyCode::F(9) => self.kill_process(),
                _ => {}
            },
            KeyModifiers::CONTROL => {
                if let KeyCode::Char('c') = key_event.code {
                    self.exit()
                }
            }
            KeyModifiers::SHIFT => {
                if let KeyCode::Char('G') = key_event.code {
                    self.go_to_last()
                }
            }
            _ => {}
        }
    }

    fn handle_key_event_filter_mode(&mut self, key_event: KeyEvent) {
        match key_event.modifiers {
            KeyModifiers::NONE => match key_event.code {
                KeyCode::Esc => self.exit_filter_mode(),
                KeyCode::Char(c) => self.state.filter_by.push(c),
                KeyCode::Backspace => {
                    self.state.filter_by.pop();
                }
                _ => {}
            },
            KeyModifiers::SHIFT => {
                if let KeyCode::Char(c) = key_event.code {
                    self.state.filter_by.push(c)
                }
            }
            _ => {}
        }
        self.render()
    }

    fn handle_render_event(&mut self) -> io::Result<()> {
        self.message_bus.check();
        self.update_data();
        self.render();
        Ok(())
    }

    pub fn render(&mut self) {
        let _ = self.terminal.draw(|frame| {
            // -- build widgets --
            let cpu = CpuWidget {
                data: &self.data.cpu,
            };
            let memory = MemoryWidget {
                data: &self.data.memory,
            };
            let line_graph = LineGraphWidget {
                data: &self.data_store,
                max_gpu_mem: self.data.gpu.as_ref().map(|g| g.max_memory),
            };
            let gpu = self.data.gpu.as_ref().map(|gd| GpuWidget { data: gd });
            let filter_by = match self.state.mode {
                Mode::Filter => Some(self.state.filter_by.as_str()),
                _ => None,
            };
            let process_table = ProcessTableWidget {
                data: &self.data.processes,
                filter_by,
            };
            let action_bar = ActionBarWidget {
                message: self.message_bus.read(),
                filter_by,
            };

            // -- build layout --
            let mut constraints = vec![
                Constraint::Length(cpu.grid_dimensions().0 + 1),
                Constraint::Length(MEMORY_WIDGET_HEIGHT),
                Constraint::Max(20),
            ];
            if self.system.gpu_available() {
                constraints.push(Constraint::Length(GPU_WIDGET_HEIGHT));
            }
            constraints.push(Constraint::Min(0));

            let areas = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(frame.area());

            // take the remaining area and split it for the table of
            // processes and the action bar.
            // They are only rendered if there's enough vertical space
            let remaining_areas = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(*areas.last().unwrap());

            // -- render widgets --
            let buf = frame.buffer_mut();
            cpu.render(areas[0], buf);
            memory.render(areas[1], buf);
            line_graph.render(areas[2], buf);
            if let Some(g) = gpu {
                g.render(areas[3], buf);
            }
            process_table.render(remaining_areas[0], buf, &mut self.state.process_table);
            action_bar.render(remaining_areas[1], frame.buffer_mut());
        });
    }

    fn deactivate(&mut self) {
        self.state.deactivate_table();
        self.render();
    }

    fn enter_filter_mode(&mut self) {
        self.state.mode = Mode::Filter;
        self.render();
    }

    fn exit_filter_mode(&mut self) {
        self.state.mode = Mode::Normal;
        self.render();
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn go_to_last(&mut self) {
        let n = self.data.processes.processes.len() - 1;
        self.state.select_row(n);
        self.render();
    }

    fn move_down(&mut self) {
        self.state.move_down();
        self.render();
    }

    fn move_up(&mut self) {
        self.state.move_up();
        self.render();
    }

    fn toggle_sort_by(&mut self) {
        self.state.toggle_sort_by();
        self.deactivate();
    }

    fn toggle_threads(&mut self) {
        self.state.toggle_show_threads();
        self.deactivate();
    }

    fn kill_process(&mut self) {
        // TODO: Potentially the State could get out of sync with what is
        // reflected in the table, so this could kill the wrong PID.
        // A more robust solution is needed
        let filter_by = match self.state.mode {
            Mode::Filter => Some(self.state.filter_by.as_str()),
            _ => None,
        };
        if let Some(selected_row) = self.state.selected_row() {
            let table = ProcessTableWidget {
                data: &self.data.processes,
                filter_by,
            };
            if let Some(pid) = table.get_nth_pid(selected_row, &mut self.state.process_table) {
                self.system.kill_process(pid as usize);
                self.message_bus.send(format!("Killed pid {}", pid));
            }
        }
        self.deactivate();
    }

    fn update_data(&mut self) {
        // we don't update processes if the table is active, because
        // then it gets annoying to select the right row if the table
        // is refreshing while we move
        let update_kind = match self.state.table_is_active() {
            true => DataUpdateKind::all().without_processes(),
            false => DataUpdateKind::all(),
        };

        let data_snapshot = self.system.collect_snapshot(&update_kind);
        let stored = StoredSnapshot::from_data_snapshot(data_snapshot.clone());
        self.data_store.save(stored);
        self.data.update_from_snapshot(data_snapshot);
    }
}

// used for testing different hardware setups
impl Tui<FakeSystem, TestBackend> {
    pub fn fake(mut system: FakeSystem, backend: TestBackend) -> Self {
        let mut message_bus = MessageBus::new();

        let data = SystemData::new_from_snapshot(system.collect_snapshot(&DataUpdateKind::all()));

        if !system.gpu_available() {
            message_bus.send("No GPU found.".to_string())
        }

        Tui {
            system,
            data,
            data_store: DataStore::new(),
            exit: false,
            message_bus,
            refresh_rate_ms: REFRESH_RATE_MILLIS,
            state: State::new(),
            terminal: Terminal::new(backend).unwrap(),
        }
    }
}
