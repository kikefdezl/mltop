use nvml_wrapper::{error::NvmlError, Nvml};
use std::slice::Iter;
use std::vec::IntoIter;
use sysinfo::{Pid, System};

#[derive(Clone)]
pub enum GpuProcessType {
    GRAPHIC,
    COMPUTE,
}

impl ToString for GpuProcessType {
    fn to_string(&self) -> String {
        match self {
            GpuProcessType::GRAPHIC => "GRAPHIC".to_string(),
            GpuProcessType::COMPUTE => "COMPUTE".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Process {
    pub pid: u32,
    pub type_: GpuProcessType,
    pub command: String,
}

#[derive(Clone)]
pub struct Processes(pub Vec<Process>);

impl Processes {
    pub fn read() -> Result<Processes, NvmlError> {
        let nvml = Nvml::init()?;

        // TODO: be more specific with initialization
        let mut sys = System::new_all();
        sys.refresh_all();

        let device = nvml.device_by_index(0)?;

        let mut processes = Vec::new();

        processes.extend(device.running_compute_processes()?.iter().filter_map(|x| {
            match sys.process(Pid::from(x.pid as usize)).unwrap().exe() {
                None => None,
                Some(exe) => Some(Process {
                    pid: x.pid,
                    type_: GpuProcessType::COMPUTE,
                    command: String::from(exe.to_str().unwrap()),
                }),
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

    pub fn into_iter(&self) -> IntoIter<Process> {
        self.0.clone().into_iter()
    }

    pub fn iter(&self) -> Iter<Process> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
