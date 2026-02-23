use mltop::config::init_config;
use mltop::constants::BYTES_PER_GB;
use mltop::data::cpu::{CoreSnapshot, CpuSnapshot};
use mltop::data::gpu::GpuSnapshot;
use mltop::data::memory::MemorySnapshot;
use mltop::data::processes::ProcessesSnapshot;
use mltop::system::FakeSystem;
use mltop::tui::Tui;
use ratatui::backend::TestBackend;

fn cpu() -> CpuSnapshot {
    let cores: Vec<CoreSnapshot> = (0..32)
        .map(|_| CoreSnapshot {
            usage: 0.5,
            temp: 50.0,
        })
        .collect();
    CpuSnapshot { usage: 0.5, cores }
}

fn memory() -> MemorySnapshot {
    MemorySnapshot {
        used: 32 * BYTES_PER_GB,  // 32 GB
        total: 64 * BYTES_PER_GB, // 64 GB
        used_swap: 0,
        total_swap: 64 * BYTES_PER_GB, // 64 GB
    }
}

fn gpu() -> GpuSnapshot {
    GpuSnapshot {
        name: String::from("RTX 5090"),
        temperature: 60,
        max_memory: 24 * BYTES_PER_GB,
        used_memory: 12 * BYTES_PER_GB,
        utilization: 50,
        max_power: 500,
        power_usage: 250,
        fan_speed: Some(50),
    }
}

fn processes() -> ProcessesSnapshot {
    ProcessesSnapshot {
        processes: Vec::new(),
    }
}

fn system() -> FakeSystem {
    FakeSystem::default()
        .with_cpu(cpu())
        .with_memory(memory())
        .with_gpu(gpu())
        .with_processes(processes())
}

#[test]
fn test_large_system() {
    init_config();
    let system: FakeSystem = system();
    let backend = TestBackend::new(120, 40);
    let mut app = Tui::fake(system, backend);
    app.render();
}
