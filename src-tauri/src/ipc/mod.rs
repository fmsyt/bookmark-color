use tauri::AppHandle;

use crate::picker::{self, Point};
use crate::event::{Watcher, MouseClickEvent};
use std::sync::{Arc, Mutex, OnceLock};

// グローバルなWatcherインスタンス
static GLOBAL_WATCHER: OnceLock<Arc<Mutex<Watcher>>> = OnceLock::new();

fn get_watcher() -> Arc<Mutex<Watcher>> {
    GLOBAL_WATCHER.get_or_init(|| {
        Arc::new(Mutex::new(Watcher::new()))
    }).clone()
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn get_point() -> Option<Point> {
    let point = picker::get_cursor_position();
    point
}

#[tauri::command]
pub fn pick_colors(p1: Point, p2: Point) -> Vec<picker::RGB> {
    let list = picker::pick_colors(p1, p2);
    list.unwrap_or_default()
}

#[tauri::command]
pub fn start_mouse_hook(app: AppHandle) -> bool {
    let watcher = get_watcher();
    let mut w = watcher.lock().unwrap();
    w.start(app);
    w.is_running()
}

#[tauri::command]
pub fn stop_mouse_hook() -> bool {
    let watcher = get_watcher();
    let mut w = watcher.lock().unwrap();
    w.stop();
    !w.is_running()
}

#[tauri::command]
pub fn is_mouse_hook_running() -> bool {
    let watcher = get_watcher();
    let w = watcher.lock().unwrap();
    w.is_running()
}

#[tauri::command]
pub fn get_mouse_events() -> Vec<MouseClickEvent> {
    let watcher = get_watcher();
    let w = watcher.lock().unwrap();
    w.get_events()
}

mod test {
    use super::*;

    #[test]
    fn test_greet() {
        let name = "World";
        let expected = "Hello, World! You've been greeted from Rust!";
        assert_eq!(greet(name), expected);
    }
}
