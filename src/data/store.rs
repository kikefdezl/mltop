use super::snapshot::DataSnapshot;

pub struct StoredSnapshot {
    pub cpu_use: f32,
    pub gpu_use: Option<u32>,
    pub gpu_mem_use: Option<u64>,
}

impl StoredSnapshot {
    pub fn from_data_snapshot(snapshot: DataSnapshot) -> StoredSnapshot {
        let (gpu_use, gpu_mem_use) = snapshot
            .gpu
            .as_ref()
            .map(|g| (g.utilization.clone(), g.used_memory))
            .unzip();

        StoredSnapshot {
            cpu_use: snapshot.cpu.unwrap().usage,
            gpu_use,
            gpu_mem_use,
        }
    }
}

pub struct DataStore {
    pub snapshots: Vec<StoredSnapshot>,
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore { snapshots: vec![] }
    }

    pub fn save(&mut self, snapshot: StoredSnapshot) {
        self.snapshots.push(snapshot);
    }
}
