use sysinfo::System;

#[derive(Clone)]
pub struct Memory {
    pub used: u64,
    pub total: u64,
    pub used_swap: u64,
    pub total_swap: u64,
}

impl Memory {
    pub fn read(sys: &System) -> Memory {
        let memory = Memory {
            used: sys.used_memory(),
            total: sys.total_memory(),
            used_swap: sys.used_swap(),
            total_swap: sys.total_swap(),
        };
        memory
    }
}
