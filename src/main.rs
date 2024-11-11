use cpu::Cpu;
use display::DisplayManager;
use memory::Memory;

use std::{thread::sleep, time::Duration};

mod config;
mod cpu;
mod display;
mod memory;

fn main() {
    let duration = Duration::from_millis(config::REFRESH_RATE_MILLIS);
    loop {
        let cpu = Cpu::read();
        let memory = Memory::read();
        let display = DisplayManager { cpu, memory };

        display.display();

        sleep(duration);
        print!("\x1B[2J\x1B[1;1H");
    }
}
