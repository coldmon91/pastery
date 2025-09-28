<script>
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  export let visible = false;
  export let position = { x: 0, y: 0 };
  export let selectedItem = null;

  function handleEdit() {
    dispatch('edit', selectedItem);
  }

  function handleDelete() {
    dispatch('delete', selectedItem);
  }

  function handleClick(event) {
    event.stopPropagation();
  }
</script>

<!-- 컨텍스트 메뉴 -->
{#if visible}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="context-menu" 
       style="left: {position.x}px; top: {position.y}px;"
       onclick={handleClick}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="context-menu-item" onclick={handleEdit}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
        <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
      </svg>
      Edit
    </div>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="context-menu-item delete" onclick={handleDelete}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
        <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
      </svg>
      Delete
    </div>
  </div>
{/if}

<style>
  /* 컨텍스트 메뉴 스타일 */
  .context-menu {
    position: fixed;
    background: white;
    border: 1px solid #ddd;
    border-radius: 6px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    z-index: 1000;
    min-width: 120px;
    overflow: hidden;
  }

  .context-menu-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    font-size: 13px;
    color: #333;
    cursor: pointer;
    transition: background-color 0.2s ease;
    border: none;
    background: none;
    width: 100%;
    text-align: left;
  }

  .context-menu-item:hover {
    background-color: #f5f5f5;
  }

  .context-menu-item.delete {
    color: #dc3545;
  }

  .context-menu-item.delete:hover {
    background-color: #ffe6e6;
  }

  .context-menu-item svg {
    flex-shrink: 0;
  }
</style>