use super::snapshot::DataSnapshot;

// Structure for storing only what we need to track
pub struct StoredSnapshot {
    pub cpu_use: f32,
    // memory use in percentage 0 - 1 (includes swap)
    pub mem_use: f64,
    pub gpu_use: Option<u32>,
    pub gpu_mem_use: Option<u64>,
}

impl StoredSnapshot {
    pub fn from_data_snapshot(snapshot: DataSnapshot) -> StoredSnapshot {
        let (gpu_use, gpu_mem_use) = snapshot
            .gpu
            .as_ref()
            .map(|g| (g.utilization, g.used_memory))
            .unzip();

        StoredSnapshot {
            cpu_use: snapshot.cpu.unwrap().usage,
            mem_use: snapshot.memory.unwrap().total_percent(),
            gpu_use,
            gpu_mem_use,
        }
    }
}

#[derive(Default)]
pub struct DataStore {
    pub snapshots: Vec<StoredSnapshot>,
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore::default()
    }

    pub fn save(&mut self, snapshot: StoredSnapshot) {
        self.snapshots.push(snapshot);
    }
}
