use sysinfo::System;

fn main() {
    let memory = get_memory();
    println!("{}", memory);
}

fn get_memory() -> u64 {
    let sys = System::new_all();
    sys.total_memory()
}
