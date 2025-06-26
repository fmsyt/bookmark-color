use crate::picker::Point;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

#[cfg(windows)]
use winapi::{
    shared::{
        windef::{HHOOK},
        minwindef::{LPARAM, LRESULT, WPARAM},
    },
    um::{
        winuser::{
            SetWindowsHookExW, UnhookWindowsHookEx, CallNextHookEx,
            WH_MOUSE_LL, HC_ACTION, WM_LBUTTONDOWN, WM_RBUTTONDOWN, WM_MBUTTONDOWN,
            MSLLHOOKSTRUCT,
        },
        libloaderapi::GetModuleHandleW,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseClickEvent {
    pub button: MouseButton,
    pub point: Point,
}

pub struct Watcher {
    #[cfg(windows)]
    hook_handle: Option<HHOOK>,

    #[cfg(not(windows))]
    _placeholder: (),

    events: Arc<Mutex<VecDeque<MouseClickEvent>>>,
}

// Windowsでのスレッド安全性を保証（HHOOKは実際にはスレッド間で適切に管理される）
#[cfg(windows)]
unsafe impl Send for Watcher {}
#[cfg(windows)]
unsafe impl Sync for Watcher {}

// グローバルな状態管理（Windowsフック用）
#[cfg(windows)]
static mut GLOBAL_EVENTS: Option<Arc<Mutex<VecDeque<MouseClickEvent>>>> = None;

impl Watcher {
    pub fn new() -> Self {
        Watcher {
            #[cfg(windows)]
            hook_handle: None,

            #[cfg(not(windows))]
            _placeholder: (),

            events: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn start(&mut self) {
        #[cfg(windows)]
        {
            if self.hook_handle.is_none() {
                self.start_hook_mouse_click();
            }
        }
    }

    pub fn stop(&mut self) {
        #[cfg(windows)]
        {
            if self.hook_handle.is_some() {
                self.stop_hook_mouse_click();
            }
        }
    }

    pub fn is_running(&self) -> bool {
        #[cfg(windows)]
        {
            self.hook_handle.is_some()
        }

        #[cfg(not(windows))]
        {
            false
        }
    }

    pub fn get_events(&self) -> Vec<MouseClickEvent> {
        let mut events = self.events.lock().unwrap();
        let result: Vec<_> = events.drain(..).collect();
        result
    }

    pub fn start_hook_mouse_click(&mut self) {
        #[cfg(windows)]
        unsafe {
            if self.hook_handle.is_some() {
                return; // 既にフック中
            }

            // グローバル変数にイベントキューの参照を設定
            GLOBAL_EVENTS = Some(Arc::clone(&self.events));

            let hook = SetWindowsHookExW(
                WH_MOUSE_LL,
                Some(low_level_mouse_proc),
                GetModuleHandleW(std::ptr::null()),
                0,
            );

            if !hook.is_null() {
                self.hook_handle = Some(hook);
            }
        }
    }

    pub fn stop_hook_mouse_click(&mut self) {
        #[cfg(windows)]
        unsafe {
            if let Some(hook) = self.hook_handle {
                UnhookWindowsHookEx(hook);
                self.hook_handle = None;
                GLOBAL_EVENTS = None;
            }
        }
    }

    fn make_handler<F>(&mut self)
    where
        F: FnMut(MouseClickEvent) + Send + 'static,
    {
        // SetWindowsHookExW を使って、クリックされたときのカーソル位置取得する関数を返す
        // この実装は上記のstart_hook_mouse_clickメソッドで完了している
        // カスタムハンドラーが必要な場合は、この関数を拡張可能
    }
}

#[cfg(windows)]
unsafe extern "system" fn low_level_mouse_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code >= HC_ACTION {
        let button = match w_param as u32 {
            WM_LBUTTONDOWN => Some(MouseButton::Left),
            WM_RBUTTONDOWN => Some(MouseButton::Right),
            WM_MBUTTONDOWN => Some(MouseButton::Middle),
            _ => None,
        };

        if let Some(button) = button {
            // フック構造体からマウス位置を取得
            let hook_struct = *(l_param as *const MSLLHOOKSTRUCT);
            let point = Point {
                x: hook_struct.pt.x,
                y: hook_struct.pt.y,
            };

            let event = MouseClickEvent { button, point };

            // グローバルイベントキューにイベントを追加
            if let Some(ref events) = GLOBAL_EVENTS {
                if let Ok(mut queue) = events.lock() {
                    queue.push_back(event);
                    // キューサイズを制限（メモリリーク防止）
                    if queue.len() > 1000 {
                        queue.pop_front();
                    }
                }
            }
        }
    }

    CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param)
}
