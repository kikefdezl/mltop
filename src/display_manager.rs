use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::terminal_data::TerminalData;
use crate::utils;

const BYTES_PER_GB: u64 = 1024_u64.pow(3);

pub struct DisplayManager {
    pub cpu: Cpu,
    pub memory: Memory,
    pub term_data: TerminalData,
}

impl DisplayManager {
    pub fn display(&self) {
        self.display_cpu();
        self.display_memory();
    }

    fn display_cpu(&self) {
        let total_bar = DisplayManager::percentage_bar(self.term_data.width - 14, self.cpu.usage);
        println!("CPU Total: {}", total_bar);
        let num_cpus = self.cpu.cores.len();
        let cpu_rows = utils::fast_int_sqrt(num_cpus);
        let mut cpu_cols = 0;
        while cpu_cols * cpu_rows < num_cpus {
            cpu_cols += 1;
        }

        let core_width = self.term_data.width as usize / cpu_cols;

        for r in 0..cpu_rows {
            for c in 0..cpu_cols {
                let i = c * cpu_rows + r;
                let bar =
                    DisplayManager::percentage_bar(core_width as u16 - 11, self.cpu.cores[i].usage);
                let s = format!("CPU {:>2}: {}", i, bar);
                print!("{} ", s);
            }
            println!("");
        }
    }

    fn display_memory(&self) {
        println!(
            "Memory Used: {:.1} GB",
            self.memory.used as f32 / BYTES_PER_GB as f32
        );
        println!(
            "Memory Total: {:.1} GB",
            (self.memory.total as f32 / BYTES_PER_GB as f32)
        );
        let percentage = self.memory.used as f32 / self.memory.total as f32 * 100.0;
        let mem_bar = DisplayManager::percentage_bar(self.term_data.width - 9, percentage);
        println!("Used: {}", mem_bar);
    }

    fn percentage_bar(width: u16, perc: f32) -> String {
        let mut s = String::from("[");
        let thresh = (width as f32 * (perc / 100.0)).round() as u16;
        let full = (0..thresh).map(|_| "|").collect::<String>();
        let empty = (thresh..width).map(|_| " ").collect::<String>();
        s.push_str(&full);
        s.push_str(&empty);
        s.push(']');
        s
    }
}
