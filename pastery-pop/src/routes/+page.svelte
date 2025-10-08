<script>
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from 'svelte';
  import ContextMenu from '../components/ContextMenu.svelte';
  import EditMemoDialog from '../components/EditMemoDialog.svelte';
  import { 
    contextMenuStore, 
    editDialogStore, 
    showContextMenu, 
    hideContextMenu, 
    showEditDialog, 
    hideEditDialog,
    createGlobalClickHandler 
  } from '../utils/contextMenuStore.js';
  import './+page.css';

  let clipboardItems = $state([]);
  let userMemoItems = $state([]);
  let loading = $state(false);
  let error = $state('');
  let showMemoDialog = $state(false);
  let newMemoContent = $state('');
  let currentView = $state('clipboard'); // 'clipboard' or 'memo'

  async function loadAllItems() {
    await Promise.all([loadClipboardItems(), loadUserMemoItems()]);
  }
  async function loadClipboardItems() {
    loading = true;
    error = '';
    try {
      const items = await invoke("get_clipboard_items", { count: 5 });
      clipboardItems = items;
    } catch (err) {
      error = (err || 'Unknown error').toString();
      console.error('Failed to load clipboard items:', err);
    }
    loading = false;
  }

  async function loadUserMemoItems() {
    try {
      const memos = await invoke("get_user_memos", { count: 5 });
      console.log('Loaded user memos:', memos);
      userMemoItems = memos;
    } catch (err) {
      console.error('Failed to load user memos:', err);
    }
  }

  async function selectItem(item) {
    // 클립보드에 내용 복사 (브라우저 API 사용)
    try {
      const textToCopy = item.content ?? item.memo;
      if (textToCopy === undefined) {
        console.error('Item content is undefined:', item);
        return;
      }
      await navigator.clipboard.writeText(textToCopy);
      await hidePopup();
    } catch (err) {
      console.error('Failed to copy to clipboard:', err);
    }
  }

  async function hidePopup() {
    try {
      await invoke("hide_popup");
    } catch (err) {
      console.error('Failed to hide popup:', err);
    }
  }

  function handleEscape(event) {
    if (event.key === 'Escape') {
      hidePopup();
    }
  }

  function handleItemKeydown(event, item) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      selectItem(item);
    }
  }

  function handleBlur() {
    // 포커스가 벗어나면 잠시 후 창을 숨김 (약간의 딜레이를 두어 실수로 닫히는 것을 방지)
    setTimeout(() => {
      hidePopup();
    }, 100);
  }

  function formatDate(dateStr) {
    try {
      const date = new Date(dateStr);
      return date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
    } catch {
      return dateStr;
    }
  }

  function truncateText(text, maxLength = 50) {
    if (text.length <= maxLength) return text;
    return text.substring(0, maxLength) + '...';
  }

  async function showAddMemoDialog() {
    showMemoDialog = true;
    newMemoContent = '';
    // 잠시 후 텍스트 에어리어에 포커스
    setTimeout(() => {
      const textarea = document.querySelector('.memo-textarea');
      if (textarea) textarea.focus();
    }, 100);
  }

  function handleAddMemoKeydown(event) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      showAddMemoDialog();
    }
  }

  async function addMemo() {
    if (!newMemoContent.trim()) return;
    
    try {
      console.log('Adding memo:', newMemoContent.trim());
      await invoke("add_user_memo", { memoContent: newMemoContent.trim() });
      console.log('Memo added successfully');
      showMemoDialog = false;
      newMemoContent = '';
      // 메모 목록 새로고침
      await loadUserMemoItems();
    } catch (err) {
      console.error('Failed to add memo:', err);
      // 더 자세한 에러 정보 표시
      if (typeof err === 'string') {
        error = err;
      } else if (err && typeof err === 'object' && err.message) {
        error = err.message;
      } else {
        error = 'Failed to add memo: ' + JSON.stringify(err);
      }
    }
  }

  function cancelMemoDialog() {
    showMemoDialog = false;
    newMemoContent = '';
  }

  function handleMemoDialogKeydown(event) {
    if (event.key === 'Escape') {
      cancelMemoDialog();
    } else if (event.key === 'Enter' && event.ctrlKey) {
      event.preventDefault();
      addMemo();
    }
  }

  function showClipboardView() {
    currentView = 'clipboard';
  }

  function showMemoView() {
    currentView = 'memo';
  }

  // 컨텍스트 메뉴 이벤트 핸들러들
  function handleContextMenuEvent(event, memo) {
    showContextMenu(event, memo);
  }

  async function handleEditMemo(event) {
    const selectedItem = event.detail;
    console.log('Edit memo selected item:', selectedItem);
    if (selectedItem) {
      // 선택된 아이템을 전역 변수에 저장
      currentEditingItem = selectedItem;
      showEditDialog(selectedItem.memo || '');
      hideContextMenu();
    }
  }

  let currentEditingItem = null;

  async function handleDeleteMemo(event) {
    const selectedItem = event.detail;
    if (!selectedItem) return;
    
    try {
      await invoke("delete_user_memo", { memoId: selectedItem.id });
      hideContextMenu();
      await loadUserMemoItems();
    } catch (err) {
      console.error('Failed to delete memo:', err);
      error = 'Failed to delete memo: ' + (err || 'Unknown error').toString();
    }
  }

  async function handleUpdateMemo(event) {
    const content = event.detail;
    
    if (!content.trim() || !currentEditingItem) return;
    
    console.log('Updating memo:', {
      id: currentEditingItem.id,
      content: content,
      currentEditingItem: currentEditingItem
    });
    
    try {
      await invoke("update_user_memo", { 
        memoId: currentEditingItem.id, 
        memoContent: content 
      });
      
      console.log('Memo update successful, refreshing list...');
      hideEditDialog();
      currentEditingItem = null;
      await loadUserMemoItems();
      console.log('User memo list refreshed');
    } catch (err) {
      console.error('Failed to update memo:', err);
      error = 'Failed to update memo: ' + (err || 'Unknown error').toString();
    }
  }

  function handleCancelEdit() {
    hideEditDialog();
    currentEditingItem = null;
  }

  onMount(() => {
    // loadClipboardItems();
    window.addEventListener('keydown', handleEscape);
    
    const unlistenRefresh = listen('refresh-clipboard', () => {
      loadClipboardItems();
      loadUserMemoItems();
      window.scrollTo(0, 0);
    });

    // 전역 클릭 이벤트로 컨텍스트 메뉴 숨기기
    const globalClickHandler = createGlobalClickHandler($contextMenuStore.visible);
    window.addEventListener('click', globalClickHandler);
    
    return () => {
      window.removeEventListener('keydown', handleEscape);
      window.removeEventListener('click', globalClickHandler);
      unlistenRefresh.then(unlisten => unlisten());
    };
  });
</script>

<svelte:window on:keydown={handleEscape} />

<div 
  class="popup-container" 
  onkeydown={handleEscape}
  onblur={handleBlur}
  tabindex="0"
  role="dialog"
  aria-label="Clipboard items popup"
>
  
  <div class="content">
    {#if loading}
      <div class="loading">Loading...</div>
    {:else if error}
      <div class="error">
        <p>Error: {error}</p>
        <button onclick={loadAllItems}>Retry</button>
      </div>
    {:else if clipboardItems.length === 0 && userMemoItems.length === 0}
      <div class="empty">No clipboard items or memos found</div>
    {:else}
      <div class="content-container">
        <!-- 버튼 행 -->
        <div class="button-row">
          <button 
            class="recent-clipboard-button {currentView === 'clipboard' ? 'active' : ''}" 
            onclick={showClipboardView}
          >📋</button>
          <button 
            class="note-button {currentView === 'memo' ? 'active' : ''}" 
            onclick={showMemoView}
          >🪄</button>
        </div>

        <!-- 클립보드 목록 -->
        {#if currentView === 'clipboard'}
          <div class="clipboard-list">
            {#if clipboardItems.length > 0}
              {#each clipboardItems as item, index}
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div 
                  class="clipboard-item" 
                  role="button"
                  tabindex="0"
                  onclick={() => selectItem(item)}
                  onkeydown={(e) => handleItemKeydown(e, item)}
                >
                  <div class="item-content">
                    <div class="item-text">{truncateText(item.content)}</div>
                    {#if item.memo}
                      <div class="item-memo">{item.memo}</div>
                    {/if}
                  </div>
                </div>
              {/each}
            {:else}
              <div class="empty">No clipboard items found</div>
            {/if}
          </div>
        {/if}

        <!-- 사용자 메모 목록 -->
        {#if currentView === 'memo'}
          <div class="usermemo-list">
            {#if userMemoItems.length > 0}
              {#each userMemoItems as memo, index}
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div 
                  class="clipboard-item memo-item" 
                  role="button"
                  tabindex="0"
                  onclick={() => selectItem(memo)}
                  onkeydown={(e) => handleItemKeydown(e, memo)}
                  oncontextmenu={(e) => handleContextMenuEvent(e, memo)}
                >
                  <div class="item-content">
                    <div class="item-text">{truncateText(memo.memo)}</div>
                  </div>
                </div>
              {/each}
            {:else}
              <div class="empty">No user memos found</div>
            {/if}
          </div>
        {/if}
      </div>
    {/if}
  </div>
  
  <div class="footer">
    <small>Press ESC to close • Click item to copy</small>
  </div>
  
  <!-- UserMemo 추가 버튼 - 메모 뷰일 때만 표시 -->
  {#if currentView === 'memo'}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="add-memo-btn" onclick={showAddMemoDialog} onkeydown={handleAddMemoKeydown} role="button" tabindex="0" title="Add User Memo">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
        <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm5 11h-4v4h-2v-4H7v-2h4V7h2v4h4v2z"/>
      </svg>
    </div>
  {/if}

  <!-- UserMemo 추가 다이얼로그 -->
  {#if showMemoDialog}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="memo-dialog-overlay" onclick={cancelMemoDialog} role="dialog" aria-label="Add memo dialog" tabindex="0">
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="memo-dialog" onclick={(e) => e.stopPropagation()}>
        <h3>Add User Memo</h3>
        <textarea 
          class="memo-textarea" 
          bind:value={newMemoContent} 
          placeholder="Enter your memo content..."
          onkeydown={handleMemoDialogKeydown}
          rows="4"
        ></textarea>
        <div class="memo-dialog-buttons">
          <button onclick={cancelMemoDialog} class="cancel-btn">Cancel</button>
          <button onclick={addMemo} class="add-btn" disabled={!newMemoContent.trim()}>Add Memo</button>
        </div>
        <div class="memo-dialog-hint">
          <small>Press Ctrl+Enter to add • ESC to cancel</small>
        </div>
      </div>
    </div>
  {/if}

  <!-- 컨텍스트 메뉴 컴포넌트 -->
  <ContextMenu 
    visible={$contextMenuStore.visible}
    position={$contextMenuStore.position}
    selectedItem={$contextMenuStore.selectedItem}
    on:edit={handleEditMemo}
    on:delete={handleDeleteMemo}
  />

  <!-- 편집 다이얼로그 컴포넌트 -->
  <EditMemoDialog 
    visible={$editDialogStore.visible}
    memoContent={$editDialogStore.content}
    on:update={handleUpdateMemo}
    on:cancel={handleCancelEdit}
  />
</div>
