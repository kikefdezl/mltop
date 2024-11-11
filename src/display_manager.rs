use crate::cpu::Cpu;
use crate::display_data::DisplayData;
use crate::memory::Memory;
use crate::utils;

const BYTES_PER_GB: u64 = 1024_u64.pow(3);

pub struct DisplayManager {
    pub cpu: Cpu,
    pub memory: Memory,
    pub data: DisplayData,
}

impl DisplayManager {
    pub fn display(&self) {
        self.display_cpu();
        self.display_memory();
    }

    fn display_cpu(&self) {
        println!("CPU Total: {:.2}%", self.cpu.usage);
        let num_cpus = self.cpu.cores.len();
        let cpu_rows = utils::fast_int_sqrt(num_cpus);
        let mut cpu_cols = 0;
        while cpu_cols * cpu_rows < num_cpus {
            cpu_cols += 1;
        }

        let core_width = self.data.width as usize / cpu_cols;

        for r in 0..cpu_rows {
            for c in 0..cpu_cols {
                let i = c * cpu_rows + r;
                let s = format!(
                    "CPU {}: {:.2}% {}°C",
                    i, self.cpu.cores[i].usage, self.cpu.cores[i].temp
                );
                print!("{s:<core_width$}", core_width = core_width);
            }
            println!("");
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
            (self.memory.used as f64 / self.memory.total as f64) * 100.0
        );
    }
}
