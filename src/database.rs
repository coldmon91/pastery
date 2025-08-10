
use redb::{Database, TableDefinition, ReadableTable, ReadableDatabase};

/**
 * clipboard data stored in redb
 * format : date-time-sequence -> clipboard_content
 * key format: "YYYY-MM-DD-sequence" (e.g., "2025-08-10-1", "2025-08-10-2")
 */

const CLIPBOARD_TABLE: TableDefinition<&str, &str> = TableDefinition::new("clipboard");

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
        let date_key = now.format("%Y-%m-%d").to_string();
        let sequence = self.get_next_sequence(&date_key);
        let full_key = format!("{}-{}", date_key, sequence);
        
        let write_txn = self.db.begin_write().expect("Failed to begin write transaction");
        {
            let mut table = write_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
            table.insert(full_key.as_str(), text)
                .expect("Failed to insert clipboard data");
        }
        write_txn.commit().expect("Failed to commit transaction");
    }
    
    pub fn read(&self, date_key: &str, sequence: u64) -> Option<String> {
        let read_txn = self.db.begin_read().expect("Failed to begin read transaction");
        let table = read_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
        
        let full_key = format!("{}-{}", date_key, sequence);
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
        let date_prefix = format!("{}-", date_key);
        
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
                // 키 형식: "YYYY-MM-DD-sequence"에서 날짜와 시퀀스 분리
                if let Some(last_dash) = key_str.rfind('-') {
                    let date_part = &key_str[..last_dash];
                    let sequence_part = &key_str[last_dash + 1..];
                    
                    if let Ok(sequence) = sequence_part.parse::<u64>() {
                        all_results.push((date_part.to_string(), sequence, value.value().to_string()));
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
        let date_prefix = format!("{}-", date_key);
        
        for item in table.iter().expect("Failed to iterate table") {
            let (key, _) = item.expect("Failed to read item");
            let key_str = key.value();
            
            // 해당 날짜의 키인지 확인
            if key_str.starts_with(&date_prefix) {
                // 키에서 시퀀스 번호 추출 (예: "2025-08-10-3"에서 "3" 추출)
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
        let date_key = "2025-08-10";
        
        // write 메서드를 직접 호출하는 대신 수동으로 데이터 삽입
        let write_txn = clipboard_data.db.begin_write().expect("Failed to begin write transaction");
        {
            let mut table = write_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
            table.insert("2025-08-10-1", "Hello, World!")
                .expect("Failed to insert clipboard data");
        }
        write_txn.commit().expect("Failed to commit transaction");
        
        // read 함수 테스트
        if let Some(content) = clipboard_data.read(date_key, 1) {
            println!("Read data: {}-1 -> {}", date_key, content);
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
        let result = clipboard_data.read("2024-01-01", 1);
        
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
            ("2025-08-10-1", "First text"),
            ("2025-08-10-2", "Second text"),
            ("2025-08-11-1", "Third text"),
        ];
        
        // 데이터 수동 삽입
        let write_txn = clipboard_data.db.begin_write().expect("Failed to begin write transaction");
        {
            let mut table = write_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
            for (full_key, text) in &test_data {
                table.insert(*full_key, *text)
                    .expect("Failed to insert clipboard data");
            }
        }
        write_txn.commit().expect("Failed to commit transaction");
        
        // 각 데이터를 읽어서 확인
        if let Some(content) = clipboard_data.read("2025-08-10", 1) {
            println!("Read data: 2025-08-10-1 -> {}", content);
            assert_eq!(content, "First text");
        } else {
            panic!("데이터를 읽을 수 없습니다: 2025-08-10-1");
        }
        
        if let Some(content) = clipboard_data.read("2025-08-10", 2) {
            println!("Read data: 2025-08-10-2 -> {}", content);
            assert_eq!(content, "Second text");
        } else {
            panic!("데이터를 읽을 수 없습니다: 2025-08-10-2");
        }
        
        // 날짜별로 모든 데이터 읽기 테스트
        let date_data = clipboard_data.read_by_date("2025-08-10");
        assert_eq!(date_data.len(), 2);
        assert_eq!(date_data[0], (1, "First text".to_string()));
        assert_eq!(date_data[1], (2, "Second text".to_string()));
        
        // 테스트 파일 정리
        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_read_last() {
        // 테스트용 임시 파일 경로
        let test_path = "test_clipboard_read_last.db";
        
        // 기존 테스트 파일이 있다면 삭제
        if std::path::Path::new(test_path).exists() {
            fs::remove_file(test_path).unwrap();
        }
        
        // ClipboardData 인스턴스 생성
        let clipboard_data = ClipboardData::new(test_path.to_string());
        
        // 테스트 데이터 준비 (다른 날짜와 시퀀스로)
        let test_data = vec![
            ("2025-08-08-2", "Text from Aug 8, seq 2"),
            ("2025-08-10-1", "Text from Aug 10, seq 1"),
            ("2025-08-09-1", "Text from Aug 9, seq 1"),
            ("2025-08-10-3", "Text from Aug 10, seq 3"),  // 가장 최신 날짜의 최고 시퀀스
            ("2025-08-10-2", "Text from Aug 10, seq 2"),
        ];
        
        // 데이터 수동 삽입
        let write_txn = clipboard_data.db.begin_write().expect("Failed to begin write transaction");
        {
            let mut table = write_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
            for (full_key, text) in &test_data {
                table.insert(*full_key, *text)
                    .expect("Failed to insert clipboard data");
            }
        }
        write_txn.commit().expect("Failed to commit transaction");
        
        // 최근 3개 데이터 조회
        let recent_data = clipboard_data.read_last(3);
        
        // 결과 검증: 날짜 내림차순, 같은 날짜면 시퀀스 내림차순
        assert_eq!(recent_data.len(), 3);
        
        // 첫 번째는 2025-08-10의 시퀀스 3이어야 함
        assert_eq!(recent_data[0].0, "2025-08-10");
        assert_eq!(recent_data[0].1, 3);
        assert_eq!(recent_data[0].2, "Text from Aug 10, seq 3");
        
        // 두 번째는 2025-08-10의 시퀀스 2여야 함
        assert_eq!(recent_data[1].0, "2025-08-10");
        assert_eq!(recent_data[1].1, 2);
        assert_eq!(recent_data[1].2, "Text from Aug 10, seq 2");
        
        // 세 번째는 2025-08-10의 시퀀스 1이어야 함
        assert_eq!(recent_data[2].0, "2025-08-10");
        assert_eq!(recent_data[2].1, 1);
        assert_eq!(recent_data[2].2, "Text from Aug 10, seq 1");
        
        println!("Recent data test passed:");
        for (date, seq, text) in &recent_data {
            println!("  {}-{} -> {}", date, seq, text);
        }
        
        // 테스트 파일 정리
        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_write_and_sequence() {
        // 테스트용 임시 파일 경로
        let test_path = "test_clipboard_write.db";
        
        // 기존 테스트 파일이 있다면 삭제
        if std::path::Path::new(test_path).exists() {
            fs::remove_file(test_path).unwrap();
        }
        
        // ClipboardData 인스턴스 생성
        let clipboard_data = ClipboardData::new(test_path.to_string());
        
        // 같은 날짜에 여러 데이터 저장
        clipboard_data.write("First clipboard content");
        clipboard_data.write("Second clipboard content");
        clipboard_data.write("Third clipboard content");
        
        // 현재 날짜의 데이터 조회
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let today_data = clipboard_data.read_by_date(&today);
        
        // 3개의 데이터가 순서대로 저장되었는지 확인
        assert_eq!(today_data.len(), 3);
        assert_eq!(today_data[0], (1, "First clipboard content".to_string()));
        assert_eq!(today_data[1], (2, "Second clipboard content".to_string()));
        assert_eq!(today_data[2], (3, "Third clipboard content".to_string()));
        
        println!("Write test passed:");
        for (seq, content) in &today_data {
            println!("  {}-{} -> {}", today, seq, content);
        }
        
        // 테스트 파일 정리
        fs::remove_file(test_path).unwrap();
    }
}