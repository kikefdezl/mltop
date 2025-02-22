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

    pub fan_speed: u32,
}

impl Gpu {
    pub fn new(nvml: &Option<Nvml>) -> Result<Gpu, NvmlError> {
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
            fan_speed: device.fan_speed(0)?,
        })
    }

    pub fn update(&mut self, nvml: &Option<Nvml>) -> Result<(), NvmlError> {
        let nvml = match nvml.as_ref() {
            Some(b) => Ok(b),
            None => Err(NvmlError::LibraryNotFound),
        };

        let device = nvml?.device_by_index(0)?;

        let memory_info = device.memory_info()?;
        self.temperature = device.temperature(TemperatureSensor::Gpu)?;
        self.used_memory.push(memory_info.used);
        self.utilization.push(device.utilization_rates()?.gpu);

        self.power_usage = device.power_usage()?;
        Ok(())
    }
}
