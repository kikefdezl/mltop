use cpu::Cpu;

mod cpu;

fn main() {
    let cpu = Cpu::read();
    for cpu_core in cpu.cores {
        println!("CPU: {}", cpu_core.usage);
    }
}
