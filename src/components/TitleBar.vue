<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { openSettingsWindow, hideTranslateWindow } from "../utils/tauri";

defineProps<{
  title: string;
  showPin?: boolean;
  showSettings?: boolean;
  showClose?: boolean;
  isPinned?: boolean;
}>();

const emit = defineEmits<{
  (e: "toggle-pin"): void;
}>();

function onTitleMouseDown() {
  getCurrentWindow()
    .startDragging()
    .then(() => console.log("[transight] drag started"))
    .catch((e) => console.error("[transight] drag error:", e));
}
</script>

<template>
  <div class="title-bar" @mousedown="onTitleMouseDown">
    <div class="title-left">
      <img class="title-icon" src="/icon.png" alt="" />
      <span class="title-text">{{ title }}</span>
    </div>
    <div class="title-controls" @mousedown.stop>
      <button
        v-if="showPin"
        class="ctrl-btn pin-btn"
        :class="{ active: isPinned }"
        @click="emit('toggle-pin')"
        title="固定窗口"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="12" y1="17" x2="12" y2="22" />
          <path d="M5 17h14v-1.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V6h1a2 2 0 0 0 0-4H8a2 2 0 0 0 0 4h1v4.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24Z" />
        </svg>
      </button>
      <button
        v-if="showSettings"
        class="ctrl-btn settings-btn"
        @click="openSettingsWindow"
        title="设置"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" />
          <circle cx="12" cy="12" r="3" />
        </svg>
      </button>
      <button
        v-if="showClose"
        class="ctrl-btn close-btn"
        @click="hideTranslateWindow"
        title="关闭"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M18 6 6 18M6 6l12 12" />
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.title-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 4px 0 4px;
  height: 32px;
  width: 100%;
  flex-shrink: 0;
  cursor: grab;
}

.title-bar:active {
  cursor: grabbing;
}

.title-left {
  display: flex;
  align-items: center;
  gap: 10px;
  flex: 1;
  height: 100%;
}

.title-icon {
  width: 20px;
  height: 20px;
  border-radius: 5px;
  flex-shrink: 0;
}

.title-text {
  font-size: var(--text-md);
  font-weight: 600;
  color: var(--color-text-primary);
  white-space: nowrap;
  pointer-events: none;
  user-select: none;
}

.title-controls {
  display: flex;
  align-items: center;
  gap: 4px;
  flex-shrink: 0;
}

.ctrl-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  color: var(--color-text-muted);
  background: var(--color-bg-secondary);
  transition: background 0.15s;
}

.ctrl-btn:hover {
  background: #e5e7eb;
}

.ctrl-btn.active {
  background: #dbeafe;
  color: #3b82f6;
}

.close-btn:hover {
  background: #fee2e2;
  color: #ef4444;
}
</style>
