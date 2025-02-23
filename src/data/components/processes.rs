use nvml_wrapper::{error::NvmlError, Nvml};
use std::vec::IntoIter;
use sysinfo::{Pid, System};

#[derive(Clone)]
pub enum ProcessType {
    CPU, // TODO: Add cpu processes
    GPU_GRAPHIC,
    GPU_COMPUTE,
}

impl ToString for ProcessType {
    fn to_string(&self) -> String {
        match self {
            ProcessType::CPU => "".to_string(),
            ProcessType::GPU_GRAPHIC => "GRAPHIC".to_string(),
            ProcessType::GPU_COMPUTE => "COMPUTE".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Process {
    pub pid: u32,
    pub type_: ProcessType,
    pub command: String,
    // percentage 0-100% (can be higher than 100% if uses more than one core)
    pub cpu_usage: f32,
    // bytes
    pub memory: u64,
    // percentage 0-100%
    pub memory_usage: f32,
}

#[derive(Clone)]
pub struct Processes(pub Vec<Process>);

impl Processes {
    pub fn read(sys: &System, nvml: &Option<Nvml>) -> Result<Processes, NvmlError> {
        if nvml.is_none() {
            return Ok(Processes(Vec::new()));
        }

        let nvml = nvml.as_ref().unwrap();
        let device = nvml.device_by_index(0)?;

        let mut processes = Vec::new();

        let total_memory = sys.total_memory();

        processes.extend(device.running_compute_processes()?.iter().filter_map(|x| {
            match sys.process(Pid::from(x.pid as usize)) {
                None => None,
                Some(p) => Some({
                    let memory = p.memory();
                    Process {
                        pid: x.pid,
                        type_: ProcessType::GPU_COMPUTE,
                        command: String::from(p.exe().unwrap().to_str().unwrap()),
                        memory,
                        memory_usage: (memory as f32 / total_memory as f32) * 100.0,
                        cpu_usage: p.cpu_usage(),
                    }
                }),
            }
        }));
        processes.extend(device.running_graphics_processes()?.iter().filter_map(|x| {
            match sys.process(Pid::from(x.pid as usize)) {
                None => None,
                Some(p) => Some({
                    let memory = p.memory();
                    Process {
                        pid: x.pid,
                        type_: ProcessType::GPU_GRAPHIC,
                        command: String::from(p.exe().unwrap().to_str().unwrap()),
                        memory,
                        memory_usage: (memory as f32 / total_memory as f32) * 100.0,
                        cpu_usage: p.cpu_usage(),
                    }
                }),
            }
        }));
        Ok(Processes(processes))
    }

    pub fn into_iter(&self) -> IntoIter<Process> {
        self.0.clone().into_iter()
    }
}
