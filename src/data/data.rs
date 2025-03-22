use crate::data::components::cpu::Cpu;
use crate::data::components::gpu::Gpu;
use crate::data::components::memory::Memory;
use crate::data::components::processes::Processes;
use nvml_wrapper::Nvml;
use sysinfo::{ProcessRefreshKind, System};

pub struct AppData {
    pub cpu: Cpu,
    pub memory: Memory,
    pub gpu: Option<Gpu>,
    pub processes: Processes,
    sys: System,
    nvml: Option<Nvml>,
}

impl AppData {
    pub fn new() -> AppData {
        let mut sys = System::new();

        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_all();

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

        AppData {
            cpu,
            memory: Memory::read(&sys),
            gpu,
            processes: Processes::read(&sys, &nvml),
            sys,
            nvml,
        }
    }

    pub fn update(&mut self) {
        self.sys.refresh_cpu_usage();
        self.cpu.update(&self.sys);

        self.sys.refresh_memory();
        self.memory = Memory::read(&self.sys);

        if let Some(gpu) = self.gpu.as_mut() {
            let _ = gpu.update(&self.nvml);
        }

        self.sys.refresh_processes_specifics(
            ProcessRefreshKind::new()
                .with_cpu()
                .with_memory()
                .with_exe(sysinfo::UpdateKind::OnlyIfNotSet),
        );
        self.processes = Processes::read(&self.sys, &self.nvml);
    }
}
