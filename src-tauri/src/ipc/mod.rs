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

mod test {
    use super::*;

    #[test]
    fn test_greet() {
        let name = "World";
        let expected = "Hello, World! You've been greeted from Rust!";
        assert_eq!(greet(name), expected);
    }
}
