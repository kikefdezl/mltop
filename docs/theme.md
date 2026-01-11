# Theme Customization

In `~/.config/mltop/config.toml`, use the `[theme]` section to tweak your
palette. You can use Ratatui color names or hex values. Examples:

```toml
[theme]
line_graph_cpu = "blue"
line_graph_gpu_mem = "light-green"
line_graph_gpu_use = "#FF5F15"
```

List of all configurable elements (see `src/config/theme.rs`):

`line_graph_cpu`
`line_graph_mem`
`line_graph_gpu_use`
`line_graph_gpu_mem`
`bar_low_use`
`bar_medium_use`
`bar_medium_high_use`
`bar_high_use`
`processes_header_fg`
`processes_header_bg`
`processes_cpu`
`processes_thread`
`processes_gpu_compute`
`processes_gpu_graphic`
`processes_bin_name`
`processes_selected_fg`
`processes_selected_bg`
`action_bar_msg_bg`
`action_bar_msg_fg`
`action_bar_cmd_bg`
`action_bar_cmd_fg`
`action_bar_key_bg`
`action_bar_key_fg`
