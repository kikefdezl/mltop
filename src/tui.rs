use std::sync::mpsc::{self, Sender};
use std::time::Duration;
use std::{io, thread};

use crate::config::REFRESH_RATE_MILLIS;
use crate::data::collector::Collector;
use crate::data::update_kind::DataUpdateKind;
use crate::data::{Data, DataStore};
use crate::event::Event;
use crate::message_bus::MessageBus;
use crate::state::State;
use crate::widgets::gpu::GPU_WIDGET_HEIGHT;
use crate::widgets::memory::MEMORY_WIDGET_HEIGHT;
use crate::widgets::process_table::ProcessTableWidget;
use crate::widgets::Widgets;

use crossterm::event::{
    self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    DefaultTerminal,
};

pub struct Tui {
    collector: Collector,
    data: Data,
    data_store: DataStore,
    exit: bool,
    message_bus: MessageBus,
    refresh_rate_ms: u64,
    state: State,
    terminal: DefaultTerminal,
    widgets: Widgets,
}

impl Tui {
    pub fn new() -> Tui {
        let mut message_bus = MessageBus::new();

        let mut collector = Collector::new();
        let data = Data::new_from_snapshot(collector.collect(&DataUpdateKind::all()));

        if !collector.can_read_gpu() {
            message_bus.send("No GPU found.".to_string())
        }

        Tui {
            collector,
            data,
            data_store: DataStore::new(),
            exit: false,
            message_bus,
            refresh_rate_ms: REFRESH_RATE_MILLIS,
            state: State::new(),
            terminal: ratatui::init(),
            widgets: Widgets::new(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.update_data();
        self.render();
        let (tx, rx) = mpsc::channel();

        Tui::spawn_crossterm_event_thread(tx.clone(), 200)?;
        Tui::spawn_render_event_thread(tx.clone(), self.refresh_rate_ms)?;

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
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.modifiers {
            KeyModifiers::NONE => match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Down | KeyCode::Char('j') => self.move_down(),
                KeyCode::Up | KeyCode::Char('k') => self.move_up(),
                KeyCode::Esc => self.deactivate(),
                KeyCode::F(5) => self.toggle_threads(),
                KeyCode::F(6) => self.toggle_sort_by(),
                KeyCode::F(9) => self.kill_process(),
                KeyCode::Char('t') => self.toggle_threads(),
                _ => {}
            },
            KeyModifiers::CONTROL => match key_event.code {
                KeyCode::Char('c') => self.exit(),
                _ => {}
            },
            KeyModifiers::SHIFT => match key_event.code {
                KeyCode::Char('G') => self.go_to_last(),
                _ => {}
            },
            _ => {}
        }
    }

    fn handle_render_event(&mut self) -> io::Result<()> {
        self.message_bus.check();
        self.update_data();
        self.render();
        Ok(())
    }

    fn render(&mut self) {
        let _ = self.terminal.draw(|frame| {
            let mut constraints = vec![
                Constraint::Length(self.widgets.cpu().grid_dimensions(&self.data.cpu).0 + 1),
                Constraint::Length(MEMORY_WIDGET_HEIGHT),
                Constraint::Max(20),
            ];
            if self.collector.can_read_gpu() {
                constraints.push(Constraint::Length(GPU_WIDGET_HEIGHT));
            }
            constraints.push(Constraint::Min(0));

            let areas = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(frame.area());

            self.widgets
                .cpu()
                .render(areas[0], frame.buffer_mut(), &self.data.cpu);
            self.widgets
                .memory()
                .render(areas[1], frame.buffer_mut(), &self.data.memory);
            self.widgets
                .line_graph()
                .render(areas[2], frame.buffer_mut(), &self.data_store);
            if self.data.has_gpu() {
                self.widgets.gpu().render(
                    areas[3],
                    frame.buffer_mut(),
                    &self.data.gpu.clone().unwrap(), // TODO: Check if can avoid clone
                );
            }

            // take the remaining area and split it for the table of
            // processes and the action bar.
            // They are only rendered if there's enough vertical space
            let remaining_areas = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(*areas.last().unwrap());

            self.widgets.process_table().render(
                remaining_areas[0],
                frame.buffer_mut(),
                &mut self.state.process_table,
                &self.data.processes,
            );
            self.widgets.action_bar.render(
                remaining_areas[1],
                frame.buffer_mut(),
                self.message_bus.read(),
            );
        });
    }

    fn deactivate(&mut self) {
        self.state.deactivate_table();
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
        if let Some(selected_row) = self.state.selected_row() {
            if let Some(pid) = ProcessTableWidget::get_nth_pid(
                self.data.processes.clone(),
                &self.state.process_table,
                selected_row,
            ) {
                self.collector.kill_process(pid as usize);
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

        let data_snapshot = self.collector.collect(&update_kind);
        self.data_store.save(data_snapshot.clone());
        self.data.update_from_snapshot(data_snapshot);
    }
}
