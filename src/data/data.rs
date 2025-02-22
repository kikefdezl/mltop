use crate::data::components::cpu::Cpu;
use crate::data::components::gpu::Gpu;
use crate::data::components::memory::Memory;
use crate::data::components::processes::Processes;

pub struct AppData {
    pub cpu: Cpu,
    pub memory: Memory,
    pub gpu: Option<Gpu>,
    pub processes: Processes,
}

impl AppData {
    pub fn new() -> AppData {
        AppData {
            cpu: Cpu::read(),
            memory: Memory::read(),
            gpu: match Gpu::read() {
                Ok(g) => Some(g),
                Err(_) => None,
            },
            processes: match Processes::read() {
                Ok(p) => p,
                Err(_) => Processes(Vec::new()),
            },
        }
    }

    pub fn update(&mut self) {
        self.cpu = Cpu::read();
        self.memory = Memory::read();
        self.gpu = match Gpu::read() {
            Ok(g) => Some(g),
            Err(_) => None,
        };
        self.processes = match Processes::read() {
            Ok(p) => p,
            Err(_) => Processes(Vec::new()),
        };
    }
}
