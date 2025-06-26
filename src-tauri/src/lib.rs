// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod ipc;
mod picker;
mod event;

use crate::ipc::{get_point, greet, pick_colors, start_mouse_hook, stop_mouse_hook, is_mouse_hook_running, get_mouse_events};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_point,
            pick_colors,
            start_mouse_hook,
            stop_mouse_hook,
            is_mouse_hook_running,
            get_mouse_events,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
