use sysinfo::System;

fn main() {
    let memory = get_memory();
    println!("Memory: {}", memory);

    let cpu_usages = get_cpu_usages();
    for cpu_usage in cpu_usages {
        println!("CPU: {}", cpu_usage);
    }
}

fn get_memory() -> u64 {
    let sys = System::new_all();
    sys.total_memory()
}

fn get_cpu_usages() -> Vec<f32> {
    let sys = System::new_all();
    let mut usages: Vec<f32> = Vec::new();
    for cpu in sys.cpus() {
        let usage: f32 = cpu.cpu_usage();
        usages.push(usage);
    }
    usages
}
