fn main() {
    // this script was mainly a sanity check to see which fields are available on my GPU
    // run with cargo run --example nvidia_check
    match nvml_wrapper::Nvml::init() {
        Ok(nvml) => {
            println!("NVML init OK");
            println!("Device count: {:?}", nvml.device_count());
            match nvml.device_by_index(0) {
                Ok(dev) => {
                    println!("Device 0 name: {:?}", dev.name());
                    println!(
                        "Temperature: {:?}",
                        dev.temperature(
                            nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu
                        )
                    );
                    println!("Memory: {:?}", dev.memory_info());
                    println!("Utilization: {:?}", dev.utilization_rates());
                    println!(
                        "Power limit (power_management_limit): {:?}",
                        dev.power_management_limit()
                    );
                    println!(
                        "Power limit (enforced_power_limit): {:?}",
                        dev.enforced_power_limit()
                    );
                    println!(
                        "Power limit (power_management_limit_default): {:?}",
                        dev.power_management_limit_default()
                    );
                    println!(
                        "Power limit (power_management_limit_constraints): {:?}",
                        dev.power_management_limit_constraints()
                    );
                    println!("Power usage: {:?}", dev.power_usage());
                    println!("Fan speed: {:?}", dev.fan_speed(0));
                    println!("Number of fans: {:?}", dev.num_fans());
                }
                Err(e) => println!("device_by_index(0) error: {:?}", e),
            }
        }
        Err(e) => println!("NVML init error: {:?}", e),
    }
}
