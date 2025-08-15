use crate::data::update_kind::DataUpdateKind;
use nvml_wrapper::Nvml;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, Pid, ProcessRefreshKind, RefreshKind};
use sysinfo::{System as SysinfoSystem, UpdateKind};

pub struct System {
    pub sys: SysinfoSystem,
    pub nvml: Option<Nvml>,
}

impl System {
    pub fn new() -> System {
        System {
            sys: SysinfoSystem::new(),
            nvml: Nvml::init().ok(),
        }
    }

    pub fn refresh(&mut self, kind: &DataUpdateKind) {
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

        let refresh_kind = if kind.processes() {
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::new().with_cpu_usage())
                .with_memory(MemoryRefreshKind::everything())
                .with_processes(
                    ProcessRefreshKind::new()
                        .with_cpu()
                        .with_memory()
                        .with_cmd(UpdateKind::OnlyIfNotSet),
                )
        } else {
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::new().with_cpu_usage())
                .with_memory(MemoryRefreshKind::everything())
        };
        self.sys.refresh_specifics(refresh_kind);
    }

    pub fn kill_process(&self, pid: usize) {
        if let Some(process) = self.sys.process(Pid::from(pid)) {
            process.kill();
        }
    }
}
