use cpu::Cpu;
use display_manager::DisplayManager;
use memory::Memory;
use terminal_data::TerminalData;

use std::{thread::sleep, time::Duration};

mod config;
mod cpu;
mod display_manager;
mod memory;
mod terminal_data;
mod utils;

fn main() {
    let duration = Duration::from_millis(config::REFRESH_RATE_MILLIS);
    loop {
        print!("\x1B[2J\x1B[1;1H"); // clear the display

        let cpu = Cpu::read();
        let memory = Memory::read();
        let term_data = TerminalData::get();
        let display = DisplayManager {
            cpu,
            memory,
            term_data,
        };

        display.display();
        sleep(duration);
    }
}
