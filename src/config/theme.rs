use ratatui::style::Color;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Theme {
    pub line_graph_cpu: Color,
    pub line_graph_mem: Color,
    pub line_graph_gpu_use: Color,
    pub line_graph_gpu_mem: Color,

    pub bar_low_use: Color,
    pub bar_medium_use: Color,
    pub bar_medium_high_use: Color,
    pub bar_high_use: Color,
    pub bar_text: Color,

    pub processes_header_fg: Color,
    pub processes_header_bg: Color,
    pub processes_cpu: Color,
    pub processes_thread: Color,
    pub processes_gpu_compute: Color,
    pub processes_gpu_graphic: Color,
    pub processes_bin_name: Color,
    pub processes_selected_fg: Color,
    pub processes_selected_bg: Color,

    pub action_bar_msg_bg: Color,
    pub action_bar_msg_fg: Color,
    pub action_bar_cmd_bg: Color,
    pub action_bar_cmd_fg: Color,
    pub action_bar_key_bg: Color,
    pub action_bar_key_fg: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            line_graph_cpu: Color::Red,
            line_graph_mem: Color::Green,
            line_graph_gpu_use: Color::Blue,
            line_graph_gpu_mem: Color::Yellow,

            bar_low_use: Color::Green,
            bar_medium_use: Color::Yellow,
            bar_medium_high_use: Color::Rgb(255, 130, 0), // orange
            bar_high_use: Color::Red,
            bar_text: Color::DarkGray,

            processes_header_fg: Color::Black,
            processes_header_bg: Color::Green,
            processes_cpu: Color::White,
            processes_thread: Color::DarkGray,
            processes_gpu_compute: Color::Magenta,
            processes_gpu_graphic: Color::Yellow,
            processes_bin_name: Color::Magenta,
            processes_selected_fg: Color::Black,
            processes_selected_bg: Color::Cyan,

            action_bar_msg_bg: Color::Red,
            action_bar_msg_fg: Color::Black,
            action_bar_cmd_bg: Color::Cyan,
            action_bar_cmd_fg: Color::Black,
            action_bar_key_bg: Color::Reset,
            action_bar_key_fg: Color::White,
        }
    }
}
