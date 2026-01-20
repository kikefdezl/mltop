use crate::data::update_kind::DataUpdateKind;
use nvml_wrapper::Nvml;
use sysinfo::ProcessesToUpdate;
use sysinfo::{MemoryRefreshKind, Pid, ProcessRefreshKind};
use sysinfo::{System as SysinfoSystem, UpdateKind};

use crate::data::cpu::CpuSnapshot;
use crate::data::gpu::GpuSnapshot;
use crate::data::memory::MemorySnapshot;
use crate::data::processes::ProcessesSnapshot;
use crate::data::snapshot::DataSnapshot;

// SystemMonitor is a trait with Real and Fake implementations.
// The fake implementation allows us to test different hardware configurations
pub trait SystemMonitor {
    fn collect_snapshot(&mut self, kind: &DataUpdateKind) -> DataSnapshot;
    fn kill_process(&self, pid: usize);
    fn gpu_available(&self) -> bool;
}

pub struct RealSystem {
    pub sys: SysinfoSystem,
    pub nvml: Option<Nvml>,
}

impl Default for RealSystem {
    fn default() -> RealSystem {
        RealSystem {
            sys: SysinfoSystem::new(),
            nvml: Nvml::init().ok(),
        }
    }
}

impl RealSystem {
    pub fn refresh(&mut self, kind: &DataUpdateKind) {
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

        self.sys.refresh_cpu_usage();
        self.sys
            .refresh_memory_specifics(MemoryRefreshKind::everything());
        if kind.processes() {
            self.sys.refresh_processes_specifics(
                ProcessesToUpdate::All,
                true,
                ProcessRefreshKind::default()
                    .with_cpu()
                    .with_memory()
                    .with_cmd(UpdateKind::OnlyIfNotSet),
            );
        };
    }
}

impl SystemMonitor for RealSystem {
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

    fn gpu_available(&self) -> bool {
        self.nvml.is_some()
    }
}

// --- Fakes for Testing ---

#[derive(Default)]
pub struct FakeSystem {
    cpu: Option<CpuSnapshot>,
    memory: Option<MemorySnapshot>,
    gpu: Option<GpuSnapshot>,
    processes: Option<ProcessesSnapshot>,
}

impl SystemMonitor for FakeSystem {
    fn collect_snapshot(&mut self, _: &DataUpdateKind) -> DataSnapshot {
        DataSnapshot {
            cpu: self.cpu.clone(),
            memory: self.memory.clone(),
            gpu: self.gpu.clone(),
            processes: self.processes.clone(),
        }
    }

    fn kill_process(&self, _: usize) {}

    fn gpu_available(&self) -> bool {
        self.gpu.is_some()
    }
}

impl FakeSystem {
    pub fn with_cpu(mut self, cpu_snapshot: CpuSnapshot) -> Self {
        self.cpu = Some(cpu_snapshot);
        self
    }

    pub fn with_memory(mut self, memory: MemorySnapshot) -> Self {
        self.memory = Some(memory);
        self
    }
    pub fn with_gpu(mut self, gpu: GpuSnapshot) -> Self {
        self.gpu = Some(gpu);
        self
    }
    pub fn with_processes(mut self, processes: ProcessesSnapshot) -> Self {
        self.processes = Some(processes);
        self
    }
}
