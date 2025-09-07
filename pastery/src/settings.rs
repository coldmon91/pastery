use serde::{Deserialize, Serialize};
use std::fs;
use rdev::Key;
use log::{warn, error};

const SETTINGS_FILE: &str = "pastery.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct KeyBinding {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub key: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub copy_key: KeyBinding,
    pub paste_key: KeyBinding,
    pub server_port: u16,
    pub max_clipboard_items: usize,
    pub db_path: String,
}

impl Default for Settings {
    fn default() -> Self {
        let tmp_dir = std::env::temp_dir().join("Pastery");
        if !tmp_dir.exists() {
            if let Err(e) = std::fs::create_dir_all(&tmp_dir) {
                error!("Failed to create temp directory {:?}: {}", tmp_dir, e);
            }
        }
        let db_path = tmp_dir.join("clip.db").to_str().unwrap().to_string();
        println!("Using temp directory for database: {:?}", db_path);
        Settings {
            copy_key: KeyBinding {
                ctrl: true,
                alt: false,
                shift: false,
                key: "c".to_string(),
            },
            paste_key: KeyBinding {
                ctrl: true,
                alt: false,
                shift: false,
                key: "v".to_string(),
            },
            server_port: 3030,
            max_clipboard_items: 1000, // 기본값: 1000개
            db_path: db_path,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        match fs::read_to_string(SETTINGS_FILE) {
            Ok(content) => {
                match serde_json::from_str(&content) {
                    Ok(settings) => settings,
                    Err(e) => {
                        warn!("Failed to parse {}: {}. Using default settings.", SETTINGS_FILE, e);
                        Self::default()
                    }
                }
            }
            Err(_) => {
                warn!("{} not found. Creating default settings file.", SETTINGS_FILE);
                let default_settings = Self::default();
                if let Err(e) = default_settings.save() {
                    error!("Failed to create {}: {}", SETTINGS_FILE, e);
                }
                default_settings
            }
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(SETTINGS_FILE, content)?;
        Ok(())
    }
}

pub fn key_binding_to_keys(binding: &KeyBinding) -> Vec<Key> {
    let mut keys = Vec::new();
    
    if binding.ctrl {
        keys.push(Key::ControlLeft);
    }
    if binding.alt {
        keys.push(Key::Alt);
    }
    if binding.shift {
        keys.push(Key::ShiftLeft);
    }
    
    // 문자열을 Key로 변환
    let main_key = match binding.key.to_lowercase().as_str() {
        "a" => Key::KeyA,
        "b" => Key::KeyB,
        "c" => Key::KeyC,
        "d" => Key::KeyD,
        "e" => Key::KeyE,
        "f" => Key::KeyF,
        "g" => Key::KeyG,
        "h" => Key::KeyH,
        "i" => Key::KeyI,
        "j" => Key::KeyJ,
        "k" => Key::KeyK,
        "l" => Key::KeyL,
        "m" => Key::KeyM,
        "n" => Key::KeyN,
        "o" => Key::KeyO,
        "p" => Key::KeyP,
        "q" => Key::KeyQ,
        "r" => Key::KeyR,
        "s" => Key::KeyS,
        "t" => Key::KeyT,
        "u" => Key::KeyU,
        "v" => Key::KeyV,
        "w" => Key::KeyW,
        "x" => Key::KeyX,
        "y" => Key::KeyY,
        "z" => Key::KeyZ,
        _ => Key::KeyC, // 기본값
    };
    
    keys.push(main_key);
    keys
}
