use crate::color;
use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::terminal_data::TerminalData;
use crate::utils;

use std::io::Stdout;
use std::io::Write as IoWrite;

const BYTES_PER_GB: u64 = 1024_u64.pow(3);

pub struct DisplayManager {
    pub cpu: Cpu,
    pub memory: Memory,
    pub term_data: TerminalData,
}

impl DisplayManager {
    pub fn display(&self, stdout: &mut Stdout) {
        let cpu_content = self.display_cpu();
        let memory_content = self.display_memory();

        let content = format!("{}{}", cpu_content, memory_content);
        write!(stdout, "{}", content).unwrap();
    }

    fn display_cpu(&self) -> String {
        let total_bar = DisplayManager::percentage_bar(self.term_data.width - 15, self.cpu.usage);
        let mut content = String::from(format!(" CPU Total: {}\r\n", total_bar));

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
                    DisplayManager::percentage_bar(core_width as u16 - 12, self.cpu.cores[i].usage);
                let bar_str = format!(" CPU {:>2}: {} ", i, bar);
                content.push_str(&bar_str);
            }
            content.push_str("\r\n");
        }
        content
    }

    fn display_memory(&self) -> String {
        let mut content = String::new();

        let used = format!(
            " Memory Used: {:.1} GB\r\n",
            self.memory.used as f32 / BYTES_PER_GB as f32
        );
        content.push_str(&used);

        let total = format!(
            " Memory Total: {:.1} GB\r\n",
            (self.memory.total as f32 / BYTES_PER_GB as f32)
        );
        content.push_str(&total);

        let percentage = self.memory.used as f32 / self.memory.total as f32 * 100.0;
        let mem_bar = DisplayManager::percentage_bar(self.term_data.width - 10, percentage);
        let mem_bar_str = format!(" Used: {}\r\n", mem_bar);
        content.push_str(&mem_bar_str);

        content
    }

    fn percentage_bar(width: u16, perc: f32) -> String {
        let mut s = String::from("[");
        let thresh = (width as f32 * (perc / 100.0)).round() as u16;

        let fill = if perc > 80.0 {
            color::red_text("|")
        } else if perc > 50.0 {
            color::orange_text("|")
        } else if perc > 20.0 {
            color::yellow_text("|")
        } else {
            String::from("|")
        };

        let full = (0..thresh).map(|_| fill.as_str()).collect::<String>();
        let empty = (thresh..width).map(|_| " ").collect::<String>();
        s.push_str(&full);
        s.push_str(&empty);
        s.push(']');
        s
    }
}
