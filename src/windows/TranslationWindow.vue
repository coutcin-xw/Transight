<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import { useTranslationStore } from "../stores/translation";
import { hideTranslateWindow } from "../utils/tauri";
import { LANGUAGES } from "../types";
import TitleBar from "../components/TitleBar.vue";
import TranslationCard from "../components/TranslationCard.vue";

const store = useTranslationStore();
const inputText = ref("");
const showSourceDropdown = ref(false);
const showTargetDropdown = ref(false);

// 触发翻译
async function handleTranslate() {
  if (!inputText.value.trim()) return;
  store.sourceText = inputText.value;
  await store.doTranslate(inputText.value);
}

// 复制到剪贴板
async function copyText(text: string) {
  try {
    await navigator.clipboard.writeText(text);
  } catch {
    // fallback
  }
}

// 交换语言
function swapLanguages() {
  store.swapLanguages();
}

// 监听后端事件
let unlisten1: (() => void) | null = null;
let unlisten2: (() => void) | null = null;

onMounted(async () => {
  // 选中文本事件
  unlisten1 = await listen<string>("selected-text", (event) => {
    const text = event.payload;
    if (text && text.trim()) {
      inputText.value = text.trim();
      handleTranslate();
    }
  });
  // pin 状态同步 (后端可能通过托盘等改变 pin)
  unlisten2 = await listen<boolean>("pin-changed", (event) => {
    store.setPinned(event.payload);
  });
});

onUnmounted(() => {
  if (unlisten1) unlisten1();
  if (unlisten2) unlisten2();
});

// 监听键盘: Escape 关闭
function onKeydown(e: KeyboardEvent) {
  if (e.key === "Escape") {
    hideTranslateWindow();
  }
  if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
    handleTranslate();
  }
}

document.addEventListener("keydown", onKeydown);
</script>

<template>
  <div class="translate-window" @keydown="onKeydown" @contextmenu.prevent>
    <!-- 标题栏 -->
    <TitleBar
      title="Transight"
      :show-pin="true"
      :show-settings="true"
      :show-close="true"
      :is-pinned="store.isPinned"
      @toggle-pin="store.togglePin()"
    />

    <!-- 语言选择器 -->
    <div class="language-selector">
      <!-- 源语言 -->
      <div class="lang-dropdown" @click="showSourceDropdown = !showSourceDropdown; showTargetDropdown = false">
        <span class="lang-value">{{ LANGUAGES[store.sourceLang] || store.sourceLang }}</span>
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m6 9 6 6 6-6"/></svg>
        <div v-if="showSourceDropdown" class="dropdown-menu">
          <div
            v-for="(label, code) in LANGUAGES"
            :key="code"
            class="dropdown-item"
            :class="{ selected: store.sourceLang === code }"
            @click.stop="store.setSourceLang(code); showSourceDropdown = false"
          >{{ label }}</div>
        </div>
      </div>

      <!-- 交换按钮 (左右交换图标) -->
      <button class="swap-btn" @click="swapLanguages" :disabled="store.sourceLang === 'auto'">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M17 3l4 4-4 4" />
          <path d="M21 7H9a2 2 0 0 0-2 2v2" />
          <path d="M7 21l-4-4 4-4" />
          <path d="M3 17h12a2 2 0 0 0 2-2v-2" />
        </svg>
      </button>

      <!-- 目标语言 -->
      <div class="lang-dropdown" @click="showTargetDropdown = !showTargetDropdown; showSourceDropdown = false">
        <span class="lang-value">{{ LANGUAGES[store.targetLang] || store.targetLang }}</span>
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m6 9 6 6 6-6"/></svg>
        <div v-if="showTargetDropdown" class="dropdown-menu">
          <div
            v-for="(label, code) in LANGUAGES"
            :key="code"
            class="dropdown-item"
            :class="{ selected: store.targetLang === code }"
            @click.stop="store.setTargetLang(code); showTargetDropdown = false"
          >{{ label }}</div>
        </div>
      </div>

      <!-- Go 按钮 (翻译中也可点击，打断并重试) -->
      <button class="go-btn" @click="handleTranslate" :disabled="!inputText.trim()">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M5 12h14M12 5l7 7-7 7" />
        </svg>
      </button>
    </div>

    <!-- 输入区域 -->
    <div class="input-section">
      <textarea
        v-model="inputText"
        class="input-area"
        placeholder="在此输入或选中文本..."
      />
    </div>

    <!-- 翻译结果 -->
    <div class="results-section">
      <div class="results-header">
        <span class="results-label">翻译结果</span>
        <span v-if="store.resultCount > 0" class="results-count">
          {{ store.resultCount }} 个结果
        </span>
      </div>

      <div v-if="store.hasResults" class="results-list">
        <TranslationCard
          v-for="(result, idx) in store.results"
          :key="result.provider || idx"
          :result="result"
          :index="idx"
          :loading="!result.translated_text && !result.error"
          @copy="copyText"
        />
      </div>

      <div v-else class="empty-state">
        <p>暂无翻译结果</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.translate-window {
  display: flex;
  flex-direction: column;
  height: 100vh;
  padding: 10px 14px 14px;
  gap: 8px;
  background: #ffffff;
  border-radius: 12px;
  border: 1px solid var(--color-border);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

/* 语言选择器 */
.language-selector {
  display: flex;
  align-items: center;
  gap: 6px;
  width: 100%;
  flex-shrink: 0;
}

.lang-dropdown {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  height: 28px;
  padding: 0 10px;
  border-radius: 8px;
  border: 1px solid var(--color-border);
  background: var(--color-bg-secondary);
  cursor: pointer;
  font-size: 11px;
  color: var(--color-text-secondary);
  transition: border-color 0.15s;
}

.lang-dropdown:hover {
  border-color: var(--color-accent);
}

.lang-value {
  white-space: nowrap;
}

.swap-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  background: #f3f4f6;
  cursor: pointer;
  color: #6b7280;
  flex-shrink: 0;
  transition: all 0.15s;
}

.swap-btn:hover:not(:disabled) {
  background: var(--color-accent-light);
  color: var(--color-accent);
}

.swap-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.go-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: 1px solid var(--color-accent);
  border-radius: 6px;
  background: var(--color-accent);
  cursor: pointer;
  color: #ffffff;
  flex-shrink: 0;
  margin-left: auto;
  transition: all 0.15s;
}

.go-btn:hover:not(:disabled) {
  background: #2563eb;
}

.go-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

/* 语言下拉菜单 */
.lang-dropdown {
  position: relative;
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  left: 0;
  margin-top: 4px;
  min-width: 120px;
  max-height: 200px;
  overflow-y: auto;
  background: #ffffff;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  z-index: 100;
}

.dropdown-item {
  padding: 6px 12px;
  font-size: 11px;
  color: var(--color-text-secondary);
  cursor: pointer;
  white-space: nowrap;
  transition: background 0.1s;
}

.dropdown-item:hover {
  background: #f3f4f6;
}

.dropdown-item.selected {
  background: var(--color-accent-light);
  color: var(--color-accent);
  font-weight: 600;
}

.dropdown-menu::-webkit-scrollbar {
  width: 4px;
}

.dropdown-menu::-webkit-scrollbar-thumb {
  background: #e5e7eb;
  border-radius: 2px;
}

/* 输入区域 */
.input-section {
  flex-shrink: 0;
}

.input-area {
  width: 100%;
  height: 72px;
  padding: 10px;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  background: var(--color-bg-secondary);
  font-family: var(--font-family);
  font-size: 12px;
  color: var(--color-text-primary);
  resize: none;
  outline: none;
  transition: border-color 0.15s;
}

.input-area::placeholder {
  color: var(--color-text-placeholder);
}

.input-area:focus {
  border-color: var(--color-accent);
}

/* 结果区域 */
.results-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 6px;
  overflow: hidden;
  min-height: 0;
}

.results-header {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.results-label {
  font-size: 12px;
  font-weight: 500;
  color: var(--color-text-secondary);
}

.results-count {
  font-size: 10px;
  font-weight: 500;
  color: #1e40af;
  background: #dbeafe;
  padding: 1px 8px;
  border-radius: 12px;
}

.results-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
  overflow-y: auto;
  flex: 1;
  min-height: 0;
}

.results-list::-webkit-scrollbar {
  width: 4px;
}

.results-list::-webkit-scrollbar-thumb {
  background: #e5e7eb;
  border-radius: 2px;
}

.empty-state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-muted);
  font-size: 12px;
}

</style>
