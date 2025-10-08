- 이 프로그램의 Pastery의 백그라운드 서버 프로그램입니다.
- GUI 프로그램은 별도의 프로세스로 실행됩니다.

### 기본 사항
- paste 동작 시 사용자가 원하는 클립보드 항목을 선택해서 paste 할 수 있습니다.
- database는 redb를 사용
- 단축키는 사용자 지정에 따라 변경할 수 있습니다.
  - 기본 복사(copy) 단축키는 ctrl+c
  - settings.json 파일을 제공하여 사용자가 단축키를 변경할 수 있습니다.
- pastery 서버는 사용자에게 보이지 않게 백그라운드에서 실행됩니다.
- pastery는 사용자의 키보드 입력을 감지하고 있다가, 사용자가 copy 단축키를 누르면 작업을 수행합니다.

## 기본 기능
### copy
- 사용자가 copy 단축키를 누르면 클립보드의 내용을 database에 저장합니다.
- database는 key-value database입니다. key format: "YYYY-MM-DD-sequence" (e.g "2025-08-10-1", "2025-08-10-2")

### serve
- 클립보드를 database 에서 조회하고 반환하는 API를 제공합니다.
- 사용자가 임의로 작성한 메모를 database에 추가/삭제/변경 하는 API를 제공합니다.

## 부가 기능
### memo 기능
- 사용자는 클립보드 항목에 메모를 추가할 수 있습니다.
- 사용자가 임의로 작성한 메모를 클립보드 항목에 추가할 수 있습니다.
- 팝업에 사용자가 임의로 작성한 메모를 표시하고 사용자는 붙여넣기 할 데이터를 선택할 수 있습니다.

## 구현된 기능

### 1. 키보드 이벤트 감지
- 사용자 정의 가능한 단축키 (settings.json)
- 기본 설정: Ctrl+C (복사), Ctrl+V (paste 메뉴 표시)

### 2. 데이터베이스 (redb)
- 클립보드 내용 저장
- 메모 기능 지원
- 날짜별 시퀀스 관리

### 3. REST API 서버
- **GET /clipboard**: 클립보드 항목 조회 (count 파라미터로 개수 제한 가능)
- **POST /memo**: 사용자 정의 메모 추가
- **PUT /memo**: 기존 항목에 메모 추가/수정
- **DELETE /memo/{date}/{sequence}**: 메모 삭제

### 4. 설정 파일 (settings.json)
```json
{
  "copy_key": {
    "ctrl": true,
    "alt": false,
    "shift": false,
    "key": "c"
  },
  "paste_key": {
    "ctrl": true,
    "alt": false,
    "shift": false,
    "key": "v"
  },
  "server_port": 3030,
  "max_clipboard_items": 1000
}
```

- **max_clipboard_items**: 데이터베이스에 저장할 클립보드 항목의 최대 개수
  - 이 개수를 초과하면 오래된 항목부터 자동으로 삭제됩니다
  - 기본값: 1000개

## 사용 방법

1. **프로그램 실행**
   ```bash
   cargo run
   ```

2. **API 테스트**
   - 서버는 기본적으로 포트 3030에서 실행됩니다
   - API_EXAMPLES.md 파일을 참고하여 curl 명령어로 테스트 가능

3. **설정 변경**
   - settings.json 파일을 수정하여 단축키와 서버 포트 변경 가능
   - 프로그램 재시작 후 적용

