use super::cpu::CpuSnapshot;
use super::gpu::GpuSnapshot;
use super::memory::MemorySnapshot;
use super::processes::ProcessesSnapshot;
use super::snapshot::DataSnapshot;

/// The main data struct used by the widgets to render information.
/// It is meant to be updated in-place by passing instances of DataSnapshot.
pub struct SystemData {
    pub cpu: CpuSnapshot,
    pub memory: MemorySnapshot,
    pub gpu: Option<GpuSnapshot>,
    pub processes: ProcessesSnapshot,
}

impl SystemData {
    pub fn new_from_snapshot(snapshot: DataSnapshot) -> SystemData {
        SystemData {
            cpu: snapshot.cpu.expect("First snapshot must have cpu"),
            memory: snapshot.memory.expect("First snapshot must have memory"),
            gpu: snapshot.gpu,
            processes: snapshot
                .processes
                .expect("First snapshot must have processes"),
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
