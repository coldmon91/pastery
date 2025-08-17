use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::localization::Language;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub language: Language,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: Language::English,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let settings_path = Self::get_settings_path();
        
        match fs::read_to_string(&settings_path) {
            Ok(content) => {
                // 파일을 성공적으로 읽은 경우 파싱 시도
                if let Ok(settings) = serde_json::from_str::<Settings>(&content) {
                    settings
                } else {
                    // 파싱에 실패한 경우 기본값 반환 (파일 덮어쓰지 않음)
                    Self::default()
                }
            }
            Err(_) => {
                // 파일이 없는 경우에만 기본 설정 파일 생성
                let default_settings = Self::default();
                let _ = default_settings.save(); // 기본 설정 파일 생성
                default_settings
            }
        }
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings_path = Self::get_settings_path();
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&settings_path, json)?;
        Ok(())
    }
    
    fn get_settings_path() -> PathBuf {
        PathBuf::from("settings.json")
    }
}
