use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct Core {
    pub usage: f32,
    pub temp: f32,
}

pub struct Cpu {
    pub cores: Vec<Core>,
}

impl Cpu {
    pub fn read() -> Cpu {
        let mut cores: Vec<Core> = Vec::new();

        let mut sys =
            System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_cpu();
        for cpu in sys.cpus() {
            let usage: f32 = cpu.cpu_usage();
            let core = Core { usage, temp: 0.0 };
            cores.push(core);
        }
        let cpu = Cpu { cores };
        cpu
    }
}
