<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import {
    isPermissionGranted,
    requestPermission,
    sendNotification,
  } from "@tauri-apps/plugin-notification";

  // Dark mode state
  let isDarkMode = $state(false);

  let permissionGranted = $state(false);
  let scheduledTime = $state("");
  let messageText = $state("");
  /**
   * @type {any[]}
   */
  let scheduledMessages = $state([]);

  // Toggle dark mode
  function toggleDarkMode() {
    isDarkMode = !isDarkMode;
    document.documentElement.classList.toggle('dark-mode', isDarkMode);
  }

  listen("notification_", (event) => {
    console.log(`${event.payload.message}`);
    if (permissionGranted) {
      sendNotification({ title: "Tauri", body: `${event.payload.message}` });
      deleteMessage(event.payload.id);
    }
  });

  async function scheduleMessage() {
    if (!scheduledTime || !messageText) {
      alert("請填寫時間和訊息");
      return;
    }

    try {
      const newMessage = {
        time: scheduledTime,
        message: messageText,
        id: Date.now(),
      };

      await invoke("schedule_notification", {
        message: newMessage,
      });

      scheduledMessages = [...scheduledMessages, newMessage];

      // 重置輸入
      scheduledTime = "";
      messageText = "";
    } catch (error) {
      console.error("排程錯誤:", error);
    }
  }

  /**
   * @param {any} id
   */
  async function deleteMessage(id) {
    try {
      await invoke("cancel_notification", { id });
      scheduledMessages = scheduledMessages.filter((msg) => msg.id !== id);
    } catch (error) {
      console.error("刪除錯誤:", error);
    }
  }

  async function permissionCheck() {
    permissionGranted = await isPermissionGranted();

    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === "granted";
    }
  }

  onMount(() => {
    permissionCheck();
    // Check system preference for dark mode
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
      toggleDarkMode();
    }
  });
</script>

<main class="app-container">
  <div class="scheduler-wrapper">
    <div class="header">
      <h1>訊息排程器</h1>
      <button class="theme-toggle" onclick={toggleDarkMode}>
        {#if isDarkMode}
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m3.343-3.657l-.707-.707m12.728 12.728l-.707.707M12 8a4 4 0 100 8 4 4 0 000-8z" />
          </svg>
        {:else}
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
          </svg>
        {/if}
      </button>
    </div>

    <div class="scheduler-form">
      <input
        type="datetime-local"
        bind:value={scheduledTime}
        placeholder="選擇時間"
      />

      <textarea 
        bind:value={messageText} 
        placeholder="輸入通知訊息"
      ></textarea>

      <button onclick={scheduleMessage}>排程通知</button>
    </div>

    <div class="scheduled-messages">
      <h2>已排程通知</h2>
      {#if scheduledMessages.length === 0}
        <p class="empty-state">尚無排程通知</p>
      {:else}
        {#each scheduledMessages as msg (msg.id)}
          <div class="message-item">
            <div class="message-details">
              <span class="message-time">{msg.time.replace("T", " ")}</span>
              <span class="message-text">{msg.message}</span>
            </div>
            <button onclick={() => deleteMessage(msg.id)}>刪除</button>
          </div>
        {/each}
      {/if}
    </div>
  </div>
</main>

<style>
  /* 全域樣式和變數 */
  :root {
    --bg-primary: #f4f4f4;
    --bg-secondary: #ffffff;
    --text-primary: #333333;
    --text-secondary: #666666;
    --accent-color: #3b82f6;
    --delete-color: #ef4444;
    --border-color: #e0e0e0;

    color-scheme: light dark;
  }

  :root.dark-mode {
    --bg-primary: #121212;
    --bg-secondary: #1e1e1e;
    --text-primary: #ffffff;
    --text-secondary: #b0b0b0;
    --accent-color: #60a5fa;
    --delete-color: #f87171;
    --border-color: #444444;

    color-scheme: dark;
  }

  /* 重置和基本設置 */
  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
    transition: background-color 0.3s, color 0.3s;
  }

  /* 主容器樣式 */
  .app-container {
    min-height: 100vh;
    background-color: var(--bg-primary);
    color: var(--text-primary);
    display: flex;
    justify-content: center;
    padding: 2rem;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }

  .scheduler-wrapper {
    width: 100%;
    max-width: 500px;
    background-color: var(--bg-secondary);
    border-radius: 12px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    padding: 1.5rem;
  }

  /* 標題和主題切換按鈕 */
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .header h1 {
    font-size: 1.5rem;
    font-weight: bold;
  }

  .theme-toggle {
    background: transparent;
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .theme-toggle svg {
    width: 1.5rem;
    height: 1.5rem;
    stroke: var(--text-primary);
  }

  /* 表單樣式 */
  .scheduler-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .scheduler-form input,
  .scheduler-form textarea {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid var(--border-color);
    border-radius: 8px;
    background-color: var(--bg-primary);
    color: var(--text-primary);
  }

  .scheduler-form textarea {
    resize: vertical;
    min-height: 100px;
  }

  .scheduler-form button {
    background-color: var(--accent-color);
    color: white;
    border: none;
    padding: 0.75rem;
    border-radius: 8px;
    cursor: pointer;
    transition: background-color 0.3s;
  }

  .scheduler-form button:hover {
    opacity: 0.9;
  }

  /* 已排程訊息樣式 */
  .scheduled-messages h2 {
    font-size: 1.25rem;
    margin-bottom: 1rem;
  }

  .empty-state {
    text-align: center;
    color: var(--text-secondary);
    padding: 1rem;
  }

  .message-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: var(--bg-primary);
    border-radius: 8px;
    padding: 1rem;
    margin-bottom: 0.75rem;
  }

  .message-details {
    flex-grow: 1;
    margin-right: 1rem;
  }

  .message-time {
    display: block;
    font-weight: bold;
    margin-bottom: 0.25rem;
  }

  .message-text {
    color: var(--text-secondary);
    font-size: 0.875rem;
  }

  .message-item button {
    background-color: var(--delete-color);
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    cursor: pointer;
  }

  .message-item button:hover {
    opacity: 0.9;
  }
</style>