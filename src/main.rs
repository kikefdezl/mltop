use cpu::Cpu;
use display_data::DisplayData;
use display_manager::DisplayManager;
use memory::Memory;

use std::{thread::sleep, time::Duration};

mod config;
mod cpu;
mod display_data;
mod display_manager;
mod memory;
mod utils;

fn main() {
    let duration = Duration::from_millis(config::REFRESH_RATE_MILLIS);
    loop {
        print!("\x1B[2J\x1B[1;1H"); // clear the display

        let cpu = Cpu::read();
        let memory = Memory::read();
        let display_data = DisplayData::get();
        let display = DisplayManager {
            cpu,
            memory,
            data: display_data,
        };

        display.display();
        sleep(duration);
    }
}
