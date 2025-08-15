use std::sync::{Arc, Mutex};
use warp::Filter;
use serde::{Deserialize, Serialize};
use crate::database::{ClipboardData, ClipboardItem};

#[derive(Deserialize)]
struct MemoRequest {
    memo: String,
}

#[derive(Deserialize)]
struct UpdateMemoRequest {
    date: String,
    sequence: u64,
    memo: String,
}

#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    message: String,
    data: Option<serde_json::Value>,
}

impl ApiResponse {
    fn success(message: &str, data: Option<serde_json::Value>) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data,
        }
    }

    fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}

pub async fn start_server(clipboard_data: Arc<Mutex<ClipboardData>>, port: u16) {
    // GET /clipboard - 클립보드 항목들 조회
    let clipboard_data_filter = warp::any().map(move || clipboard_data.clone());
    
    let get_clipboard = warp::path("clipboard")
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(clipboard_data_filter.clone())
        .and_then(handle_get_clipboard);

    // POST /memo - 사용자 정의 메모 추가
    let add_memo = warp::path("memo")
        .and(warp::post())
        .and(warp::body::json())
        .and(clipboard_data_filter.clone())
        .and_then(handle_add_memo);

    // PUT /memo - 기존 항목에 메모 추가/수정
    let update_memo = warp::path("memo")
        .and(warp::put())
        .and(warp::body::json())
        .and(clipboard_data_filter.clone())
        .and_then(handle_update_memo);

    // DELETE /memo/{date}/{sequence} - 메모 삭제
    let delete_memo = warp::path("memo")
        .and(warp::path::param::<String>())
        .and(warp::path::param::<u64>())
        .and(warp::delete())
        .and(clipboard_data_filter.clone())
        .and_then(handle_delete_memo);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);

    let routes = get_clipboard
        .or(add_memo)
        .or(update_memo)
        .or(delete_memo)
        .with(cors);

    println!("Starting server on port {}", port);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

async fn handle_get_clipboard(
    query: std::collections::HashMap<String, String>,
    clipboard_data: Arc<Mutex<ClipboardData>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let count = query.get("count")
        .and_then(|c| c.parse::<usize>().ok());

    let clipboard_data = clipboard_data.lock().unwrap();
    let items = clipboard_data.get_clipboard_items(count);
    
    let response = ApiResponse::success(
        "Clipboard items retrieved successfully",
        Some(serde_json::to_value(&items).unwrap()),
    );
    
    Ok(warp::reply::json(&response))
}

async fn handle_add_memo(
    request: MemoRequest,
    clipboard_data: Arc<Mutex<ClipboardData>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let clipboard_data = clipboard_data.lock().unwrap();
    let key = clipboard_data.add_custom_memo(&request.memo);
    
    let response = ApiResponse::success(
        "Custom memo added successfully",
        Some(serde_json::json!({"key": key})),
    );
    
    Ok(warp::reply::json(&response))
}

async fn handle_update_memo(
    request: UpdateMemoRequest,
    clipboard_data: Arc<Mutex<ClipboardData>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let clipboard_data = clipboard_data.lock().unwrap();
    clipboard_data.update_memo(&request.date, request.sequence, &request.memo);
    
    let response = ApiResponse::success("Memo updated successfully", None);
    Ok(warp::reply::json(&response))
}

async fn handle_delete_memo(
    date: String,
    sequence: u64,
    clipboard_data: Arc<Mutex<ClipboardData>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let clipboard_data = clipboard_data.lock().unwrap();
    clipboard_data.delete_memo(&date, sequence);
    
    let response = ApiResponse::success("Memo deleted successfully", None);
    Ok(warp::reply::json(&response))
}
