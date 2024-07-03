use cpu::Cpu;
use memory::Memory;

mod cpu;
mod memory;

fn main() {
    let cpu = Cpu::read();
    let memory = Memory::read();

    cpu.display();
    memory.display();
}
