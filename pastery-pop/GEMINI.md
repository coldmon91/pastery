# Pastery-pop
- 기본경로: ${workspaceFolder}/pastery-pop
- Tauri + SvelteKit
- This template should help get you started developing with Tauri and SvelteKit in Vite.
- pastery-pop은 pastery 프로젝트의 GUI 애플리케이션이다. 
- pastery는 REST 서버이다. API는 {workspaceFolder}/pastery/API.md 파일을 참고한다.

## 동작 방식

- 프로그램 실행시 기본적으로 백그라운드에서 동작한다. (GUI는 사용자에게 보이지 않는다.)
- 사용자의 키보드 입력을 감지한다. 
- 사용자가 키조합(`ctrl + shift + v`)를 입력하면 마우스 옆에 `PopupGUI`가 나타난다. 키조합은 `settings.json`에서 변경할 수 있다.
- `PopupGUI`에는 최근 5개의 클립보드 항목이 표시된다. 이 클립보드 항목은 pastery의 REST API를 통해 가져온다.
- `PopupGUI`에 포커스가 있는동안 창이 유지되고, `PopupGUI`에 포커스가 벗어나면 즉시 `PopupGUI` Window는 닫힌다.

## PopupGUI에 표시되는 항목

- 최근 클립보드 항목 n개 (기본값: 5, `settings.json "max_items_display"`에서 변경 가능)
- 사용자 메모 n개 (기본값: 5)

## 설정

`settings.json` 파일을 통해 다음 설정을 변경할 수 있습니다:
- `hotkey`: 팝업을 띄우는 키조합 (기본값: "Ctrl+Shift+V")
- `server_url`: pastery 서버 URL (기본값: "http://127.0.0.1:3030")
- `max_items_display`: 표시할 최대 클립보드 항목 수 (기본값: 5)
- `popup_width`: 팝업 창 너비 (기본값: 350)
- `popup_height`: 팝업 창 높이 (기본값: 450)

## 사용법

1. pastery 서버가 실행 중인지 확인
2. pastery-pop 실행
3. 설정된 hotkey (기본: Ctrl+Shift+V)을 눌러 팝업 표시
4. 원하는 클립보드 항목을 클릭하여 클립보드에 복사
5. ESC 키로 팝업 닫기 또는 다른곳을 클릭하여 팝업 닫기

## 빌드 및 실행

```bash
npm install
npm run tauri dev
```
