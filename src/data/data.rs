use crate::data::components::cpu::Cpu;
use crate::data::components::gpu::Gpu;
use crate::data::components::memory::Memory;
use crate::data::components::processes::Processes;
use nvml_wrapper::Nvml;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

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
        let mut sys =
            System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::new()));

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
                g.update(&nvml);
                Some(g)
            }
        };

        AppData {
            cpu,
            memory: Memory::read(&sys),
            gpu,
            processes: match Processes::read(&sys, &nvml) {
                Ok(p) => p,
                Err(_) => Processes(Vec::new()),
            },
            sys,
            nvml,
        }
    }

    pub fn update(&mut self) {
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        self.sys.refresh_all();

        self.cpu.update(&self.sys);
        self.memory = Memory::read(&self.sys);

        if let Some(gpu) = self.gpu.as_mut() {
            gpu.update(&self.nvml);
        }

        self.processes = match Processes::read(&self.sys, &self.nvml) {
            Ok(p) => p,
            Err(_) => Processes(Vec::new()),
        };
    }
}
