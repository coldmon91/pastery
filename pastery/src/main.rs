// this program is Pastery's background server
// it runs in the background and listens for keyboard events
// it communicates with the GUI program via a message channel
// it uses a database to store clipboard history
mod database;
mod key_combination;
mod server;
mod settings;

use std::sync::{mpsc, Arc, Mutex};
use arboard::Clipboard;
use rdev::{listen, Event};
use settings::Settings;

fn create_key_combination_from_settings(binding: &settings::KeyBinding) -> key_combination::KeyCombination {
    let keys = settings::key_binding_to_keys(binding);
    if keys.len() >= 2 {
        key_combination::KeyCombination::new(keys[0], keys[1])
    } else {
        // 기본값으로 fallback
        key_combination::KeyCombination::new(rdev::Key::ControlLeft, rdev::Key::KeyC)
    }
}

fn key_event_handle(
    channel: mpsc::Receiver<Event>, 
    clipboard_data: Arc<Mutex<database::ClipboardData>>,
    settings: Settings,
) {
    let mut copy_key_combination = create_key_combination_from_settings(&settings.copy_key);
    let mut paste_key_combination = create_key_combination_from_settings(&settings.paste_key);
    
    loop {
        match channel.recv() {
            Ok(event) => {
                match event.event_type {
                    rdev::EventType::KeyRelease(key) => {
                        if paste_key_combination.contains(key) {
                            paste_key_combination.release_key(key);
                        }
                        if copy_key_combination.contains(key) {
                            if copy_key_combination.is_active() {
                                let mut clipboard = Clipboard::new().unwrap();
                                if let Ok(text) = clipboard.get_text() {
                                    println!("Clipboard content: {}", text);
                                    let clipboard_data = clipboard_data.lock().unwrap();
                                    clipboard_data.write(&text);
                                }
                            }
                            copy_key_combination.release_key(key);
                        }
                    },
                    rdev::EventType::KeyPress(key) => {
                        if copy_key_combination.contains(key) {
                            copy_key_combination.press_key(key);
                        }

                        if paste_key_combination.contains(key) {
                            paste_key_combination.press_key(key);
                        }
                        // if paste_key_combination.is_active() {
                        //     // paste from user's choice - 최근 5개 클립보드 항목 표시
                        //     let clipboard_data = clipboard_data.lock().unwrap();
                        //     let items = clipboard_data.get_clipboard_items(Some(5));
                        //     for item in items {
                        //         println!("Clipboard data: {}-{}: \"{}\"", item.date, item.sequence, item.content);
                        //     }
                        // }
                    },
                    _ => {}
                }
            },
            Err(_) => break,
        }
    }
}

fn callback(event: Event, channel: mpsc::Sender<Event>) {
    channel.send(event.clone()).unwrap();
}

#[tokio::main]
async fn main() {
    println!("Pastery is running");
    
    // 설정 로드
    let settings = Settings::load();
    println!("Settings loaded. Server will run on port {}, max clipboard items: {}", 
             settings.server_port, settings.max_clipboard_items);
    
    // 통합 데이터베이스 초기화 (clip.db 파일 하나만 사용)
    let db_path = "clip.db".to_string();
    let clipboard_data = Arc::new(Mutex::new(database::ClipboardData::new(
        db_path.clone(),
        settings.max_clipboard_items
    )));
    
    // 키보드 이벤트 처리를 위한 채널
    let (tx, rx) = mpsc::channel();
    
    // 서버용 클립보드 데이터 복사
    let server_clipboard_data = clipboard_data.clone();
    let server_port = settings.server_port;
    
    // 서버 시작 (백그라운드)
    tokio::spawn(async move {
        server::start_server(server_clipboard_data, server_port).await;
    });
    
    // 키보드 이벤트 처리 스레드
    let keyboard_clipboard_data = clipboard_data.clone();
    let keyboard_settings = settings.clone();
    std::thread::spawn(move || {
        key_event_handle(rx, keyboard_clipboard_data, keyboard_settings);
    });
    
    // 키보드 리스너 시작
    if let Err(error) = listen(move |event| {
        callback(event, tx.clone())
    }) {
        println!("Error: {:?}", error)
    }
}
