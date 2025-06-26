use crate::picker::{Point, RGB, pick_color};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

use tauri::{AppHandle, Manager, Emitter};
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

#[cfg(windows)]
static mut GLOBAL_APP_HANDLE: Option<Arc<Mutex<Option<AppHandle>>>> = None;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmitMouseClickEvent {
    pub x: i32,
    pub y: i32,
    pub button: MouseButton,
    pub rgb: Option<RGB>,
}

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

    pub fn start(&mut self, app: AppHandle) {
        #[cfg(windows)]
        {
            if self.hook_handle.is_none() {
                self.start_hook_mouse_click(app);
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
        if !result.is_empty() {
            println!("[マウスフック] {}個のイベントを取得しました", result.len());
        }
        result
    }

    pub fn start_hook_mouse_click(&mut self, app: AppHandle) {
        #[cfg(windows)]
        unsafe {
            if self.hook_handle.is_some() {
                println!("[マウスフック] 既にフック中のため開始をスキップ");
                return; // 既にフック中
            }

            println!("[マウスフック] フック開始処理を実行中...");

            // グローバル変数にイベントキューとAppHandleの参照を設定
            GLOBAL_EVENTS = Some(Arc::clone(&self.events));
            GLOBAL_APP_HANDLE = Some(Arc::new(Mutex::new(Some(app))));

            let hook = SetWindowsHookExW(
                WH_MOUSE_LL,
                Some(low_level_mouse_proc),
                GetModuleHandleW(std::ptr::null()),
                0,
            );

            if !hook.is_null() {
                self.hook_handle = Some(hook);
                println!("[マウスフック] フック開始成功！マウスクリックの監視を開始しました");
            } else {
                println!("[マウスフック] エラー: フック開始に失敗しました");
            }
        }
    }

    pub fn stop_hook_mouse_click(&mut self) {
        #[cfg(windows)]
        unsafe {
            if let Some(hook) = self.hook_handle {
                println!("[マウスフック] フック停止処理を実行中...");
                UnhookWindowsHookEx(hook);
                self.hook_handle = None;
                GLOBAL_EVENTS = None;
                GLOBAL_APP_HANDLE = None;
                println!("[マウスフック] フック停止完了！マウス監視を終了しました");
            } else {
                println!("[マウスフック] フックが開始されていないため停止をスキップ");
            }
        }
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

            let event = MouseClickEvent { button: button.clone(), point };

            // STDOUTにクリックイベントをログ出力
            println!("[マウスフック] {:?}ボタンクリック検知: 座標({}, {})",
                     button, point.x, point.y);

            // グローバルイベントキューにイベントを追加
            if let Some(ref events) = GLOBAL_EVENTS {
                if let Ok(mut queue) = events.lock() {
                    queue.push_back(event.clone());
                    // キューサイズを制限（メモリリーク防止）
                    if queue.len() > 1000 {
                        queue.pop_front();
                    }
                    println!("[マウスフック] イベントキューサイズ: {}", queue.len());
                }
            }

            // AppHandleを使用した処理（例：フロントエンドへのイベント送信）
            if let Some(ref app_handle_arc) = GLOBAL_APP_HANDLE {
                if let Ok(app_handle_lock) = app_handle_arc.lock() {
                    if let Some(ref app_handle) = *app_handle_lock {
                        // AppHandleを使用した例：基本的なTauri機能

                        println!("[マウスフック] AppHandleが利用可能になりました");

                        // TODO: Tauri v2のイベントAPIを使用したい場合は以下のようなパターンを検討
                        // let _ = app_handle.emit_to("main", "mouse-click", &event);

                        // ウィンドウ操作（main window取得）
                        if let Some(window) = app_handle.get_webview_window("main") {

                            let rgb = pick_color(point);
                            let data: EmitMouseClickEvent = EmitMouseClickEvent {
                                x: point.x,
                                y: point.y,
                                button: button.clone(),
                                rgb,
                            };


                            window.emit("mouse-click", data)
                                .expect("[マウスフック] イベント送信に失敗しました");
                        }

                        // AppHandleから利用可能な主要機能:
                        // - app_handle.path() - パス関連操作
                        // - app_handle.state::<T>() - アプリ状態管理
                        // - app_handle.manage() - 状態の登録
                        // - app_handle.get_webview_window() - ウィンドウ取得
                        // など多数のTauri機能が利用可能
                    }
                }
            }
        }
    }

    CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param)
}
