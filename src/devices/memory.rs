use sysinfo::System;

pub struct Memory {
    pub used: u64,
    pub total: u64,
}

impl Memory {
    pub fn read() -> Memory {
        let sys = System::new_all();

        let memory = Memory {
            used: sys.used_memory(),
            total: sys.total_memory(),
        };
        memory
    }
}
