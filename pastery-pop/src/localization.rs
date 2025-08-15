use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Korean,
    English,
}

impl Language {
    pub fn font(&self) -> iced::Font {
        match self {
            Language::Korean => iced::Font::with_name("Malgun Gothic"),
            Language::English => iced::Font::DEFAULT,
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::Korean => write!(f, "한국어"),
            Language::English => write!(f, "English"),
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::Korean
    }
}

#[derive(Debug, Clone)]
pub struct Texts {
    pub app_title: &'static str,
    pub app_description: &'static str,
    pub refresh: &'static str,
    pub loading: &'static str,
    pub status: &'static str,
    pub retry: &'static str,
    pub no_items: &'static str,
    pub server_check: &'static str,
    pub total_items: &'static str,
    pub memo: &'static str,
    pub copy_failed: &'static str,
    pub copied_to_clipboard: &'static str,
    pub language: &'static str,
}

impl Texts {
    pub fn new(language: Language) -> Self {
        match language {
            Language::Korean => Self::korean(),
            Language::English => Self::english(),
        }
    }

    fn korean() -> Self {
        Self {
            app_title: "Pastery Pop - 클립보드 관리자",
            app_description: "Pastery 서버의 클립보드 항목들을 관리할 수 있습니다.",
            refresh: "새로고침",
            loading: "클립보드 항목을 불러오는 중...",
            status: "상태",
            retry: "다시 시도",
            no_items: "클립보드 항목이 없습니다.",
            server_check: "Pastery 서버가 실행 중인지 확인해주세요. (http://localhost:3030)",
            total_items: "개의 클립보드 항목:",
            memo: "메모",
            copy_failed: "클립보드 복사 실패",
            copied_to_clipboard: "클립보드에 복사됨",
            language: "언어",
        }
    }

    fn english() -> Self {
        Self {
            app_title: "Pastery Pop - Clipboard Manager",
            app_description: "Manage clipboard items from Pastery server.",
            refresh: "Refresh",
            loading: "Loading clipboard items...",
            status: "Status",
            retry: "Retry",
            no_items: "No clipboard items found.",
            server_check: "Please check if Pastery server is running. (http://localhost:3030)",
            total_items: "clipboard items:",
            memo: "Memo",
            copy_failed: "Failed to copy to clipboard",
            copied_to_clipboard: "Copied to clipboard",
            language: "Language",
        }
    }
}
