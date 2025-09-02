use sysinfo::System;

/// Memory usage
///
/// All values in bytes.
#[derive(Clone)]
pub struct MemorySnapshot {
    pub used: u64,
    pub total: u64,
    pub used_swap: u64,
    pub total_swap: u64,
}

impl MemorySnapshot {
    pub fn from_sysinfo(sys: &System) -> MemorySnapshot {
        MemorySnapshot {
            used: sys.used_memory(),
            total: sys.total_memory(),
            used_swap: sys.used_swap(),
            total_swap: sys.total_swap(),
        }
    }

    pub fn total_percent(&self) -> f64 {
        ((self.used + self.used_swap) as f64) / ((self.total + self.total_swap) as f64)
    }
}
