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
    let clipboard_data = ClipboardData::new(test_path.to_string(), 1000);
    
    // 시간을 미리 고정
    let date_key = "2025-08-10";
    
    // write 메서드를 직접 호출하는 대신 수동으로 데이터 삽입 (새로운 키 형식 사용)
    let write_txn = clipboard_data.db.begin_write().expect("Failed to begin write transaction");
    {
        let mut table = write_txn.open_table(CLIPBOARD_TABLE).expect("Failed to open table");
        table.insert("clipboard-2025-08-10-1", "Hello, World!")
            .expect("Failed to insert clipboard data");
    }
    write_txn.commit().expect("Failed to commit transaction");
    
    // read 함수 테스트
    if let Some(content) = clipboard_data.read(date_key, 1) {
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
    let clipboard_data = ClipboardData::new(test_path.to_string(), 1000);
    
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
    let clipboard_data = ClipboardData::new(test_path.to_string(), 1000);
    
    // 테스트 데이터와 키 준비 (새로운 키 형식 사용)
    let test_data = vec![
        ("clipboard-2025-08-10-1", "First text"),
        ("clipboard-2025-08-10-2", "Second text"),
        ("clipboard-2025-08-11-1", "Third text"),
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
        assert_eq!(content, "First text");
    } else {
        panic!("데이터를 읽을 수 없습니다: 2025-08-10-1");
    }
    
    if let Some(content) = clipboard_data.read("2025-08-10", 2) {
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
    let clipboard_data = ClipboardData::new(test_path.to_string(), 1000);
    
    // 테스트 데이터 준비 (다른 날짜와 시퀀스로, 새로운 키 형식 사용)
    let test_data = vec![
        ("clipboard-2025-08-08-2", "Text from Aug 8, seq 2"),
        ("clipboard-2025-08-10-1", "Text from Aug 10, seq 1"),
        ("clipboard-2025-08-09-1", "Text from Aug 9, seq 1"),
        ("clipboard-2025-08-10-3", "Text from Aug 10, seq 3"),  // 가장 최신 날짜의 최고 시퀀스
        ("clipboard-2025-08-10-2", "Text from Aug 10, seq 2"),
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
    let clipboard_data = ClipboardData::new(test_path.to_string(), 1000);
    
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
    
    // 테스트 파일 정리
    fs::remove_file(test_path).unwrap();
}

#[test]
fn test_max_items_cleanup() {
    // 테스트용 임시 파일 경로
    let test_path = "test_clipboard_max_items.db";
    
    // 기존 테스트 파일이 있다면 삭제
    if std::path::Path::new(test_path).exists() {
        fs::remove_file(test_path).unwrap();
    }
    
    // 최대 3개 항목으로 제한하여 ClipboardData 인스턴스 생성
    let clipboard_data = ClipboardData::new(test_path.to_string(), 3);
    
    // 5개 항목 추가 (최대 3개를 초과)
    clipboard_data.write("First item");
    clipboard_data.write("Second item");
    clipboard_data.write("Third item");
    clipboard_data.write("Fourth item");  // 이때 첫 번째 항목이 삭제되어야 함
    clipboard_data.write("Fifth item");   // 이때 두 번째 항목이 삭제되어야 함
    
    // 현재 저장된 항목들 확인
    let items = clipboard_data.get_clipboard_items(None);
    
    // 최대 3개만 남아있어야 함
    assert_eq!(items.len(), 3);
    
    // 최신 3개 항목이 남아있는지 확인
    assert_eq!(items[0].content, "Fifth item");
    assert_eq!(items[1].content, "Fourth item");
    assert_eq!(items[2].content, "Third item");
    
    // 테스트 파일 정리
    fs::remove_file(test_path).unwrap();
}

#[test]
fn test_memo_item_functionality() {
    // 테스트용 임시 파일 경로
    let test_path = "test_memo_items.db";
    
    // 기존 테스트 파일이 있다면 삭제
    if std::path::Path::new(test_path).exists() {
        fs::remove_file(test_path).unwrap();
    }
    
    // MemoData 인스턴스 생성
    let memo_data = ClipboardData::new(test_path.to_string(), 1000);
    
    // 메모 추가 테스트 (새로운 API 사용)
    let key1 = memo_data.add_memo("Test memo 1");
    let key2 = memo_data.add_memo("Test memo 2");
    let key3 = memo_data.add_memo("Test memo 3");
    
    // 키에서 시퀀스 추출
    let seq1: u64 = key1.strip_prefix("memo-").unwrap().parse().unwrap();
    let seq2: u64 = key2.strip_prefix("memo-").unwrap().parse().unwrap();
    let seq3: u64 = key3.strip_prefix("memo-").unwrap().parse().unwrap();
    
    // 개별 메모 조회 테스트
    let memo_item = memo_data.get_memo_item(seq1);
    assert!(memo_item.is_some());
    let memo_item = memo_item.unwrap();
    assert_eq!(memo_item.sequence, seq1);
    assert_eq!(memo_item.memo, "Test memo 1");
    
    // 모든 메모 조회 테스트
    let memo_items = memo_data.get_memo_items(None);
    assert_eq!(memo_items.len(), 3);
    
    // 정렬 확인 (시퀀스 내림차순)
    assert_eq!(memo_items[0].sequence, seq3);
    assert_eq!(memo_items[0].memo, "Test memo 3");
    
    assert_eq!(memo_items[1].sequence, seq2);
    assert_eq!(memo_items[1].memo, "Test memo 2");
    
    assert_eq!(memo_items[2].sequence, seq1);
    assert_eq!(memo_items[2].memo, "Test memo 1");
    
    // 개수 제한 테스트
    let limited_memo_items = memo_data.get_memo_items(Some(2));
    assert_eq!(limited_memo_items.len(), 2);
    
    // 테스트 파일 정리
    fs::remove_file(test_path).unwrap();
}
