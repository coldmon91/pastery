
use redb::{Database, TableDefinition, ReadableTable, ReadableDatabase};

/**
 * clipboard data stored in redb
 * format : date-time -> (sequence, clipboard_content)
 */

const CLIPBOARD_TABLE: TableDefinition<&str, (u64, &str)> = TableDefinition::new("clipboard");

pub struct ClipboardData {
    db: Database,
}

impl ClipboardData {
    pub fn new(path: String) -> Self {
        let dir = std::path::Path::new(&path).parent();
        if let Some(dir) = dir {
            if !dir.exists() {
                std::fs::create_dir_all(dir).expect(&format!("Failed to create directory {}", dir.display()));
            }
        }
        let db = Database::create(&path).expect(&format!("Failed to create database at {}", &path));
        println!("Database created at {}", &path);
        // Initialize the table
        let write_txn = db.begin_write().expect("Failed to begin write transaction");
        {
            let _ = write_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
        }
        write_txn.commit().expect("Failed to commit transaction");
        
        ClipboardData { db }
    }

    pub fn write(&self, text: &str) {
        let now = chrono::Local::now();
        let sequence = self.get_next_sequence();
        let write_txn = self.db.begin_write().expect("Failed to begin write transaction");
        {
            let mut table = write_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
            table.insert(now.format("%Y-%m-%d").to_string().as_str(), (sequence, text))
                .expect("Failed to insert clipboard data");
        }
        write_txn.commit().expect("Failed to commit transaction");
    }
    
    pub fn read(&self, date_time: &str) -> Option<(u64, String)> {
        let read_txn = self.db.begin_read().expect("Failed to begin read transaction");
        let table = read_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
        
        if let Some(value) = table.get(date_time).expect("Failed to get clipboard data") {
            Some((value.value().0, value.value().1.to_string()))
        } else {
            None
        }
    }


    fn get_next_sequence(&self) -> u64 {
        let read_txn = self.db.begin_read().expect("Failed to begin read transaction");
        let table = read_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
        
        let mut max_sequence = 0u64;
        for item in table.iter().expect("Failed to iterate table") {
            let (_, value) = item.expect("Failed to read item");
            let (sequence, _) = value.value();
            if sequence > max_sequence {
                max_sequence = sequence;
            }
        }
        max_sequence + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_read_existing_data() {
        // 테스트용 임시 파일 경로
        let test_path = "test_clipboard.db";
        
        // 기존 테스트 파일이 있다면 삭제
        if std::path::Path::new(test_path).exists() {
            fs::remove_file(test_path).unwrap();
        }
        
        // ClipboardData 인스턴스 생성
        let clipboard_data = ClipboardData::new(test_path.to_string());
        
        // 시간을 미리 고정
        let now = chrono::Local::now();
        let date_time_key = now.format("%Y-%m-%d").to_string();
        
        // write 메서드를 직접 호출하는 대신 수동으로 데이터 삽입
        let write_txn = clipboard_data.db.begin_write().expect("Failed to begin write transaction");
        {
            let mut table = write_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
            table.insert(date_time_key.as_str(), (1u64, "Hello, World!"))
                .expect("Failed to insert clipboard data");
        }
        write_txn.commit().expect("Failed to commit transaction");
        
        // read 함수 테스트
        if let Some((sequence, content)) = clipboard_data.read(&date_time_key) {
            println!("Read data: {} -> ({}, {})", date_time_key, sequence, content);
            assert_eq!(sequence, 1);
            assert_eq!(content, "Hello, World!");
        } else {
            panic!("데이터를 읽을 수 없습니다");
        }
        
        // 테스트 파일 정리
        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_read_nonexistent_data() {
        // 테스트용 임시 파일 경로
        let test_path = "test_clipboard_empty.db";
        
        // 기존 테스트 파일이 있다면 삭제
        if std::path::Path::new(test_path).exists() {
            fs::remove_file(test_path).unwrap();
        }
        
        // ClipboardData 인스턴스 생성
        let clipboard_data = ClipboardData::new(test_path.to_string());
        
        // 존재하지 않는 키로 읽기 시도
        let result = clipboard_data.read("2024-01-01");
        
        // None이 반환되어야 함
        assert!(result.is_none());
        
        // 테스트 파일 정리
        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_read_multiple_data() {
        // 테스트용 임시 파일 경로
        let test_path = "test_clipboard_multiple.db";
        
        // 기존 테스트 파일이 있다면 삭제
        if std::path::Path::new(test_path).exists() {
            fs::remove_file(test_path).unwrap();
        }
        
        // ClipboardData 인스턴스 생성
        let clipboard_data = ClipboardData::new(test_path.to_string());
        
        // 테스트 데이터와 키 준비
        let test_data = vec![
            ("2025-08-10", 1u64, "First text"),
            ("2025-08-11", 2u64, "Second text"),
            ("2025-08-12", 3u64, "Third text"),
        ];
        
        // 데이터 수동 삽입
        let write_txn = clipboard_data.db.begin_write().expect("Failed to begin write transaction");
        {
            let mut table = write_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
            for (date_key, sequence, text) in &test_data {
                table.insert(*date_key, (*sequence, *text))
                    .expect("Failed to insert clipboard data");
            }
        }
        write_txn.commit().expect("Failed to commit transaction");
        
        // 각 데이터를 읽어서 확인
        for (date_key, expected_sequence, expected_text) in &test_data {
            if let Some((sequence, content)) = clipboard_data.read(date_key) {
                println!("Read data: {} -> ({}, {})", date_key, sequence, content);
                assert_eq!(sequence, *expected_sequence);
                assert_eq!(content, *expected_text);
            } else {
                panic!("데이터를 읽을 수 없습니다: {}", date_key);
            }
        }
        
        // 테스트 파일 정리
        fs::remove_file(test_path).unwrap();
    }
}