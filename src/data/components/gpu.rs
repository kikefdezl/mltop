use nvml_wrapper::{error::NvmlError, Nvml};

use nvml_wrapper::enum_wrappers::device::TemperatureSensor;

#[derive(Clone)]
pub struct Gpu {
    pub name: String,
    pub temperature: u32,

    pub max_memory: u64,
    pub used_memory: Vec<u64>,

    pub utilization: Vec<u32>,

    pub max_power: u32,
    pub power_usage: u32,
}

impl Gpu {
    pub fn read(nvml: &Option<Nvml>) -> Result<Gpu, NvmlError> {
        let nvml = match nvml.as_ref() {
            Some(b) => Ok(b),
            None => Err(NvmlError::LibraryNotFound),
        };

        let device = nvml?.device_by_index(0)?;

        let memory_info = device.memory_info()?;
        Ok(Gpu {
            name: device.name()?,
            temperature: device.temperature(TemperatureSensor::Gpu)?,
            max_memory: memory_info.total,
            used_memory: vec![memory_info.used],
            utilization: vec![device.utilization_rates()?.gpu],
            max_power: device.power_management_limit()?,
            power_usage: device.power_usage()?,
        })
    }

    pub fn update(&mut self) {
        let nvml = match Nvml::init() {
            Err(_) => return,
            Ok(nvml) => nvml,
        };
        let device = match nvml.device_by_index(0) {
            Err(_) => return,
            Ok(device) => device,
        };
        let memory_info = match device.memory_info() {
            Err(_) => return,
            Ok(memory_info) => memory_info,
        };
        let utilization = match device.utilization_rates() {
            Err(_) => return,
            Ok(rates) => rates.gpu,
        };

        self.used_memory.push(memory_info.used);
        self.utilization.push(utilization);
    }
}
