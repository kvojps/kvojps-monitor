// Impede uma janela de console extra no Windows em release; inofensivo no Linux.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    kvojps_monitor_lib::run()
}
