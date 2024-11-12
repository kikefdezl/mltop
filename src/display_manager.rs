use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::terminal_data::TerminalData;
use crate::utils;
use std::fmt::Write;

const BYTES_PER_GB: u64 = 1024_u64.pow(3);

pub struct DisplayManager {
    pub cpu: Cpu,
    pub memory: Memory,
    pub term_data: TerminalData,
}

impl DisplayManager {
    pub fn display(&self) {
        let cpu_content = self.display_cpu();
        let memory_content = self.display_memory();

        let content = format!("{}{}", cpu_content, memory_content);
        print!("\x1B[2J\x1B[1;1H"); // clear the display
        print!("{}", content);
    }

    fn display_cpu(&self) -> String {
        let total_bar = DisplayManager::percentage_bar(self.term_data.width - 14, self.cpu.usage);
        let mut content = String::from(format!("CPU Total: {}\n", total_bar));
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
                write!(content, "CPU {:>2}: {} ", i, bar).unwrap();
            }
            write!(content, "\n").unwrap();
        }
        content
    }

    fn display_memory(&self) -> String {
        let mut content = String::new();
        write!(
            content,
            "Memory Used: {:.1} GB\n",
            self.memory.used as f32 / BYTES_PER_GB as f32
        )
        .unwrap();
        write!(
            content,
            "Memory Total: {:.1} GB\n",
            (self.memory.total as f32 / BYTES_PER_GB as f32)
        )
        .unwrap();
        let percentage = self.memory.used as f32 / self.memory.total as f32 * 100.0;
        let mem_bar = DisplayManager::percentage_bar(self.term_data.width - 9, percentage);
        write!(content, "Used: {}\n", mem_bar).unwrap();
        content
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
