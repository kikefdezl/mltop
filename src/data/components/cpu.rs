use sysinfo::{Components, System};

use std::collections::HashMap;

#[derive(Clone)]
pub struct Core {
    pub usage: f32,
    pub temp: f32,
}

#[derive(Clone)]
pub struct Cpu {
    pub usage: Vec<f32>, // as a value between 0.0 and 100.0
    pub cores: Vec<Core>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            usage: vec![],
            cores: vec![],
        }
    }

    pub fn update(&mut self, sys: &System) {
        let mut cores: Vec<Core> = Vec::new();

        // TODO: Fix temperature mismatched for all cores. Have to find a more robust way
        // to find the 1:1 core temperatures.

        // gather temperatures from Components in to a map
        let components = Components::new_with_refreshed_list();
        let mut temperatures = HashMap::new();
        for component in &components {
            let label = component.label();
            if label.contains("Core") {
                if let Ok(id) = label.split_whitespace().last().unwrap().parse::<usize>() {
                    temperatures.insert(id, component.temperature());
                }
            }
        }

        for cpu in sys.cpus() {
            let id: usize = cpu.name()[3..].parse().unwrap();
            let usage: f32 = cpu.cpu_usage();
            let temperature: f32 = match temperatures.get(&id) {
                Some(t) => *t,
                None => 0.0,
            };
            let core = Core {
                usage,
                temp: temperature,
            };
            cores.push(core);
        }

        let usage = sys.global_cpu_info().cpu_usage();
        self.usage.push(usage);
        self.cores = cores;
    }
}
