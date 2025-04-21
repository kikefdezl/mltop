use super::cpu::CpuSnapshot;
use super::gpu::GpuSnapshot;
use super::memory::MemorySnapshot;
use super::processes::ProcessesSnapshot;
use super::system::System;

use sysinfo::Pid;

use super::{update_kind::DataUpdateKind, DataSnapshot};

pub struct Collector {
    system: System,
}

impl Collector {
    pub fn new() -> Collector {
        let mut system = System::new();
        system.refresh(&DataUpdateKind::all());

        Collector { system }
    }

    pub fn kill_process(&self, pid: usize) {
        if let Some(process) = self.system.sys.process(Pid::from(pid)) {
            process.kill();
        }
    }

    pub fn collect(&mut self, kind: &DataUpdateKind) -> DataSnapshot {
        self.system.refresh(kind);

        let cpu = if kind.cpu() {
            Some(CpuSnapshot::from_sysinfo(&self.system.sys))
        } else {
            None
        };

        let memory = if kind.memory() {
            Some(MemorySnapshot::from_sysinfo(&self.system.sys))
        } else {
            None
        };

        let gpu = if kind.gpu() {
            match &self.system.nvml {
                Some(n) => {
                    let result = GpuSnapshot::from_nvml(&n);
                    match result {
                        Ok(g) => Some(g),
                        Err(_) => None,
                    }
                }
                None => None,
            }
        } else {
            None
        };

        let processes = if kind.processes() {
            Some(ProcessesSnapshot::from_sysinfo_nvml(
                &self.system.sys,
                self.system.nvml.as_ref(),
            ))
        } else {
            None
        };
        DataSnapshot {
            cpu,
            memory,
            gpu,
            processes,
        }
    }

    pub fn can_read_gpu(&self) -> bool {
        self.system.nvml.is_some()
    }
}
