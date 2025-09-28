import { writable } from 'svelte/store';

// 컨텍스트 메뉴 상태 관리
export const contextMenuStore = writable({
  visible: false,
  position: { x: 0, y: 0 },
  selectedItem: null
});

// 편집 다이얼로그 상태 관리
export const editDialogStore = writable({
  visible: false,
  content: ''
});

// 컨텍스트 메뉴 표시
export function showContextMenu(event, item) {
  event.preventDefault();
  event.stopPropagation();
  
  contextMenuStore.set({
    visible: true,
    position: { x: event.clientX, y: event.clientY },
    selectedItem: item
  });
}

// 컨텍스트 메뉴 숨기기
export function hideContextMenu() {
  contextMenuStore.set({
    visible: false,
    position: { x: 0, y: 0 },
    selectedItem: null
  });
}

// 편집 다이얼로그 표시
export function showEditDialog(content) {
  editDialogStore.set({
    visible: true,
    content: content || ''
  });
}

// 편집 다이얼로그 숨기기
export function hideEditDialog() {
  editDialogStore.set({
    visible: false,
    content: ''
  });
}

// 전역 클릭 이벤트 핸들러
export function createGlobalClickHandler(contextMenuVisible) {
  return function handleGlobalClick(event) {
    if (contextMenuVisible) {
      hideContextMenu();
    }
  };
}