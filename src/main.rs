use devices::cpu::Cpu;
use devices::gpu::Gpu;
use devices::memory::Memory;
use display_manager::DisplayManager;
use processes::Processes;
use std::io;
use std::io::{stdout, Write};
use std::time::Instant;
use terminal_data::TerminalData;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::time::Duration;

mod color;
mod config;
mod devices;
mod display_manager;
mod processes;
mod terminal_data;
mod utils;

fn main() -> io::Result<()> {
    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let mut gpu = match Gpu::read() {
        Err(_) => None,
        Ok(gpu) => Some(gpu),
    };

    let mut last_tick = Instant::now() - Duration::from_secs(2);
    loop {
        if last_tick.elapsed() >= Duration::from_millis(config::REFRESH_RATE_MILLIS) {
            let cpu = Cpu::read();
            let memory = Memory::read();
            let processes = match Processes::read() {
                Err(_) => None,
                Ok(processes) => Some(processes),
            };
            let term_data = TerminalData::get();

            match &mut gpu {
                None => {}
                Some(gpu) => gpu.update(),
            }

            let display = DisplayManager {
                cpu: &cpu,
                memory: &memory,
                gpu: &gpu,
                processes: &processes,
                term_data: &term_data,
            };

            execute!(
                stdout,
                cursor::MoveTo(0, 0),
                terminal::Clear(ClearType::All)
            )?;
            display.display(&mut stdout);
            stdout.flush()?;

            if last_tick.elapsed() >= Duration::from_millis(config::REFRESH_RATE_MILLIS) {
                last_tick = Instant::now();
            }
        }

        // capture exit signals 'q' or 'C - c'
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('q')
                    || key_event.code == KeyCode::Char('c')
                        && key_event.modifiers.contains(event::KeyModifiers::CONTROL)
                {
                    break;
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, cursor::Show)?;

    Ok(())
}
