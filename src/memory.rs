use sysinfo::System;

pub struct Memory {
    pub used: u64,
    pub total: u64,
}

const BYTES_PER_GB: u64 = 1024_u64.pow(3);

impl Memory {
    pub fn read() -> Memory {
        let sys = System::new_all();

        let memory = Memory {
            used: sys.used_memory(),
            total: sys.total_memory(),
        };
        memory
    }

    pub fn display(self) {
        println!("---");
        println!(
            "Memory Used: {:.1} GB",
            self.used as f64 / BYTES_PER_GB as f64
        );
        println!(
            "Memory Total: {:.1} GB",
            (self.total as f64 / BYTES_PER_GB as f64)
        );
        println!(
            "Used: {:.2} %",
            self.used as f64 / self.total as f64 
        );
        println!("---");
    }
}
