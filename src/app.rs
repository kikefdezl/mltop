use std::io;

use crate::config::REFRESH_RATE_MILLIS;
use crate::data::data::AppData;
use crate::widgets::cpu::CpuWidget;
use crate::widgets::gpu::{GpuWidget, GPU_WIDGET_HEIGHT};
use crate::widgets::line_graph::LineGraphWidget;
use crate::widgets::memory::{MemoryWidget, MEMORY_WIDGET_HEIGHT};
use crate::widgets::processes::ProcessesWidget;
use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    DefaultTerminal, Frame,
};
use std::time::Duration;

pub struct App {
    data: AppData,
    exit: bool,
    refresh_rate_ms: u64,
}

impl App {
    pub fn new() -> App {
        let data = AppData::new();

        App {
            data,
            exit: false,
            refresh_rate_ms: REFRESH_RATE_MILLIS,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            self.data.update();
            terminal.draw(|frame| {
                self.draw(frame);
            })?;
            if poll(Duration::from_millis(self.refresh_rate_ms))? {
                self.handle_events()?;
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
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

    fn draw(&self, frame: &mut Frame) {
        let cpu_widget = CpuWidget::new(self.data.cpu.clone());
        let memory_widget = MemoryWidget::new(self.data.memory.clone());
        let line_graph_widget = LineGraphWidget::new(self.data.cpu.clone(), self.data.gpu.clone());
        let gpu_widget = GpuWidget::new(self.data.gpu.clone());
        let processes_widget = ProcessesWidget::new(self.data.processes.clone());

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
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}
