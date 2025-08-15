use iced::widget::{button, column, container, scrollable, text};
use iced::{Application, Command, Element, Font, Length, Settings, Theme};
use serde::{Deserialize, Serialize};

// 한글 폰트 정의
const KOREAN_FONT: Font = Font::with_name("Malgun Gothic");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItem {
    pub date: String,
    pub sequence: u32,
    pub content: String,
    pub memo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<Vec<ClipboardItem>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadClipboard,
    ClipboardLoaded(Result<Vec<ClipboardItem>, String>),
    SelectItem(ClipboardItem),
    Refresh,
}

struct PasteryPop {
    clipboard_items: Vec<ClipboardItem>,
    loading: bool,
    error_message: Option<String>,
}

impl Default for PasteryPop {
    fn default() -> Self {
        Self {
            clipboard_items: Vec::new(),
            loading: false,
            error_message: None,
        }
    }
}

impl Application for PasteryPop {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::perform(load_clipboard_blocking(), Message::ClipboardLoaded))
    }

    fn title(&self) -> String {
        String::from("Pastery Pop - 클립보드 관리자")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LoadClipboard | Message::Refresh => {
                self.loading = true;
                self.error_message = None;
                Command::perform(load_clipboard_blocking(), Message::ClipboardLoaded)
            }
            Message::ClipboardLoaded(result) => {
                self.loading = false;
                match result {
                    Ok(items) => {
                        self.clipboard_items = items;
                        self.error_message = None;
                    }
                    Err(error) => {
                        self.error_message = Some(error);
                    }
                }
                Command::none()
            }
            Message::SelectItem(item) => {
                // 선택된 항목을 클립보드에 복사
                if let Err(e) = copy_to_clipboard(&item.content) {
                    self.error_message = Some(format!("클립보드 복사 실패: {}", e));
                } else {
                    let preview = truncate_string(&item.content, 30);
                    self.error_message = Some(format!("클립보드에 복사됨: {}", preview));
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let header = column![
            text("Pastery Pop - 클립보드 관리자").size(24).font(KOREAN_FONT),
            text("Pastery 서버의 클립보드 항목들을 관리할 수 있습니다.").size(14).font(KOREAN_FONT),
            button(text("새로고침").font(KOREAN_FONT)).on_press(Message::Refresh)
        ]
        .spacing(10);

        let content = if self.loading {
            column![text("클립보드 항목을 불러오는 중...").size(16).font(KOREAN_FONT)]
        } else if let Some(error) = &self.error_message {
            column![
                text(format!("상태: {}", error)).size(14).font(KOREAN_FONT),
                button(text("다시 시도").font(KOREAN_FONT)).on_press(Message::Refresh)
            ]
            .spacing(10)
        } else if self.clipboard_items.is_empty() {
            column![
                text("클립보드 항목이 없습니다.").size(16).font(KOREAN_FONT),
                text("Pastery 서버가 실행 중인지 확인해주세요. (http://localhost:3030)").size(12).font(KOREAN_FONT),
                button(text("새로고침").font(KOREAN_FONT)).on_press(Message::Refresh)
            ]
            .spacing(10)
        } else {
            let mut items_column = column![
                text(format!("총 {} 개의 클립보드 항목:", self.clipboard_items.len())).size(16).font(KOREAN_FONT)
            ].spacing(10);

            for (index, item) in self.clipboard_items.iter().enumerate() {
                let preview = truncate_string(&item.content, 80);

                let item_text = if let Some(memo) = &item.memo {
                    format!("{}. [{}] {} (메모: {})", index + 1, item.date, preview, memo)
                } else {
                    format!("{}. [{}] {}", index + 1, item.date, preview)
                };

                items_column = items_column.push(
                    button(text(item_text).font(KOREAN_FONT))
                        .width(Length::Fill)
                        .on_press(Message::SelectItem(item.clone())),
                );
            }

            items_column
        };

        let main_content = column![header, content].spacing(20);

        container(
            scrollable(main_content)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .into()
    }
}

// UTF-8 문자 경계를 고려하여 안전하게 문자열을 자르는 함수
fn truncate_string(s: &str, max_chars: usize) -> String {
    let char_count = s.chars().count();
    if char_count > max_chars {
        format!("{}...", s.chars().take(max_chars).collect::<String>())
    } else {
        s.to_string()
    }
}

// blocking을 사용해서 tokio 런타임 문제 방지
async fn load_clipboard_blocking() -> Result<Vec<ClipboardItem>, String> {
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| format!("런타임 생성 실패: {}", e))?;
    
    rt.block_on(async {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .map_err(|e| format!("HTTP 클라이언트 생성 실패: {}", e))?;
            
        let response = client
            .get("http://localhost:3030/clipboard?count=20")
            .send()
            .await
            .map_err(|e| format!("네트워크 오류: {} (Pastery 서버가 실행 중인지 확인하세요)", e))?;

        let api_response: ApiResponse = response
            .json()
            .await
            .map_err(|e| format!("JSON 파싱 오류: {}", e))?;

        if api_response.success {
            Ok(api_response.data.unwrap_or_default())
        } else {
            Err(api_response.message)
        }
    })
}

fn copy_to_clipboard(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    use clipboard::{ClipboardContext, ClipboardProvider};
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(content.to_owned())?;
    Ok(())
}

fn main() -> iced::Result {
    let settings = Settings {
        ..Settings::default()
    };

    PasteryPop::run(settings)
}