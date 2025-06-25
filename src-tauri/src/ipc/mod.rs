use crate::picker::{self, Point};

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

mod test {
    use super::*;

    #[test]
    fn test_greet() {
        let name = "World";
        let expected = "Hello, World! You've been greeted from Rust!";
        assert_eq!(greet(name), expected);
    }
}
