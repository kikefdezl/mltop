use crate::data::update_kind::DataUpdateKind;
use nvml_wrapper::Nvml;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, Pid, ProcessRefreshKind, RefreshKind};
use sysinfo::{System as SysinfoSystem, UpdateKind};

use crate::data::cpu::CpuSnapshot;
use crate::data::gpu::GpuSnapshot;
use crate::data::memory::MemorySnapshot;
use crate::data::processes::ProcessesSnapshot;
use crate::data::snapshot::DataSnapshot;

pub trait SystemProvider {
    fn collect_snapshot(&mut self, kind: &DataUpdateKind) -> DataSnapshot;
    fn kill_process(&self, pid: usize);
    fn can_read_gpu(&self) -> bool;
}

pub struct System {
    pub sys: SysinfoSystem,
    pub nvml: Option<Nvml>,
}

impl System {
    pub fn new() -> System {
        System {
            sys: SysinfoSystem::new(),
            nvml: Nvml::init().ok(),
        }
    }

    pub fn refresh(&mut self, kind: &DataUpdateKind) {
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

        let refresh_kind = if kind.processes() {
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::new().with_cpu_usage())
                .with_memory(MemoryRefreshKind::everything())
                .with_processes(
                    ProcessRefreshKind::new()
                        .with_cpu()
                        .with_memory()
                        .with_cmd(UpdateKind::OnlyIfNotSet),
                )
        } else {
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::new().with_cpu_usage())
                .with_memory(MemoryRefreshKind::everything())
        };
        self.sys.refresh_specifics(refresh_kind);
    }
}

impl SystemProvider for System {
    fn kill_process(&self, pid: usize) {
        if let Some(process) = self.sys.process(Pid::from(pid)) {
            process.kill();
        }
    }

    fn collect_snapshot(&mut self, kind: &DataUpdateKind) -> DataSnapshot {
        self.refresh(kind);

        let cpu = if kind.cpu() {
            Some(CpuSnapshot::from_sysinfo(&self.sys))
        } else {
            None
        };

        let memory = if kind.memory() {
            Some(MemorySnapshot::from_sysinfo(&self.sys))
        } else {
            None
        };

        let gpu = if kind.gpu() {
            match &self.nvml {
                Some(n) => {
                    let result = GpuSnapshot::from_nvml(n);
                    result.ok()
                }
                None => None,
            }
        } else {
            None
        };

        let processes = if kind.processes() {
            Some(ProcessesSnapshot::from_sysinfo_nvml(
                &self.sys,
                self.nvml.as_ref(),
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

    fn can_read_gpu(&self) -> bool {
        self.nvml.is_some()
    }
}
