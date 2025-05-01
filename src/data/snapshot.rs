use super::cpu::CpuSnapshot;
use super::gpu::GpuSnapshot;
use super::memory::MemorySnapshot;
use super::processes::ProcessesSnapshot;

#[derive(Clone)]
pub struct DataSnapshot {
    pub cpu: Option<CpuSnapshot>,
    pub memory: Option<MemorySnapshot>,
    pub gpu: Option<GpuSnapshot>,
    pub processes: Option<ProcessesSnapshot>,
}
