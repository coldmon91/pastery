<script>
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from 'svelte';

  let clipboardItems = $state([]);
  let loading = $state(false);
  let error = $state('');

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

  async function selectItem(item) {
    // 클립보드에 내용 복사 (브라우저 API 사용)
    try {
      await navigator.clipboard.writeText(item.content);
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

  onMount(() => {
    loadClipboardItems();
    window.addEventListener('keydown', handleEscape);
    
    // 클립보드 갱신 이벤트 리스너 등록
    const unlistenRefresh = listen('refresh-clipboard', () => {
      loadClipboardItems();
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
        <button onclick={loadClipboardItems}>Retry</button>
      </div>
    {:else if clipboardItems.length === 0}
      <div class="empty">No clipboard items found</div>
    {:else}
      <div class="clipboard-list">
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
      </div>
    {/if}
  </div>
  
  <div class="footer">
    <small>Press ESC to close • Click item to copy</small>
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    background: transparent;
  }

  .popup-container {
    width: 350px;
    height: 450px;
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(10px);
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.3);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: rgba(0, 0, 0, 0.05);
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  }

  .header h2 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
    color: #333;
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 20px;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
    color: #666;
    line-height: 1;
  }

  .close-btn:hover {
    background: rgba(0, 0, 0, 0.1);
    color: #333;
  }

  .content {
    flex: 1;
    padding: 8px;
    overflow-y: auto;
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

  .item-number {
    background: #007acc;
    color: white;
    width: 24px;
    height: 24px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    font-weight: 600;
    flex-shrink: 0;
    margin-top: 2px;
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

  .item-date {
    font-size: 11px;
    color: #999;
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

  /* 스크롤바 스타일링 */
  .content::-webkit-scrollbar {
    width: 6px;
  }

  .content::-webkit-scrollbar-track {
    background: transparent;
  }

  .content::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 3px;
  }

  .content::-webkit-scrollbar-thumb:hover {
    background: rgba(0, 0, 0, 0.3);
  }
</style>
