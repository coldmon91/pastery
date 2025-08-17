
use redb::{Database, TableDefinition, ReadableTable, ReadableDatabase};
use serde::{Serialize, Deserialize};

/**
 * clipboard data stored in redb
 * format : clipboard-date-time-sequence -> clipboard_content
 * key format: "clipboard-YYYY-MM-DD-sequence" (e.g., "clipboard-2025-08-10-1", "clipboard-2025-08-10-2")
 * 
 * memo data stored in redb
 * format : memo-sequence -> memo_content
 * key format: "memo-sequence" (e.g., "memo-1", "memo-2")
 */

const CLIPBOARD_TABLE: TableDefinition<&str, &str> = TableDefinition::new("clipboard");
const MEMO_TABLE: TableDefinition<&str, &str> = TableDefinition::new("memo");

#[derive(Serialize, Deserialize, Clone)]
pub struct ClipboardItem {
    pub date: String,
    pub sequence: u64,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MemoItem {
    pub date: String,
    pub sequence: u64,
    pub memo: String,
}

pub struct ClipboardData {
    db: Database,
    max_items: usize,
}

impl ClipboardData {
    pub fn new(path: String, max_items: usize) -> Self {
        let dir = std::path::Path::new(&path).parent();
        if let Some(dir) = dir {
            if !dir.exists() {
                std::fs::create_dir_all(dir).expect(&format!("Failed to create directory {}", dir.display()));
            }
        }
        let db = Database::create(&path).expect(&format!("Failed to create database at {}", &path));
        println!("Clipboard database using clip.db at {} with max items: {}", &path, max_items);
        // Initialize the tables (클립보드와 메모 테이블 모두 초기화)
        let write_txn = db.begin_write().expect("Failed to begin write transaction");
        {
            let _ = write_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open clipboard table");
            let _ = write_txn.open_table(MEMO_TABLE).expect("Failed to open memo table");
        }
        write_txn.commit().expect("Failed to commit transaction");
        
        ClipboardData { db, max_items }
    }

    pub fn write(&self, text: &str) {
        let now = chrono::Local::now();
        let date_key = now.format("%Y-%m-%d").to_string();
        let sequence = self.get_next_sequence(&date_key);
        let full_key = format!("clipboard-{}-{}", date_key, sequence);
        
        let write_txn = self.db.begin_write().expect("Failed to begin write transaction");
        {
            let mut table = write_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
            table.insert(full_key.as_str(), text)
                .expect("Failed to insert clipboard data");
        }
        write_txn.commit().expect("Failed to commit transaction");
        
        // 최대 개수 제한 확인 및 정리
        self.cleanup_old_items();
    }
    
    pub fn read(&self, date_key: &str, sequence: u64) -> Option<String> {
        let read_txn = self.db.begin_read().expect("Failed to begin read transaction");
        let table = read_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
        
        let full_key = format!("clipboard-{}-{}", date_key, sequence);
        if let Some(value) = table.get(full_key.as_str()).expect("Failed to get clipboard data") {
            Some(value.value().to_string())
        } else {
            None
        }
    }
    
    pub fn read_by_date(&self, date_key: &str) -> Vec<(u64, String)> {
        let read_txn = self.db.begin_read().expect("Failed to begin read transaction");
        let table = read_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
        
        let mut results = Vec::new();
        let date_prefix = format!("clipboard-{}-", date_key);
        
        for item in table.iter().expect("Failed to iterate table") {
            if let Ok((key, value)) = item {
                let key_str = key.value();
                if key_str.starts_with(&date_prefix) {
                    // 키에서 시퀀스 번호 추출
                    if let Some(sequence_str) = key_str.strip_prefix(&date_prefix) {
                        if let Ok(sequence) = sequence_str.parse::<u64>() {
                            results.push((sequence, value.value().to_string()));
                        }
                    }
                }
            }
        }
        
        // 시퀀스 번호로 정렬
        results.sort_by(|a, b| a.0.cmp(&b.0));
        results
    }

    pub fn read_last(&self, count: usize) -> Vec<(String, u64, String)> {
        let read_txn = self.db.begin_read().expect("Failed to begin read transaction");
        let table = read_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
        
        let mut all_results = Vec::new();
        
        // 모든 데이터를 수집하고 키에서 날짜와 시퀀스를 파싱
        for item in table.iter().expect("Failed to iterate table") {
            if let Ok((key, value)) = item {
                let key_str = key.value();
                // 키 형식: "clipboard-YYYY-MM-DD-sequence"에서 날짜와 시퀀스 분리
                if let Some(clipboard_prefix) = key_str.strip_prefix("clipboard-") {
                    if let Some(last_dash) = clipboard_prefix.rfind('-') {
                        let date_part = &clipboard_prefix[..last_dash];
                        let sequence_part = &clipboard_prefix[last_dash + 1..];
                        
                        if let Ok(sequence) = sequence_part.parse::<u64>() {
                            all_results.push((date_part.to_string(), sequence, value.value().to_string()));
                        }
                    }
                }
            }
        }
        
        // 날짜를 기준으로 내림차순, 같은 날짜면 시퀀스를 기준으로 내림차순 정렬
        all_results.sort_by(|a, b| {
            match b.0.cmp(&a.0) { // 날짜 내림차순
                std::cmp::Ordering::Equal => b.1.cmp(&a.1), // 시퀀스 내림차순
                other => other,
            }
        });
        
        // 요청된 개수만큼 반환
        all_results.into_iter().take(count).collect()
    }


    fn get_next_sequence(&self, date_key: &str) -> u64 {
        let read_txn = self.db.begin_read().expect("Failed to begin read transaction");
        let table = read_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
        
        let mut max_sequence = 0u64;
        let date_prefix = format!("clipboard-{}-", date_key);
        
        for item in table.iter().expect("Failed to iterate table") {
            let (key, _) = item.expect("Failed to read item");
            let key_str = key.value();
            
            // 해당 날짜의 키인지 확인
            if key_str.starts_with(&date_prefix) {
                // 키에서 시퀀스 번호 추출 (예: "clipboard-2025-08-10-3"에서 "3" 추출)
                if let Some(sequence_str) = key_str.strip_prefix(&date_prefix) {
                    if let Ok(sequence) = sequence_str.parse::<u64>() {
                        if sequence > max_sequence {
                            max_sequence = sequence;
                        }
                    }
                }
            }
        }
        max_sequence + 1
    }

    pub fn get_clipboard_items(&self, count: Option<usize>) -> Vec<ClipboardItem> {
        let read_txn = self.db.begin_read().expect("Failed to begin read transaction");
        let clipboard_table = read_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open clipboard table");
        
        let mut all_results = Vec::new();
        
        // 모든 클립보드 데이터를 수집
        for item in clipboard_table.iter().expect("Failed to iterate clipboard table") {
            if let Ok((key, value)) = item {
                let key_str = key.value();
                // 키 형식: "clipboard-YYYY-MM-DD-sequence"에서 날짜와 시퀀스 분리
                if let Some(clipboard_prefix) = key_str.strip_prefix("clipboard-") {
                    if let Some(last_dash) = clipboard_prefix.rfind('-') {
                        let date_part = &clipboard_prefix[..last_dash];
                        let sequence_part = &clipboard_prefix[last_dash + 1..];
                        
                        if let Ok(sequence) = sequence_part.parse::<u64>() {
                            all_results.push(ClipboardItem {
                                date: date_part.to_string(),
                                sequence,
                                content: value.value().to_string(),
                            });
                        }
                    }
                }
            }
        }
        
        // 날짜를 기준으로 내림차순, 같은 날짜면 시퀀스를 기준으로 내림차순 정렬
        all_results.sort_by(|a, b| {
            match b.date.cmp(&a.date) {
                std::cmp::Ordering::Equal => b.sequence.cmp(&a.sequence),
                other => other,
            }
        });
        
        // 요청된 개수만큼 반환
        if let Some(count) = count {
            all_results.into_iter().take(count).collect()
        } else {
            all_results
        }
    }

    // 오래된 항목들을 정리하여 최대 개수를 유지
    fn cleanup_old_items(&self) {
        let read_txn = self.db.begin_read().expect("Failed to begin read transaction");
        let clipboard_table = read_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open clipboard table");
        
        let mut all_items = Vec::new();
        
        // 모든 클립보드 데이터를 수집
        for item in clipboard_table.iter().expect("Failed to iterate clipboard table") {
            if let Ok((key, _)) = item {
                let key_str = key.value();
                // 키 형식: "clipboard-YYYY-MM-DD-sequence"에서 날짜와 시퀀스 분리
                if let Some(clipboard_prefix) = key_str.strip_prefix("clipboard-") {
                    if let Some(last_dash) = clipboard_prefix.rfind('-') {
                        let date_part = &clipboard_prefix[..last_dash];
                        let sequence_part = &clipboard_prefix[last_dash + 1..];
                        
                        if let Ok(sequence) = sequence_part.parse::<u64>() {
                            all_items.push((date_part.to_string(), sequence, key_str.to_string()));
                        }
                    }
                }
            }
        }
        drop(read_txn); // 읽기 트랜잭션 종료
        
        // 항목 수가 최대치를 초과하는 경우에만 정리
        if all_items.len() > self.max_items {
            // 날짜를 기준으로 내림차순, 같은 날짜면 시퀀스를 기준으로 내림차순 정렬
            all_items.sort_by(|a, b| {
                match b.0.cmp(&a.0) {
                    std::cmp::Ordering::Equal => b.1.cmp(&a.1),
                    other => other,
                }
            });
            
            // 최대치를 초과하는 오래된 항목들 삭제
            let items_to_delete = &all_items[self.max_items..];
            
            let write_txn = self.db.begin_write().expect("Failed to begin write transaction");
            {
                let mut clipboard_table = write_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open clipboard table");
                
                for (_, _, key) in items_to_delete {
                    clipboard_table.remove(key.as_str()).ok(); // 에러 무시
                }
            }
            write_txn.commit().expect("Failed to commit transaction");
            
            println!("Cleaned up {} old clipboard items. Current count: {}", 
                     items_to_delete.len(), self.max_items);
        }
    }

    // 메모 관련 메서드들
    pub fn add_memo(&self, memo: &str) -> String {
        let sequence = self.get_next_memo_sequence();
        let full_key = format!("memo-{}", sequence);
        let write_txn = self.db.begin_write().expect("Failed to begin write transaction");
        {
            let mut table = write_txn.open_table(MEMO_TABLE).expect("Failed to open memo table");
            table.insert(full_key.as_str(), memo).expect("Failed to insert memo");
        }
        write_txn.commit().expect("Failed to commit transaction");
        full_key
    }

    pub fn get_memo(&self, sequence: u64) -> Option<String> {
        let full_key = format!("memo-{}", sequence);
        let read_txn = self.db.begin_read().expect("Failed to begin read transaction");
        let table = read_txn.open_table(MEMO_TABLE).expect("Failed to open memo table");
        
        if let Some(value) = table.get(full_key.as_str()).expect("Failed to get memo") {
            Some(value.value().to_string())
        } else {
            None
        }
    }

    pub fn update_memo(&self, sequence: u64, memo: &str) {
        let full_key = format!("memo-{}", sequence);
        let write_txn = self.db.begin_write().expect("Failed to begin write transaction");
        {
            let mut table = write_txn.open_table(MEMO_TABLE).expect("Failed to open memo table");
            table.insert(full_key.as_str(), memo).expect("Failed to insert memo");
        }
        write_txn.commit().expect("Failed to commit transaction");
    }

    pub fn delete_memo(&self, sequence: u64) {
        let full_key = format!("memo-{}", sequence);
        let write_txn = self.db.begin_write().expect("Failed to begin write transaction");
        {
            let mut table = write_txn.open_table(MEMO_TABLE).expect("Failed to open memo table");
            table.remove(full_key.as_str()).expect("Failed to remove memo");
        }
        write_txn.commit().expect("Failed to commit transaction");
    }

    pub fn get_memo_items(&self, count: Option<usize>) -> Vec<MemoItem> {
        let read_txn = self.db.begin_read().expect("Failed to begin read transaction");
        let memo_table = read_txn.open_table(MEMO_TABLE).expect("Failed to open memo table");
        
        let mut all_results = Vec::new();
        
        // 모든 메모 데이터를 수집
        for item in memo_table.iter().expect("Failed to iterate memo table") {
            if let Ok((key, value)) = item {
                let key_str = key.value();
                // memo-sequence 형식에서 sequence 추출
                if let Some(sequence_str) = key_str.strip_prefix("memo-") {
                    if let Ok(sequence) = sequence_str.parse::<u64>() {
                        all_results.push(MemoItem {
                            date: String::new(), // memo는 더 이상 날짜를 사용하지 않음
                            sequence,
                            memo: value.value().to_string(),
                        });
                    }
                }
            }
        }
        
        // 시퀀스를 기준으로 내림차순 정렬
        all_results.sort_by(|a, b| b.sequence.cmp(&a.sequence));
        
        // 요청된 개수만큼 반환
        if let Some(count) = count {
            all_results.into_iter().take(count).collect()
        } else {
            all_results
        }
    }

    pub fn get_memo_item(&self, sequence: u64) -> Option<MemoItem> {
        let full_key = format!("memo-{}", sequence);
        let read_txn = self.db.begin_read().expect("Failed to begin read transaction");
        let table = read_txn.open_table(MEMO_TABLE).expect("Failed to open memo table");
        
        if let Some(value) = table.get(full_key.as_str()).expect("Failed to get memo") {
            Some(MemoItem {
                date: String::new(), // memo는 더 이상 날짜를 사용하지 않음
                sequence,
                memo: value.value().to_string(),
            })
        } else {
            None
        }
    }

    fn get_next_memo_sequence(&self) -> u64 {
        let read_txn = self.db.begin_read().expect("Failed to begin read transaction");
        let table = read_txn.open_table(MEMO_TABLE).expect("Failed to open memo table");
        
        let mut max_sequence = 0u64;
        
        for item in table.iter().expect("Failed to iterate table") {
            let (key, _) = item.expect("Failed to read item");
            let key_str = key.value();
            
            // memo-sequence 형식에서 sequence 추출
            if let Some(sequence_str) = key_str.strip_prefix("memo-") {
                if let Ok(sequence) = sequence_str.parse::<u64>() {
                    if sequence > max_sequence {
                        max_sequence = sequence;
                    }
                }
            }
        }
        max_sequence + 1
    }
}

#[cfg(test)]
mod tests;