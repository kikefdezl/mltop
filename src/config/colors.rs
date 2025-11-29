use ratatui::style::Color;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct ColorConfig {
    pub line_graph_cpu: Color,
    pub line_graph_mem: Color,
    pub line_graph_gpu_use: Color,
    pub line_graph_gpu_mem: Color,

    pub processes_header_fg: Color,
    pub processes_header_bg: Color,
    pub processes_cpu: Color,
    pub processes_thread: Color,
    pub processes_gpu_compute: Color,
    pub processes_gpu_graphic: Color,
    pub processes_bin_name: Color,
}

impl Default for ColorConfig {
    fn default() -> Self {
        ColorConfig {
            line_graph_cpu: Color::Red,
            line_graph_mem: Color::Green,
            line_graph_gpu_use: Color::Blue,
            line_graph_gpu_mem: Color::Yellow,

            processes_header_fg: Color::Black,
            processes_header_bg: Color::White,
            processes_cpu: Color::White,
            processes_thread: Color::DarkGray,
            processes_gpu_compute: Color::Magenta,
            processes_gpu_graphic: Color::Yellow,
            processes_bin_name: Color::Magenta,
        }
    }
}
