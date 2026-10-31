// Prevent an extra console window on Windows release builds; harmless on Linux.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Run by the Claude Code status line wrapper: copy rate_limits and exit.
    let args: Vec<String> = std::env::args().collect();
    if let [_, mode, cache] = args.as_slice() {
        if mode == "statusline" {
            return kvojps_monitor_lib::copy_status_line(cache);
        }
    }
    kvojps_monitor_lib::run()
}
