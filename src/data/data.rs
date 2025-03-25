use crate::data::components::cpu::Cpu;
use crate::data::components::gpu::Gpu;
use crate::data::components::memory::Memory;
use crate::data::components::processes::Processes;
use nvml_wrapper::Nvml;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, ProcessRefreshKind, RefreshKind, System};

pub struct Data {
    pub cpu: Cpu,
    pub memory: Memory,
    pub gpu: Option<Gpu>,
    pub processes: Processes,
    sys: System,
    nvml: Option<Nvml>,
}

impl Data {
    pub fn new() -> Data {
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

        Data {
            cpu,
            memory: Memory::read(&sys),
            gpu,
            processes: Processes::read(&sys, &nvml),
            sys,
            nvml,
        }
    }

    pub fn update(&mut self) {
        Self::refresh_system(&mut self.sys);

        self.cpu.update(&self.sys);
        self.memory = Memory::read(&self.sys);
        self.processes = Processes::read(&self.sys, &self.nvml);

        if let Some(gpu) = self.gpu.as_mut() {
            let _ = gpu.update(&self.nvml);
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
}
