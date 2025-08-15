# Pastery API 사용 예제

Pastery 서버가 실행된 후 다음 API들을 사용할 수 있습니다.

## 설정 및 제한사항

- **최대 클립보드 항목 수**: `settings.json`의 `max_clipboard_items`로 설정 (기본값: 1000개)
- 최대 개수를 초과하면 오래된 항목부터 자동으로 삭제됩니다
- 삭제 시 해당 항목의 메모도 함께 삭제됩니다

## API 엔드포인트

### 1. 클립보드 항목 조회
```bash
# 모든 클립보드 항목 조회
curl "http://localhost:3030/clipboard"

# 최근 5개 항목만 조회
curl "http://localhost:3030/clipboard?count=5"
```

### 2. 사용자 정의 메모 추가
```bash
curl -X POST "http://localhost:3030/memo" \
  -H "Content-Type: application/json" \
  -d '{"memo": "중요한 메모입니다"}'
```

### 3. 기존 항목에 메모 추가/수정
```bash
curl -X PUT "http://localhost:3030/memo" \
  -H "Content-Type: application/json" \
  -d '{
    "date": "2025-08-15",
    "sequence": 1,
    "memo": "이 클립보드 항목에 대한 메모"
  }'
```

### 4. 메모 삭제
```bash
curl -X DELETE "http://localhost:3030/memo/2025-08-15/1"
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

## 클립보드 항목 데이터 구조

```json
{
  "date": "2025-08-15",
  "sequence": 1,
  "content": "클립보드 내용",
  "memo": "메모 (없으면 null)"
}
```
