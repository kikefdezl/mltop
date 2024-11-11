use sysinfo::{CpuRefreshKind, RefreshKind, System};

#[derive(Clone)]
pub struct Core {
    pub usage: f32,
    pub temp: f32,
}

pub struct Cpu {
    pub usage: f32, // as a value between 0.0 and 100.0
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
            //let temperature: f32 = cpu.temperature();
            let core = Core { usage, temp: 0.0 };
            cores.push(core);
        }

        let usage = sys.global_cpu_info().cpu_usage();
        let cpu = Cpu { usage, cores };
        cpu
    }
}
