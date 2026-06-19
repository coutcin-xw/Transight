<script setup lang="ts">
import { ref } from "vue";
import { useTranslationStore } from "../stores/translation";
import { getSelectedText, hideTranslateWindow } from "../utils/tauri";
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

// 从选中文本初始化
async function initFromSelection() {
  try {
    const text = await getSelectedText();
    if (text) {
      inputText.value = text;
      await handleTranslate();
    }
  } catch {
    // 获取选中文本失败，静默处理
  }
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

// 监听键盘
function onKeydown(e: KeyboardEvent) {
  if (e.key === "Escape") {
    hideTranslateWindow();
  }
  if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
    handleTranslate();
  }
}

// 初始化
initFromSelection();
document.addEventListener("keydown", onKeydown);
</script>

<template>
  <div class="translate-window" @keydown="onKeydown">
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
      <div class="lang-dropdown" @click="showSourceDropdown = !showSourceDropdown">
        <span class="lang-value">{{ LANGUAGES[store.sourceLang] || store.sourceLang }}</span>
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m6 9 6 6 6-6"/></svg>
      </div>

      <!-- 交换按钮 -->
      <button class="swap-btn" @click="swapLanguages" :disabled="store.sourceLang === 'auto'">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M7 16V4m0 0L3 8m4-4 4 4M17 8v12m0 0 4-4m-4 4-4-4" />
        </svg>
      </button>

      <!-- 目标语言 -->
      <div class="lang-dropdown" @click="showTargetDropdown = !showTargetDropdown">
        <span class="lang-value">{{ LANGUAGES[store.targetLang] || store.targetLang }}</span>
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m6 9 6 6 6-6"/></svg>
      </div>
    </div>

    <!-- 输入区域 -->
    <div class="input-section">
      <textarea
        v-model="inputText"
        class="input-area"
        placeholder="在此输入或选中文本..."
        @keydown.enter.exact="handleTranslate"
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

      <div v-if="store.isLoading" class="loading">
        <span>翻译中...</span>
      </div>

      <div v-else-if="store.error" class="error">
        <span>{{ store.error }}</span>
      </div>

      <div v-else-if="store.hasResults" class="results-list">
        <TranslationCard
          v-for="(result, idx) in store.results"
          :key="idx"
          :result="result"
          :index="idx"
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

.loading,
.error,
.empty-state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-muted);
  font-size: 12px;
}

.error {
  color: #ef4444;
}
</style>
