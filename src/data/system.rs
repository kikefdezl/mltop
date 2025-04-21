use crate::data::update_kind::DataUpdateKind;
use nvml_wrapper::Nvml;
use sysinfo::System as SysinfoSystem;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, ProcessRefreshKind, RefreshKind};

pub struct System {
    pub sys: SysinfoSystem,
    pub nvml: Option<Nvml>,
}

impl System {
    pub fn new() -> System {
        let nvml = match Nvml::init() {
            Ok(n) => Some(n),
            Err(_) => None,
        };
        System {
            sys: SysinfoSystem::new(),
            nvml,
        }
    }

    pub fn refresh(&mut self, kind: &DataUpdateKind) {
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

        let refresh_kind = if kind.processes() {
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::new().with_cpu_usage())
                .with_memory(MemoryRefreshKind::everything())
                .with_processes(ProcessRefreshKind::everything())
        } else {
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::new().with_cpu_usage())
                .with_memory(MemoryRefreshKind::everything())
        };
        self.sys.refresh_specifics(refresh_kind);
    }
}
