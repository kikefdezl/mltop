use nvml_wrapper::{error::NvmlError, Nvml};
use std::fmt;

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
}

pub struct Processes(Vec<Process>);

impl Processes {
    pub fn read() -> Result<Processes, NvmlError> {
        let nvml = Nvml::init()?;

        let device = nvml.device_by_index(0)?;

        let mut processes = Vec::new();

        processes.extend(device.running_compute_processes()?.iter().map(|x| Process {
            pid: x.pid,
            type_: GpuProcessType::COMPUTE,
        }));
        processes.extend(
            device
                .running_graphics_processes()?
                .iter()
                .map(|x| Process {
                    pid: x.pid,
                    type_: GpuProcessType::GRAPHIC,
                }),
        );
        Ok(Processes(processes))
    }

    pub fn iter(&self) -> std::slice::Iter<Process> {
        self.0.iter()
    }
}
