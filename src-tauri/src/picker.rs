use serde::{Deserialize, Serialize};

pub type RGB = (u8, u8, u8);

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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

/// 指定範囲のRGB値をVec<RGB>で返す。終点がNoneなら始点1点のみ。
pub fn pick_colors(start: Point, end: Point) -> Option<Vec<RGB>> {
    #[cfg(not(windows))]
    {
        return None;
    }
    #[cfg(windows)]
    unsafe {
        use winapi::um::winuser::{GetDC, ReleaseDC, GetDesktopWindow};
        use winapi::um::wingdi::{CreateCompatibleDC, CreateCompatibleBitmap, SelectObject, BitBlt, GetDIBits, SRCCOPY, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS};
        use winapi::shared::windef::HWND;
        use winapi::shared::minwindef::{DWORD, UINT};
        use std::mem;

        let hwnd: HWND = GetDesktopWindow();
        let hdc = GetDC(hwnd);
        if hdc.is_null() {
            return None;
        }
        let mem_dc = CreateCompatibleDC(hdc);

        // 範囲計算
        let x1 = start.x.min(end.x);
        let y1 = start.y.min(end.y);
        let x2 = start.x.max(end.x);
        let y2 = start.y.max(end.y);

        let width = (x2 - x1 + 1).max(1);
        let height = (y2 - y1 + 1).max(1);

        let bmp = CreateCompatibleBitmap(hdc, width, height);
        if bmp.is_null() {
            ReleaseDC(hwnd, hdc);
            return None;
        }
        SelectObject(mem_dc, bmp as _);
        if BitBlt(mem_dc, 0, 0, width, height, hdc, x1, y1, SRCCOPY) == 0 {
            ReleaseDC(hwnd, hdc);
            return None;
        }
        // ビットマップ情報構造体
        let mut bmi: BITMAPINFO = mem::zeroed();
        bmi.bmiHeader.biSize = mem::size_of::<BITMAPINFOHEADER>() as DWORD;
        bmi.bmiHeader.biWidth = width;
        bmi.bmiHeader.biHeight = -height; // top-down
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 24;
        bmi.bmiHeader.biCompression = 0;
        bmi.bmiHeader.biSizeImage = 0;
        bmi.bmiHeader.biXPelsPerMeter = 0;
        bmi.bmiHeader.biYPelsPerMeter = 0;
        bmi.bmiHeader.biClrUsed = 0;
        bmi.bmiHeader.biClrImportant = 0;
        let row_stride = ((24 * width as usize + 31) / 32) * 4;
        let mut buf = vec![0u8; row_stride * height as usize];
        if GetDIBits(mem_dc, bmp, 0, height as UINT, buf.as_mut_ptr() as *mut _, &mut bmi, DIB_RGB_COLORS) == 0 {
            ReleaseDC(hwnd, hdc);
            return None;
        }
        // RGB配列に変換
        let mut result = Vec::with_capacity((width * height) as usize);
        for y in 0..height as usize {
            for x in 0..width as usize {
                let offset = y * row_stride + x * 3;
                let b = buf[offset];
                let g = buf[offset + 1];
                let r = buf[offset + 2];
                result.push((r, g, b));
            }
        }
        ReleaseDC(hwnd, hdc);
        Some(result)
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
                println!("Picked color at ({}, {}): RGB({}, {}, {})", point.x, point.y, color.0, color.1, color.2);
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

    #[test]
    fn test_pick_colors() {
        if let Some(start) = get_cursor_position() {
            // 5x5の範囲をテスト
            let end = Point { x: start.x + 4, y: start.y + 4 };
            if let Some(colors) = pick_colors(start, end) {
                println!("Picked {} colors in area (({}, {})-({}, {}))", colors.len(), start.x, start.y, end.x, end.y);
                assert_eq!(colors.len(), 25);
                for (i, rgb) in colors.iter().enumerate() {
                    assert!(rgb.0 <= 255 && rgb.1 <= 255 && rgb.2 <= 255, "Invalid RGB at {}: {:?}", i, rgb);
                }
            } else {
                println!("Failed to pick colors in area");
                assert!(false, "pick_colors failed");
            }
        } else {
            println!("Failed to get cursor position");
            assert!(false, "Cursor position retrieval failed");
        }
    }
}
