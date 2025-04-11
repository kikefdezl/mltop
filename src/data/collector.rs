use crate::data::models::cpu::Cpu;
use crate::data::models::gpu::Gpu;
use crate::data::models::memory::Memory;
use crate::data::models::processes::Processes;
use nvml_wrapper::Nvml;
use sysinfo::{
    CpuRefreshKind, MemoryRefreshKind, Pid, ProcessRefreshKind, RefreshKind, Signal, System,
};

use super::update_kind::DataUpdateKind;

pub struct Collector {
    pub cpu: Cpu,
    pub memory: Memory,
    pub gpu: Option<Gpu>,
    pub processes: Processes,
    sys: System,
    nvml: Option<Nvml>,
}

impl Collector {
    pub fn new() -> Collector {
        let mut sys = System::new();

        Self::refresh_system(&mut sys);

        let mut cpu = Cpu::new();
        cpu.update(&sys);

        let nvml = match Nvml::init() {
            Ok(n) => Some(n),
            Err(_) => None,
        };

        let gpu = match Gpu::new(&nvml) {
            Err(_) => None,
            Ok(mut g) => {
                let _ = g.update(&nvml);
                Some(g)
            }
        };

        let mut processes = Processes::new();
        processes.update(&sys, &nvml);

        Collector {
            cpu,
            memory: Memory::read(&sys),
            gpu,
            processes,
            sys,
            nvml,
        }
    }

    pub fn update(&mut self, kind: &DataUpdateKind) {
        if kind.any() {
            Self::refresh_system(&mut self.sys);

            if kind.cpu() {
                self.cpu.update(&self.sys);
            }
            if kind.memory() {
                self.memory = Memory::read(&self.sys);
            }
            if kind.processes() {
                self.processes.update(&self.sys, &self.nvml);
            }

            if kind.gpu() {
                if let Some(gpu) = self.gpu.as_mut() {
                    let _ = gpu.update(&self.nvml);
                }
            }
        }
    }

    pub fn terminate_process(&self, pid: usize) {
        if let Some(process) = self.sys.process(Pid::from(pid)) {
            process.kill_with(Signal::Term);
        }
    }

    pub fn kill_process(&self, pid: usize) {
        if let Some(process) = self.sys.process(Pid::from(pid)) {
            process.kill_with(Signal::Kill);
        }
    }

    fn refresh_system(sys: &mut System) {
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        let refresh_kind = RefreshKind::new()
            .with_cpu(CpuRefreshKind::new().with_cpu_usage())
            .with_memory(MemoryRefreshKind::everything())
            .with_processes(
                ProcessRefreshKind::everything(), // .with_cpu()
                                                  // .with_memory()
                                                  // .with_cmd(sysinfo::UpdateKind::OnlyIfNotSet),
            );
        sys.refresh_specifics(refresh_kind);
    }

    pub fn has_gpu(&self) -> bool {
        self.gpu.is_some()
    }
}
