# Pastery API 사용 예제

Pastery 서버가 실행된 후 다음 API들을 사용할 수 있습니다.

## 설정 및 제한사항

- **최대 클립보드 항목 수**: `settings.json`의 `max_clipboard_items`로 설정 (기본값: 1000개)
- 최대 개수를 초과하면 가장 오래된 클립보드 항목부터 자동으로 삭제됩니다.
- **참고**: 클립보드 항목이 삭제되어도 관련 메모는 삭제되지 않습니다.

## API 엔드포인트

### 클립보드 API

**1. 클립보드 항목 조회**
```bash
# 모든 클립보드 항목 조회 (최신순)
curl "http://localhost:3030/clipboard"

# 최근 5개 항목만 조회
curl "http://localhost:3030/clipboard?count=5"
```

### 메모 API

**1. 메모 조회**
```bash
# 모든 메모 조회 (최신순)
curl "http://localhost:3030/memo"

# 최근 5개 메모만 조회
curl "http://localhost:3030/memo?count=5"
```

**2. 메모 추가**
- 새로운 메모를 생성합니다. 생성된 메모는 고유한 `sequence` 번호를 가집니다.
```bash
curl -X POST "http://localhost:3030/memo" \
  -H "Content-Type: application/json" \
  -d '{"memo": "새로운 독립적인 메모입니다"}'
```

**3. 메모 수정**
- 지정된 `sequence` 번호의 메모 내용을 수정합니다.
```bash
curl -X PUT "http://localhost:3030/memo/1" \
  -H "Content-Type: application/json" \
  -d '{"memo": "수정된 메모 내용입니다"}'
```

**4. 메모 삭제**
- 지정된 `sequence` 번호의 메모를 삭제합니다.
```bash
curl -X DELETE "http://localhost:3030/memo/1"
```

## 응답 형식

모든 API는 다음과 같은 형식으로 응답합니다:

```json
{
  "success": true,
  "message": "성공 메시지",
  "data": {
    // 실제 데이터 (있는 경우)
  }
}
```

## 데이터 구조

**클립보드 항목 (`ClipboardItem`)**
```json
{
  "date": "2025-08-15",
  "sequence": 1,
  "content": "클립보드 내용"
}
```

**메모 항목 (`MemoItem`)**
```json
{
  "sequence": 1,
  "memo": "메모 내용"
}
```