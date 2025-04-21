use nvml_wrapper::{error::NvmlError, Nvml};

use nvml_wrapper::enum_wrappers::device::TemperatureSensor;

#[derive(Clone)]
pub struct GpuSnapshot {
    pub name: String,
    pub temperature: u32,
    pub max_memory: u64,
    pub used_memory: u64,
    pub utilization: u32,
    pub max_power: u32,
    pub power_usage: u32,
    pub fan_speed: u32,
}

impl GpuSnapshot {
    pub fn from_nvml(nvml: &Nvml) -> Result<GpuSnapshot, NvmlError> {
        let device = nvml.device_by_index(0)?;
        let memory_info = device.memory_info()?;

        Ok(GpuSnapshot {
            name: device.name()?,
            temperature: device.temperature(TemperatureSensor::Gpu)?,
            max_memory: memory_info.total,
            used_memory: memory_info.used,
            utilization: device.utilization_rates()?.gpu,
            max_power: device.power_management_limit()?,
            power_usage: device.power_usage()?,
            fan_speed: device.fan_speed(0)?,
        })
    }
}
