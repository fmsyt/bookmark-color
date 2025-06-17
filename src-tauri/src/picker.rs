use serde::Serialize;

pub type RGB = (u8, u8, u8);

#[derive(Serialize, Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn get_cursor_position() -> Option<Point> {
    #[cfg(not(windows))]
    {
        return None // Placeholder for non-Windows platforms
    }

    use winapi::shared::windef::POINT;
    use winapi::um::winuser::GetCursorPos;

    let mut point = POINT { x: 0, y: 0 };

    unsafe {
        let success = GetCursorPos(&mut point);

        if success != 0 {
            Some(Point {
                x: point.x,
                y: point.y,
            })
        } else {
            None
        }
    }
}

pub fn pick_color(point: Point) -> Option<RGB> {
    #[cfg(not(windows))]
    {
        return None; // Placeholder for non-Windows platforms
    }

    unsafe {
        use winapi::um::winuser::GetDC;
        use winapi::um::wingdi::GetPixel;
        use winapi::shared::windef::HWND;
        use winapi::um::winuser::GetDesktopWindow;

        let hwnd: HWND = GetDesktopWindow();
        let hdc = GetDC(hwnd);

        if hdc.is_null() {
            return None;
        }

        let pixel = GetPixel(hdc, point.x, point.y);
        if pixel == 0xFFFFFFFF { // Check for invalid pixel
            return None;
        }

        let r = (pixel & 0xFF) as u8;
        let g = ((pixel >> 8) & 0xFF) as u8;
        let b = ((pixel >> 16) & 0xFF) as u8;

        // Release the device context
        winapi::um::winuser::ReleaseDC(hwnd, hdc);

        Some((r, g, b))
    }
}

mod test {
    use super::*;

    #[test]
    fn test_get_cursor_position() {
        if let Some( Point { x, y }) = get_cursor_position() {
            println!("Cursor position: ({}, {})", x, y);
            assert!(x >= 0 && y >= 0);
        } else {
            println!("Failed to get cursor position");
            assert!(false, "Cursor position retrieval failed");
        }
    }

    #[cfg(not(windows))]
    #[test]
    fn test_get_cursor_position_not_windows() {
        println!("Cursor position retrieval is not implemented for this platform");
        assert!(true, "Test skipped for non-Windows platform");
    }

    #[test]
    fn test_pick_color() {
        if let Some(point) = get_cursor_position() {
            if let Some(color) = pick_color(point) {
                println!("Picked color at ({}, {}): RGB({}, {}, {})", point.0, point.1, color.0, color.1, color.2);
                assert!(color.0 <= 255 && color.1 <= 255 && color.2 <= 255);
            } else {
                println!("Failed to pick color at cursor position");
                assert!(false, "Color picking failed");
            }
        } else {
            println!("Failed to get cursor position");
            assert!(false, "Cursor position retrieval failed");
        }
    }

    #[cfg(not(windows))]
    #[test]
    fn test_pick_color_not_windows() {
        println!("Color picking is not implemented for this platform");
        assert!(true, "Test skipped for non-Windows platform");
    }
}
