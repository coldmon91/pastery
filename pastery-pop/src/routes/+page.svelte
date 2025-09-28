<script>
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from 'svelte';

  let clipboardItems = $state([]);
  let userMemoItems = $state([]);
  let loading = $state(false);
  let error = $state('');
  let showMemoDialog = $state(false);
  let newMemoContent = $state('');

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

  onMount(() => {
    // loadClipboardItems();
    window.addEventListener('keydown', handleEscape);
    
    const unlistenRefresh = listen('refresh-clipboard', () => {
      loadClipboardItems();
      loadUserMemoItems();
      window.scrollTo(0, 0);
    });
    
    return () => {
      window.removeEventListener('keydown', handleEscape);
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
      <div class="clipboard-list">
        {#if clipboardItems.length > 0}
          <div class="section-title">Recent Clipboard</div>
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
        {/if}

        <div class="usermemo-list">
          {#if userMemoItems.length > 0}
            <div class="section-title">User Memos</div>
            {#each userMemoItems as memo, index}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div 
                class="clipboard-item memo-item" 
                role="button"
                tabindex="0"
                onclick={() => selectItem(memo)}
                onkeydown={(e) => handleItemKeydown(e, memo)}
              >
                <div class="item-content">
                  <div class="item-text">{truncateText(memo.memo)}</div>
                </div>
              </div>
            {/each}
          {/if}
        </div>
      </div>
    {/if}
  </div>
  
  <div class="footer">
    <small>Press ESC to close • Click item to copy</small>
  </div>
  
  <!-- UserMemo 추가 버튼 -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="add-memo-btn" onclick={showAddMemoDialog} onkeydown={handleAddMemoKeydown} role="button" tabindex="0" title="Add User Memo">
    <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
      <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm5 11h-4v4h-2v-4H7v-2h4V7h2v4h4v2z"/>
    </svg>
  </div>

  <!-- UserMemo 추가 다이얼로그 -->
  {#if showMemoDialog}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="memo-dialog-overlay" onclick={cancelMemoDialog} role="dialog" aria-label="Add memo dialog">
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
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    background: transparent;
    -ms-overflow-style: none; /* IE and Edge */
    scrollbar-width: none; /* Firefox */
  }
  :global(*)::-webkit-scrollbar {
    display: none;
  }

  .popup-container {
    width: 232px;
    height: 320px;
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(10px);
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.3);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    position: relative;
    -ms-overflow-style: none; /* IE and Edge */
    scrollbar-width: none; /* Firefox */
  }

  .popup-container::-webkit-scrollbar {
    display: none;
  }

  .content {
    flex: 1;
    padding: 8px;
    overflow-y: auto;
    -ms-overflow-style: none; /* IE and Edge */
    scrollbar-width: none; /* Firefox */
  }

  .content::-webkit-scrollbar {
    display: none;
  }

  .loading, .error, .empty {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    text-align: center;
    color: #666;
  }

  .error {
    flex-direction: column;
    gap: 12px;
  }

  .error button {
    padding: 6px 13px;
    background: #007acc;
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 12px;
  }

  .error button:hover {
    background: #005a9e;
  }

  .clipboard-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .section-title {
    font-size: 12px;
    font-weight: 600;
    color: #666;
    margin: 8px 0 4px 0;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .clipboard-item {
    display: flex;
    align-items: flex-start;
    padding: 12px;
    background: rgba(255, 255, 255, 0.7);
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
    gap: 12px;
  }

  .clipboard-item:hover {
    background: rgba(0, 120, 212, 0.1);
    border-color: rgba(0, 120, 212, 0.3);
    transform: translateY(-1px);
  }

  .memo-item {
    border-left: 3px solid #007acc;
  }

  .item-content {
    flex: 1;
    min-width: 0;
  }

  .item-text {
    font-size: 14px;
    line-height: 1.4;
    color: #333;
    word-wrap: break-word;
    margin-bottom: 4px;
  }

  .item-memo {
    font-size: 12px;
    color: #666;
    font-style: italic;
    margin-bottom: 4px;
  }

  .footer {
    padding: 8px 16px;
    background: rgba(0, 0, 0, 0.05);
    border-top: 1px solid rgba(0, 0, 0, 0.1);
    text-align: center;
  }

  .footer small {
    color: #666;
    font-size: 11px;
  }

  /* UserMemo 추가 버튼 스타일 */
  .add-memo-btn {
    position: absolute;
    bottom: 16px;
    right: 16px;
    width: 48px;
    height: 48px;
    background: #007acc;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s ease;
    box-shadow: 0 4px 12px rgba(0, 122, 204, 0.3);
    color: white;
  }

  .add-memo-btn:hover {
    background: #005a9e;
    transform: scale(1.1);
    box-shadow: 0 6px 16px rgba(0, 122, 204, 0.4);
  }

  .add-memo-btn:active {
    transform: scale(0.95);
  }

  /* UserMemo 추가 다이얼로그 스타일 */
  .memo-dialog-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .memo-dialog {
    background: white;
    border-radius: 12px;
    padding: 24px;
    width: 150px;
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.3);
    animation: memo-dialog-in 0.2s ease-out;
  }

  @keyframes memo-dialog-in {
    from {
      opacity: 0;
      transform: scale(0.9) translateY(-20px);
    }
    to {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
  }

  .memo-dialog h3 {
    margin: 0 0 16px 0;
    font-size: 16px;
    font-weight: 600;
    color: #333;
  }

  .memo-textarea {
    width: 100%;
    border: 1px solid #ddd;
    border-radius: 6px;
    padding: 12px;
    font-size: 14px;
    font-family: inherit;
    resize: vertical;
    min-height: 80px;
    box-sizing: border-box;
    margin-bottom: 16px;
  }

  .memo-textarea:focus {
    outline: none;
    border-color: #007acc;
    box-shadow: 0 0 0 2px rgba(0, 122, 204, 0.2);
  }

  .memo-dialog-buttons {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  .memo-dialog-buttons button {
    padding: 8px 16px;
    border: 1px solid #ddd;
    border-radius: 6px;
    font-size: 13px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .cancel-btn {
    background: white;
    color: #666;
  }

  .cancel-btn:hover {
    background: #f5f5f5;
    border-color: #ccc;
  }

  .add-btn {
    background: #007acc;
    color: white;
    border-color: #007acc;
  }

  .add-btn:hover:not(:disabled) {
    background: #005a9e;
    border-color: #005a9e;
  }

  .add-btn:disabled {
    background: #ccc;
    border-color: #ccc;
    cursor: not-allowed;
  }

  .memo-dialog-hint {
    text-align: center;
    margin-top: 12px;
  }

  .memo-dialog-hint small {
    color: #999;
    font-size: 11px;
  }


</style>
