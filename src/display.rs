use crate::cpu::Cpu;
use crate::memory::Memory;

const BYTES_PER_GB: u64 = 1024_u64.pow(3);

pub struct DisplayManager {
    pub cpu: Cpu,
    pub memory: Memory,
}

impl DisplayManager {
    pub fn display(&self) {
        self.display_cpu();
        self.display_memory();
    }

    fn display_cpu(&self) {
        println!("CPU: {}%", self.cpu.usage);
        for i in 0..self.cpu.cores.len() {
            println!(
                "CPU {}: {:.2}% {}°C",
                i, self.cpu.cores[i].usage, self.cpu.cores[i].temp
            );
        }
    }

    fn display_memory(&self) {
        println!(
            "Memory Used: {:.1} GB",
            self.memory.used as f64 / BYTES_PER_GB as f64
        );
        println!(
            "Memory Total: {:.1} GB",
            (self.memory.total as f64 / BYTES_PER_GB as f64)
        );
        println!(
            "Used: {:.2} %",
            self.memory.used as f64 / self.memory.total as f64
        );
    }
}
