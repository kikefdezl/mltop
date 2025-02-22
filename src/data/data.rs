use crate::data::components::cpu::Cpu;
use crate::data::components::gpu::Gpu;
use crate::data::components::memory::Memory;
use crate::data::components::processes::Processes;
use nvml_wrapper::Nvml;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct AppData {
    pub cpu: Vec<Cpu>,
    pub memory: Memory,
    pub gpu: Option<Vec<Gpu>>,
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

        let nvml = match Nvml::init() {
            Ok(n) => Some(n),
            Err(_) => None,
        };

        AppData {
            cpu: vec![Cpu::read(&sys)],
            memory: Memory::read(&sys),
            gpu: match Gpu::read(&nvml) {
                Ok(g) => Some(vec![g]),
                Err(_) => None,
            },
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

        self.cpu.push(Cpu::read(&self.sys));
        self.memory = Memory::read(&self.sys);

        if self.gpu.is_some() {
            match Gpu::read(&self.nvml) {
                Ok(g) => self.gpu.as_mut().unwrap().push(g),
                Err(_) => {}
            }
        }

        self.processes = match Processes::read(&self.sys, &self.nvml) {
            Ok(p) => p,
            Err(_) => Processes(Vec::new()),
        };
    }
}
