AI agent는 .md 파일을 수정하지 않는다.   

# Pastery-pop
- Tauri + SvelteKit
- This template should help get you started developing with Tauri and SvelteKit in Vite.
- pastery-pop은 pastery 프로젝트의 GUI 애플리케이션이다. 
- pastery는 REST 서버이다. API는 {workspaceFolder}/pastery/API.md 파일을 참고한다.

# 동작 방식
- 프로그램 실행시 기본적으로 백그라운드에서 동작한다. (GUI는 사용자에게 보이지 않는다.)
- 사용자의 키보드 입력을 감지한다. 
- 사용자가 키조합(`ctrl + v`)를 입력하면 마우스 옆에 GUI가 나타난다. 키조합은 `settings.json`에서 변경할 수 있다.
- GUI에는 최근 5개의 클립보드 항목이 표시된다. 클립보드 항목은 pastery의 REST API를 통해 가져온다.

