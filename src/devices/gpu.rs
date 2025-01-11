use nvml_wrapper::{error::NvmlError, Nvml};

use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};

pub struct Gpu {
    pub name: String,
    pub temperature: u32,
    pub max_mem: u64,
    pub avail_mem: u64,
    pub use_perc: f32,
}

impl Gpu {
    pub fn read() -> Result<Gpu, NvmlError> {
        let nvml = Nvml::init()?;

        let device = nvml.device_by_index(0)?;

        let temperature = device.temperature(TemperatureSensor::Gpu)?;
        // let brand = device.brand()?; // GeForce on my system
        // let fan_speed = device.fan_speed(0)?; // Currently 17% on my system
        // let power_limit = device.enforced_power_limit()?; // 275k milliwatts on my system
        // let encoder_util = device.encoder_utilization()?; // Currently 0 on my system; Not encoding anything
        let memory_info = device.memory_info()?;
        Ok(Gpu {
            name: device.name()?,
            temperature,
            max_mem: memory_info.total,
            avail_mem: memory_info.free,
            use_perc: 0.5,
        })
    }
}
