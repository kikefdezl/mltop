use crate::color;
use crate::devices::cpu::Cpu;
use crate::devices::gpu::Gpu;
use crate::devices::memory::Memory;
use crate::terminal_data::TerminalData;
use crate::utils;

use std::io::Stdout;
use std::io::Write as IoWrite;

const BYTES_PER_GB: u64 = 1024_u64.pow(3);

pub struct DisplayManager<'a> {
    pub cpu: &'a Cpu,
    pub memory: &'a Memory,
    pub gpu: &'a Option<Gpu>,
    pub term_data: &'a TerminalData,
}

impl DisplayManager<'_> {
    pub fn display(&self, stdout: &mut Stdout) {
        let cpu_content = self.display_cpu();
        let memory_content = self.display_memory();
        let gpu_content = self.display_gpu();

        let content = format!("{}\r\n{}\r\n{}", cpu_content, memory_content, gpu_content);
        write!(stdout, "{}", content).unwrap();
    }

    fn display_cpu(&self) -> String {
        let text = format!("{:.2}%", self.cpu.usage);
        let total_bar =
            DisplayManager::percentage_bar(self.term_data.width - 10, self.cpu.usage, &text);
        let mut content = String::from(format!(" {} {}\r\n", color::cyan_text("Total"), total_bar));

        let num_cpus = self.cpu.cores.len();
        let cpu_rows = utils::fast_int_sqrt(num_cpus);
        let mut cpu_cols = 0;
        while cpu_cols * cpu_rows < num_cpus {
            cpu_cols += 1;
        }

        let core_width = self.term_data.width as usize / cpu_cols;

        for r in 0..cpu_rows {
            content.push_str("   ");
            for c in 0..cpu_cols {
                let i = c * cpu_rows + r;
                let text = format!("{:.2}%", self.cpu.cores[i].usage);
                let bar = DisplayManager::percentage_bar(
                    core_width as u16 - 14,
                    self.cpu.cores[i].usage,
                    &text,
                );
                let cpu_num = color::cyan_text(&format!("{:>2}", i));
                let bar_str = format!("  {:>2}{}", cpu_num, bar);
                content.push_str(&bar_str);

                let temp_str = if self.cpu.cores[i].temp == 0.0 {
                    " N/A   ".to_string()
                } else {
                    let temp_str = format!("{:>5.1}°C", self.cpu.cores[i].temp);
                    if self.cpu.cores[i].temp > 90.0 {
                        color::red_text(&temp_str)
                    } else if self.cpu.cores[i].temp > 80.0 {
                        color::orange_text(&temp_str)
                    } else if self.cpu.cores[i].temp > 70.0 {
                        color::yellow_text(&temp_str)
                    } else {
                        temp_str.to_string()
                    }
                };
                content.push_str(&temp_str);
            }
            content.push_str("\r\n");
        }
        content
    }

    fn display_memory(&self) -> String {
        let mut content = String::new();

        let percentage = self.memory.used as f32 / self.memory.total as f32 * 100.0;
        let text = format!("{:.2}%", percentage);
        let mem_bar = DisplayManager::percentage_bar(self.term_data.width - 11, percentage, &text);
        let mem_bar_str = format!(" {} {}\r\n", color::cyan_text("Memory"), mem_bar);
        content.push_str(&mem_bar_str);

        let used = format!(
            " {} {:.1} GB\r\n",
            color::cyan_text("Used:"),
            self.memory.used as f32 / BYTES_PER_GB as f32
        );
        content.push_str(&used);

        let total = format!(
            " {} {:.1} GB\r\n",
            color::cyan_text("Total:"),
            (self.memory.total as f32 / BYTES_PER_GB as f32)
        );
        content.push_str(&total);

        content
    }

    fn display_gpu(&self) -> String {
        let mut content = String::new();
        match &self.gpu {
            None => content.push_str("No GPU found."),
            Some(gpu) => {
                let name = format!(" {} {}", color::cyan_text("Device 0:"), gpu.name);
                content.push_str(&name);

                let temp = format!(" {} {}°C\r\n", color::cyan_text("TEMP:"), gpu.temperature);
                content.push_str(&temp);

                let total_width = self.term_data.width;

                let utilization = gpu.utilization.last().unwrap();
                let use_bar = DisplayManager::percentage_bar(
                    total_width / 3 - 5,
                    *utilization as f32,
                    &format!("{}%", utilization),
                );
                let use_perc = format!(" {}{}", color::cyan_text("GPU"), use_bar);
                content.push_str(&use_perc);

                let used = gpu.used_memory.last().unwrap();
                let mem_perc: f32 = (*used as f32 / gpu.max_memory as f32) * 100.0;
                let mem_bar = DisplayManager::percentage_bar(
                    total_width / 3 - 5,
                    mem_perc,
                    &format!("{:.2}Gi/{:.2}Gi", (*used as f32) / 1000000000.0, (gpu.max_memory as f32) / 1000000000.0),
                );
                let mem_perc = format!(" {}{}\r\n", color::cyan_text("MEM"), mem_bar);
                content.push_str(&mem_perc);
            }
        }
        content
    }

    // TODO: Use generics here
    fn percentage_bar(width: u16, perc: f32, text: &str) -> String {
        let mut s = String::from("[");

        let color_fn: fn(&str) -> String = if perc > 80.0 {
            |x| color::red_text(x)
        } else if perc > 50.0 {
            |x| color::orange_text(x)
        } else if perc > 20.0 {
            |x| color::yellow_text(x)
        } else {
            |x| x.to_string()
        };

        let full_width = (width as f32 * (perc / 100.0)).round() as u16;
        let text_width = text.chars().count() as u16;
        let bar_width = std::cmp::min(full_width, width - text_width);

        let bar = (0..bar_width).map(|_| "|").collect::<String>();
        let bar_colored = color_fn(&bar);
        let empty = (bar_width..(width - text_width))
            .map(|_| " ")
            .collect::<String>();

        let colored_text_width = text_width.saturating_sub(width - full_width);
        let colored_text = color_fn(&text[..colored_text_width as usize]);
        let grey_text = color::gray_text(&text[colored_text_width as usize..]);

        s.push_str(&bar_colored);
        s.push_str(&empty);
        s.push_str(&colored_text);
        s.push_str(&grey_text);
        s.push(']');
        s
    }
}
