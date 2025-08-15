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
            language: Language::Korean,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let settings_path = Self::get_settings_path();
        
        if let Ok(content) = fs::read_to_string(&settings_path) {
            if let Ok(settings) = serde_json::from_str::<Settings>(&content) {
                return settings;
            }
        }
        
        // 파일이 없거나 파싱에 실패한 경우 기본값 사용
        let default_settings = Self::default();
        let _ = default_settings.save(); // 기본 설정 파일 생성
        default_settings
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
