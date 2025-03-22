use std::sync::mpsc::{self, Sender};
use std::time::Duration;
use std::{io, thread};

use crate::config::REFRESH_RATE_MILLIS;
use crate::data::data::AppData;
use crate::event::Event;
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

pub struct App {
    terminal: DefaultTerminal,
    data: AppData,
    exit: bool,
    refresh_rate_ms: u64,
}

impl App {
    pub fn new() -> App {
        let terminal = ratatui::init();
        let data = AppData::new();
        App {
            terminal,
            data,
            exit: false,
            refresh_rate_ms: REFRESH_RATE_MILLIS,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.data.update();
        self.render();
        let (tx, rx) = mpsc::channel();

        App::spawn_crossterm_event_thread(tx.clone(), 200)?;
        App::spawn_render_event_thread(tx.clone(), self.refresh_rate_ms)?;

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
        self.data.update();
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
                    Constraint::Length(cpu_widget.grid_dimensions().1),
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
            frame.render_widget(processes_widget, layout[4]);
        });
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
