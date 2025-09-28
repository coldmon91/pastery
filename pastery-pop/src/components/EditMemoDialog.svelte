<script>
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  export let visible = false;
  export let memoContent = '';

  function handleCancel() {
    dispatch('cancel');
  }

  function handleUpdate() {
    if (!memoContent.trim()) return;
    dispatch('update', memoContent.trim());
  }

  function handleKeydown(event) {
    if (event.key === 'Escape') {
      handleCancel();
    } else if (event.key === 'Enter' && event.ctrlKey) {
      event.preventDefault();
      handleUpdate();
    }
  }

  function handleOverlayClick() {
    handleCancel();
  }

  function handleDialogClick(event) {
    event.stopPropagation();
  }

  // 다이얼로그가 보일 때 텍스트 에어리어에 포커스
  $: if (visible) {
    setTimeout(() => {
      const textarea = document.querySelector('.edit-memo-textarea');
      if (textarea) textarea.focus();
    }, 100);
  }
</script>

<!-- 메모 편집 다이얼로그 -->
{#if visible}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="memo-dialog-overlay" onclick={handleOverlayClick} role="dialog" aria-label="Edit memo dialog" tabindex="0">
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="memo-dialog" onclick={handleDialogClick}>
      <h3>Edit User Memo</h3>
      <textarea 
        class="memo-textarea edit-memo-textarea" 
        bind:value={memoContent} 
        placeholder="Enter your memo content..."
        onkeydown={handleKeydown}
        rows="4"
      ></textarea>
      <div class="memo-dialog-buttons">
        <button onclick={handleCancel} class="cancel-btn">Cancel</button>
        <button onclick={handleUpdate} class="add-btn" disabled={!memoContent.trim()}>Update Memo</button>
      </div>
      <div class="memo-dialog-hint">
        <small>Press Ctrl+Enter to update • ESC to cancel</small>
      </div>
    </div>
  </div>
{/if}

<style>
  /* 메모 다이얼로그 스타일 */
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