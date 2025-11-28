use nvml_wrapper::{error::NvmlError, Nvml};
use std::collections::HashMap;
use std::fmt::{self, Display};
use sysinfo::System;
use sysinfo::ThreadKind;

#[derive(Clone)]
pub enum ProcessType {
    GpuGraphic,
    GpuCompute,
    Cpu,
    UserThread,
    KernelThread,
}

impl ProcessType {
    fn is_thread(&self) -> bool {
        matches!(self, ProcessType::UserThread | ProcessType::KernelThread)
    }
}

impl Display for ProcessType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessType::GpuGraphic => write!(f, "GRAPHIC"),
            ProcessType::GpuCompute => write!(f, "COMPUTE"),
            ProcessType::Cpu => write!(f, "CPU"),
            // For now i'll display both thread types equally,
            // I'm not sure if we really want to distiguish between the two
            ProcessType::UserThread => write!(f, "THREAD"),
            ProcessType::KernelThread => write!(f, "THREAD"),
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

impl Process {
    pub fn is_thread(&self) -> bool {
        self.type_.is_thread()
    }
}

#[derive(Clone)]
pub struct ProcessesSnapshot {
    pub processes: Vec<Process>,
}

impl ProcessesSnapshot {
    pub fn from_sysinfo_nvml(sys: &System, nvml: Option<&Nvml>) -> ProcessesSnapshot {
        let total_memory = sys.total_memory();

        let mut processes: HashMap<u32, Process> = sys
            .processes()
            .iter()
            .filter_map(|(pid, p)| {
                let pid = pid.as_u32();
                let memory = p.memory();

                // TODO: Investigate why p.cmd() sometimes returns an empty array
                let cmd_list = p.cmd();
                if cmd_list.is_empty() {
                    return None;
                }

                Some((
                    pid,
                    Process {
                        pid,
                        type_: match p.thread_kind() {
                            Some(tk) => match tk {
                                ThreadKind::Kernel => ProcessType::KernelThread,
                                ThreadKind::Userland => ProcessType::UserThread,
                            },
                            None => ProcessType::Cpu,
                        },
                        command: cmd_list
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                            .join(" "),
                        memory,
                        memory_usage: (memory as f32 / total_memory as f32) * 100.0,
                        cpu_usage: p.cpu_usage(),
                    },
                ))
            })
            .collect();

        // find which ones are GPU and mark them as such
        if let Some(n) = nvml {
            if let Ok(pids) = _gpu_graphics_pids(n) {
                _update_process_type(pids, &mut processes, ProcessType::GpuCompute)
            }
            if let Ok(pids) = _gpu_graphics_pids(n) {
                _update_process_type(pids, &mut processes, ProcessType::GpuGraphic)
            }
        }

        ProcessesSnapshot {
            processes: processes.into_values().collect(),
        }
    }
}

fn _gpu_compute_pids(nvml: &Nvml) -> Result<Vec<u32>, NvmlError> {
    let device = nvml.device_by_index(0)?;
    Ok(device
        .running_compute_processes()?
        .iter()
        .map(|pi| pi.pid)
        .collect())
}

fn _gpu_graphics_pids(nvml: &Nvml) -> Result<Vec<u32>, NvmlError> {
    let device = nvml.device_by_index(0)?;
    Ok(device
        .running_graphics_processes()?
        .iter()
        .map(|pi| pi.pid)
        .collect())
}

fn _update_process_type(
    pids: Vec<u32>,
    processes: &mut HashMap<u32, Process>,
    process_type: ProcessType,
) {
    for pid in pids {
        if let Some(obj) = processes.get_mut(&pid) {
            obj.type_ = process_type.clone();
        }
    }
}
