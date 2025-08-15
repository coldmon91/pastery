
## Pastery-Pop (Pastery server's GUI program)
- Pastery 백그라운드 서버의 GUI 프로그램 입니다. 
- iced(https://github.com/iced-rs/iced) 프레임워크를 사용 합니다.

## 기본기능
#### 붙여넣기 단축키 감지
- 사용자의 붙여넣기 단축키(Ctrl+V) 입력을 감지합니다. 
- 일반적으로 GUI윈도우는 보이지 않지만, 붙여넣기 입력이 감지되면, Pastery 서버로 부터 클립보드 목록을 불러와서 Popup창에 표시합니다.

# Pastery API 사용 예제
Pastery 서버가 실행되어 있으면 다음 API들을 사용할 수 있습니다.
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
