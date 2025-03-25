use crate::data::update_kind::DataUpdateKind;
use std::sync::mpsc::{self, Sender};
use std::time::Duration;
use std::{io, thread};

use crate::config::REFRESH_RATE_MILLIS;
use crate::data::data::Data;
use crate::event::Event;
use crate::state::State;
use crate::widgets::cpu::CpuWidget;
use crate::widgets::gpu::{GpuWidget, GPU_WIDGET_HEIGHT};
use crate::widgets::line_graph::LineGraphWidget;
use crate::widgets::memory::{MemoryWidget, MEMORY_WIDGET_HEIGHT};
use crate::widgets::table_of_processes::TableOfProcessesWidget;

use crossterm::event::{
    self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    DefaultTerminal,
};

pub struct Tui {
    terminal: DefaultTerminal,
    data: Data,
    state: State,
    exit: bool,
    refresh_rate_ms: u64,
}

impl Tui {
    pub fn new() -> Tui {
        Tui {
            terminal: ratatui::init(),
            data: Data::new(),
            state: State::new(),
            exit: false,
            refresh_rate_ms: REFRESH_RATE_MILLIS,
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
                KeyCode::F(9) => self.kill_process(),
                KeyCode::F(12) => self.terminate_process(),
                _ => {}
            },
            KeyModifiers::CONTROL => match key_event.code {
                KeyCode::Char('c') => self.exit(),
                _ => {}
            },
            _ => {}
        }
    }

    fn handle_render_event(&mut self) -> io::Result<()> {
        self.update_data();
        self.render();
        Ok(())
    }

    fn render(&mut self) {
        let cpu_widget = CpuWidget::new(&self.data.cpu);
        let memory_widget = MemoryWidget::new(&self.data.memory);
        let line_graph_widget = LineGraphWidget::new(&self.data.cpu, &self.data.gpu);
        let gpu_widget = GpuWidget::new(&self.data.gpu);
        let processes_widget = TableOfProcessesWidget::new(&self.data.processes);

        let _ = self.terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(cpu_widget.grid_dimensions().1 + 1),
                    Constraint::Length(MEMORY_WIDGET_HEIGHT),
                    Constraint::Max(20),
                    Constraint::Length(GPU_WIDGET_HEIGHT),
                    Constraint::Min(0),
                ])
                .split(frame.area());
            frame.render_widget(cpu_widget, layout[0]);
            frame.render_widget(memory_widget, layout[1]);
            frame.render_widget(line_graph_widget, layout[2]);
            frame.render_widget(gpu_widget, layout[3]);
            frame.render_stateful_widget(
                processes_widget,
                layout[4],
                &mut self.state.table_of_processes,
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

    fn move_down(&mut self) {
        self.state.move_down();
        self.render();
    }

    fn move_up(&mut self) {
        self.state.move_up();
        self.render();
    }

    fn terminate_process(&mut self) {
        if let Some(selected) = self.state.selected_row() {
            if let Some(process) = self.data.processes.get(selected) {
                self.data.terminate_process(process.pid as usize);
            }
        }
    }

    fn kill_process(&mut self) {
        if let Some(selected) = self.state.selected_row() {
            if let Some(process) = self.data.processes.get(selected) {
                self.data.kill_process(process.pid as usize);
            }
        }
    }

    fn update_data(&mut self) {
        // we don't update processes if the table is active, because
        // then it gets annoying to select the right row if the table
        // is refreshing while we move
        let update_kind = match self.state.table_is_active() {
            true => DataUpdateKind::all().without_processes(),
            false => DataUpdateKind::all(),
        };
        self.data.update(&update_kind);
    }
}
