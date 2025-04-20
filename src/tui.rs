use std::sync::mpsc::{self, Sender};
use std::time::Duration;
use std::{io, thread};

use crate::config::REFRESH_RATE_MILLIS;
use crate::data::collector::Collector;
use crate::data::update_kind::DataUpdateKind;
use crate::event::Event;
use crate::message_bus::MessageBus;
use crate::state::State;
use crate::widgets::action_bar::ActionBarWidget;
use crate::widgets::cpu::CpuWidget;
use crate::widgets::gpu::{GpuWidget, GPU_WIDGET_HEIGHT};
use crate::widgets::line_graph::LineGraphWidget;
use crate::widgets::memory::{MemoryWidget, MEMORY_WIDGET_HEIGHT};
use crate::widgets::process_table::ProcessTableWidget;

use crossterm::event::{
    self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    DefaultTerminal,
};

pub struct Tui {
    terminal: DefaultTerminal,
    collector: Collector,
    state: State,
    message_bus: MessageBus,
    exit: bool,
    refresh_rate_ms: u64,
}

impl Tui {
    pub fn new() -> Tui {
        let mut message_bus = MessageBus::new();

        let data = Collector::new();
        if !data.has_gpu() {
            message_bus.send("No GPU found.".to_string())
        }

        Tui {
            terminal: ratatui::init(),
            collector: data,
            state: State::new(),
            message_bus,
            exit: false,
            refresh_rate_ms: REFRESH_RATE_MILLIS,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
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
        let cpu_widget = CpuWidget::new(&self.collector.cpu);
        let memory_widget = MemoryWidget::new(&self.collector.memory);
        let line_graph_widget = LineGraphWidget::new(&self.collector.cpu, &self.collector.gpu);
        let gpu_widget = GpuWidget::new(&self.collector.gpu);
        let processes_widget = ProcessTableWidget::new(&self.collector.processes);
        let action_bar_widget = ActionBarWidget::new(self.message_bus.read());

        let _ = self.terminal.draw(|frame| {
            let mut constraints = vec![
                Constraint::Length(cpu_widget.grid_dimensions().0 + 1),
                Constraint::Length(MEMORY_WIDGET_HEIGHT),
                Constraint::Max(20),
            ];
            if self.collector.has_gpu() {
                constraints.push(Constraint::Length(GPU_WIDGET_HEIGHT));
            }
            constraints.push(Constraint::Min(0));

            let areas = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(frame.area());

            frame.render_widget(cpu_widget, areas[0]);
            frame.render_widget(memory_widget, areas[1]);
            frame.render_widget(line_graph_widget, areas[2]);
            if self.collector.has_gpu() {
                frame.render_widget(gpu_widget, areas[3]);
            }

            // take the remaining area and split it for the table of
            // processes and the action bar.
            // They are only rendered if there's enough vertical space
            let remaining_areas = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(*areas.last().unwrap());

            frame.render_stateful_widget(
                processes_widget,
                remaining_areas[0],
                &mut self.state.process_table,
            );
            frame.render_widget(action_bar_widget, remaining_areas[1]);
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
        let n = self.collector.processes.len() - 1;
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
                self.collector.processes.into_vec(),
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
        self.collector.update(&update_kind);
    }
}
