use action_bar::ActionBarWidget;
use cpu::CpuWidget;
use gpu::GpuWidget;
use line_graph::LineGraphWidget;
use memory::MemoryWidget;
use process_table::ProcessTableWidget;

pub mod action_bar;
pub mod cpu;
pub mod gpu;
pub mod line_graph;
pub mod memory;
pub mod percentage_bar;
pub mod process_table;
pub mod state;

#[derive(Default)]
pub struct Widgets {
    pub cpu: CpuWidget,
    pub memory: MemoryWidget,
    pub line_graph: LineGraphWidget,
    pub gpu: GpuWidget,
    pub process_table: ProcessTableWidget,
    pub action_bar: ActionBarWidget,
}

impl Widgets {
    pub fn new() -> Widgets {
        Widgets {
            cpu: CpuWidget::new(),
            memory: MemoryWidget::new(),
            line_graph: LineGraphWidget::new(),
            gpu: GpuWidget::new(),
            process_table: ProcessTableWidget::new(),
            action_bar: ActionBarWidget::new(),
        }
    }

    pub fn cpu(&self) -> &CpuWidget {
        &self.cpu
    }

    pub fn memory(&self) -> &MemoryWidget {
        &self.memory
    }

    pub fn line_graph(&self) -> &LineGraphWidget {
        &self.line_graph
    }

    pub fn gpu(&self) -> &GpuWidget {
        &self.gpu
    }

    pub fn process_table(&self) -> &ProcessTableWidget {
        &self.process_table
    }
}
