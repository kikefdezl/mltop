use self::cpu::CpuSnapshot;
use self::gpu::GpuSnapshot;
use self::memory::MemorySnapshot;
use self::processes::ProcessesSnapshot;

pub mod collector;
pub mod cpu;
pub mod gpu;
pub mod memory;
pub mod processes;
pub mod system;
pub mod update_kind;

pub struct Data {
    pub cpu: CpuSnapshot,
    pub memory: MemorySnapshot,
    pub gpu: Option<GpuSnapshot>,
    pub processes: ProcessesSnapshot,
}

impl Data {
    // the snapshot here MUST contain cpu, memory, processes or we panic
    pub fn new_from_snapshot(snapshot: DataSnapshot) -> Data {
        Data {
            cpu: snapshot.cpu.expect("Snapshot must have cpu"),
            memory: snapshot.memory.expect("Snapshot must have memory"),
            gpu: snapshot.gpu,
            processes: snapshot.processes.expect("Snapshot must have processes"),
        }
    }

    pub fn update_from_snapshot(&mut self, snapshot: DataSnapshot) {
        if let Some(c) = snapshot.cpu {
            self.cpu = c;
        }
        if let Some(m) = snapshot.memory {
            self.memory = m;
        }
        if let Some(g) = snapshot.gpu {
            self.gpu = Some(g);
        }
        if let Some(p) = snapshot.processes {
            self.processes = p;
        }
    }

    pub fn has_gpu(&self) -> bool {
        self.gpu.is_some()
    }
}

#[derive(Clone)]
pub struct DataSnapshot {
    pub cpu: Option<CpuSnapshot>,
    pub memory: Option<MemorySnapshot>,
    pub gpu: Option<GpuSnapshot>,
    pub processes: Option<ProcessesSnapshot>,
}

pub struct DataStore {
    pub snapshots: Vec<DataSnapshot>,
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore { snapshots: vec![] }
    }

    pub fn save(&mut self, snapshot: DataSnapshot) {
        self.snapshots.push(snapshot);
    }
}
