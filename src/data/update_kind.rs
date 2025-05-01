pub struct DataUpdateKind {
    cpu: bool,
    memory: bool,
    gpu: bool,
    processes: bool,
}

impl DataUpdateKind {
    pub fn all() -> DataUpdateKind {
        DataUpdateKind {
            cpu: true,
            memory: true,
            gpu: true,
            processes: true,
        }
    }

    pub fn without_processes(&self) -> DataUpdateKind {
        Self {
            cpu: self.cpu,
            memory: self.memory,
            gpu: self.gpu,
            processes: false,
        }
    }

    pub fn cpu(&self) -> bool {
        self.cpu
    }

    pub fn memory(&self) -> bool {
        self.memory
    }

    pub fn gpu(&self) -> bool {
        self.gpu
    }

    pub fn processes(&self) -> bool {
        self.processes
    }
}
