use tauri::{
    generate_context, generate_handler,
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder},
    AppHandle, 
    Manager,
    Emitter,
};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, GlobalShortcutExt};
use serde::{Deserialize, Serialize};
use std::fs;

const SETTINGS_FILE: &str = "pastery-pop.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ClipboardItem {
    date: String,
    sequence: i32,
    content: String,
    memo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MemoItem {
    sequence: i32,
    memo: String,
}

// 프론트엔드에서 사용할 통합 아이템 구조체
#[derive(Debug, Serialize, Deserialize, Clone)]
struct DisplayItem {
    date: Option<String>,
    sequence: i32,
    content: String,
    memo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    hotkey: String,
    server_url: String,
    max_items_display: u32,
    popup_width: f64,
    popup_height: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            hotkey: "Ctrl+Shift+V".to_string(),
            server_url: "http://127.0.0.1:3030".to_string(),
            max_items_display: 5,
            popup_width: 350.0,
            popup_height: 450.0,
        }
    }
}

fn load_settings() -> Settings {
    match fs::read_to_string(SETTINGS_FILE) {
        Ok(content) => {
            match serde_json::from_str::<Settings>(&content) {
                Ok(settings) => settings,
                Err(_) => Settings::default(),
            }
        }
        Err(_) => Settings::default(),
    }
}


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_user_memos(count: Option<u32>) -> Result<Vec<DisplayItem>, String> {
    let settings = load_settings();
    let count = count.unwrap_or(settings.max_items_display);
    let url = format!("{}/memo?count={}", settings.server_url, count);
    
    let client = reqwest::Client::new();
    match client.get(&url).send().await {
        Ok(response) => {
            match response.json::<ApiResponse<Vec<MemoItem>>>().await {
                Ok(api_response) => {
                    if api_response.success {
                        let display_items = api_response.data.unwrap_or_default()
                            .into_iter()
                            .map(|memo| DisplayItem {
                                date: None,
                                sequence: memo.sequence,
                                content: memo.memo.clone(),
                                memo: Some(memo.memo),
                            })
                            .collect();
                        Ok(display_items)
                    } else {
                        Err(api_response.message)
                    }
                }
                Err(e) => Err(format!("Failed to parse response: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to fetch user memos: {}", e)),
    }
}

#[tauri::command]
async fn add_user_memo(memo_content: String) -> Result<(), String> {
    let settings = load_settings();
    let url = format!("{}/memo", settings.server_url);
    
    let memo_data = serde_json::json!({
        "memo": memo_content
    });
    
    let client = reqwest::Client::new();
    match client.post(&url)
        .header("Content-Type", "application/json")
        .json(&memo_data)
        .send().await {
        Ok(response) => {
            // 응답 상태 코드 확인
            let status = response.status();
            println!("Response status: {}", status);
            
            // 응답 텍스트를 먼저 읽어서 로깅
            match response.text().await {
                Ok(text) => {
                    println!("Response text: {}", text);
                    
                    // JSON 파싱 시도
                    match serde_json::from_str::<ApiResponse<serde_json::Value>>(&text) {
                        Ok(api_response) => {
                            if api_response.success {
                                Ok(())
                            } else {
                                Err(api_response.message)
                            }
                        }
                        Err(e) => Err(format!("Failed to parse JSON response '{}': {}", text, e)),
                    }
                }
                Err(e) => Err(format!("Failed to read response text: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to add user memo: {}", e)),
    }
}

#[tauri::command]
async fn get_clipboard_items(count: Option<u32>) -> Result<Vec<DisplayItem>, String> {
    let settings = load_settings();
    let count = count.unwrap_or(settings.max_items_display);
    let url = format!("{}/clipboard?count={}", settings.server_url, count);
    
    let client = reqwest::Client::new();
    match client.get(&url).send().await {
        Ok(response) => {
            match response.json::<ApiResponse<Vec<ClipboardItem>>>().await {
                Ok(api_response) => {
                    if api_response.success {
                        let display_items = api_response.data.unwrap_or_default()
                            .into_iter()
                            .map(|item| DisplayItem {
                                date: Some(item.date),
                                sequence: item.sequence,
                                content: item.content,
                                memo: item.memo,
                            })
                            .collect();
                        Ok(display_items)
                    } else {
                        Err(api_response.message)
                    }
                }
                Err(e) => Err(format!("Failed to parse response: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to fetch clipboard items: {}", e)),
    }
}

#[tauri::command]
async fn show_popup_at_cursor(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        // 마우스 커서 위치 가져오기
        let cursor_pos = get_cursor_position();
        let settings = load_settings();
        
        // 화면 경계를 고려하여 위치 조정
        let x = if cursor_pos.0 + settings.popup_width as i32 > 1920 {
            cursor_pos.0 - settings.popup_width as i32
        } else {
            cursor_pos.0
        };
        
        let y = if cursor_pos.1 + settings.popup_height as i32 > 1080 {
            cursor_pos.1 - settings.popup_height as i32
        } else {
            cursor_pos.1
        };
        
        let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(x, y)));
        let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize::new(
            settings.popup_width as u32, 
            settings.popup_height as u32
        )));
        let _ = window.show();
        let _ = window.set_focus();
        
        // 팝업이 표시될 때 클립보드 아이템 갱신 이벤트 발생
        let _ = window.emit_to("main", "refresh-clipboard", ());
    }
    Ok(())
}

fn get_cursor_position() -> (i32, i32) {
    // Windows API를 사용하여 마우스 위치 가져오기
    #[cfg(target_os = "windows")]
    {
        #[repr(C)]
        struct POINT {
            x: i32,
            y: i32,
        }
        
        extern "system" {
            fn GetCursorPos(lpPoint: *mut POINT) -> i32;
        }
        
        let mut point = POINT { x: 0, y: 0 };
        unsafe {
            GetCursorPos(&mut point);
        }
        (point.x, point.y)
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // 다른 OS의 경우 기본 위치 반환
        (400, 300)
    }
}

#[tauri::command]
async fn hide_popup(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    Ok(())
}

fn create_tray(app: &AppHandle) -> tauri::Result<()> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i])?;

    let _ = TrayIconBuilder::with_id("tray")
        .menu(&menu)
        .on_menu_event(move |_app, event| match event.id.as_ref() {
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        })
        .build(app);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let background_handle = std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            pastery::run_pastery();
        });
    });
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(move |app| {
            let _settings = load_settings();

            // 시스템 트레이 설정
            create_tray(app.handle())?;
            
            // 메인 윈도우를 처음에는 숨김
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }

            // Register global shortcut
            let app_handle = app.handle().clone();
            let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV);
            
            match app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, _event| {
                let app_handle = app_handle.clone();
                tauri::async_runtime::spawn(async move {
                    let _ = show_popup_at_cursor(app_handle).await;
                });
            }) {
                Ok(_) => println!("Global shortcut registered successfully"),
                Err(e) => {
                    eprintln!("Failed to register global shortcut: {}", e);
                    // 이미 등록된 shortcut이 있을 수 있으므로 앱을 계속 실행
                }
            }

            Ok(())
        })
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::Focused(false) = event {
                _window.hide().unwrap();
            }
        })
        .invoke_handler(generate_handler![
            greet,
            get_clipboard_items,
            get_user_memos,
            add_user_memo,
            show_popup_at_cursor,
            hide_popup
        ])
        .run(generate_context!())
        .expect("error while running tauri application");

    let _ = background_handle.join();
}