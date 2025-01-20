use nvml_wrapper::{error::NvmlError, Nvml};
use std::fmt;
use sysinfo::{Components, CpuRefreshKind, Pid, RefreshKind, System};

pub enum GpuProcessType {
    GRAPHIC,
    COMPUTE,
}

impl fmt::Display for GpuProcessType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GpuProcessType::GRAPHIC => write!(f, "GRAPHIC"),
            GpuProcessType::COMPUTE => write!(f, "COMPUTE"),
        }
    }
}

pub struct Process {
    pub pid: u32,
    pub type_: GpuProcessType,
    pub command: String,
}

pub struct Processes(Vec<Process>);

impl Processes {
    pub fn read() -> Result<Processes, NvmlError> {
        let nvml = Nvml::init()?;

        // TODO: be more specific
        let mut sys = System::new_all();
        sys.refresh_all();

        let device = nvml.device_by_index(0)?;

        let mut processes = Vec::new();

        processes.extend(device.running_compute_processes()?.iter().map(|x| {
            Process {
                pid: x.pid,
                type_: GpuProcessType::COMPUTE,
                command: String::from(
                    sys.process(Pid::from(x.pid as usize))
                        .unwrap()
                        .exe()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                ),
            }
        }));
        processes.extend(device.running_graphics_processes()?.iter().map(|x| {
            Process {
                pid: x.pid,
                type_: GpuProcessType::GRAPHIC,
                command: String::from(
                    sys.process(Pid::from(x.pid as usize))
                        .unwrap()
                        .exe()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                ),
            }
        }));
        Ok(Processes(processes))
    }

    pub fn iter(&self) -> std::slice::Iter<Process> {
        self.0.iter()
    }
}
