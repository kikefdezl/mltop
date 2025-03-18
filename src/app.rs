use std::io;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use crate::config::REFRESH_RATE_MILLIS;
use crate::data::data::AppData;
use crate::event::Event;
use crate::widgets::cpu::CpuWidget;
use crate::widgets::gpu::{GpuWidget, GPU_WIDGET_HEIGHT};
use crate::widgets::line_graph::LineGraphWidget;
use crate::widgets::memory::{MemoryWidget, MEMORY_WIDGET_HEIGHT};
use crate::widgets::processes::ProcessesWidget;

use crossterm::event::{
    self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    DefaultTerminal,
};
use std::time::Duration;

pub struct App {
    terminal: DefaultTerminal,
    data: AppData,
    exit: bool,
    refresh_rate: Duration,
}

impl App {
    pub fn new() -> App {
        let terminal = ratatui::init();
        let data = AppData::new();
        let refresh_rate = Duration::from_millis(REFRESH_RATE_MILLIS);
        App {
            terminal,
            data,
            exit: false,
            refresh_rate,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.data.update();
        self.render();
        let (tx, rx) = mpsc::channel();

        let event_tx = tx.clone();
        thread::spawn(move || loop {
            if event::poll(Duration::from_millis(100)).unwrap() {
                if let Ok(evt) = event::read() {
                    event_tx.send(Event::Crossterm(evt)).unwrap();
                };
            }
        });

        let duration = self.refresh_rate;
        let event_tx = tx.clone();
        thread::spawn(move || loop {
            thread::sleep(duration);
            event_tx.send(Event::Render).unwrap();
        });

        while !self.exit {
            let _ = self.handle_events(&rx);
        }
        Ok(())
    }

    fn handle_events(&mut self, rx: &Receiver<Event>) -> io::Result<()> {
        match rx.recv().unwrap() {
            Event::Crossterm(evt) => self.handle_crossterm_event(evt)?,
            Event::Render => self.handle_render_event()?,
        }
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
        let cpu_widget = CpuWidget::new(self.data.cpu.clone());
        let memory_widget = MemoryWidget::new(self.data.memory.clone());
        let line_graph_widget = LineGraphWidget::new(self.data.cpu.clone(), self.data.gpu.clone());
        let gpu_widget = GpuWidget::new(self.data.gpu.clone());
        let processes_widget = ProcessesWidget::new(self.data.processes.clone());

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
