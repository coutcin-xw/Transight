<script setup lang="ts">
import { ref } from "vue";
import { marked } from "marked";
import type { TranslationResult } from "../types";

defineProps<{
  result: TranslationResult;
  index: number;
  loading?: boolean;
}>();

const emit = defineEmits<{
  (e: "copy", text: string): void;
}>();

const collapsed = ref(false);
const useMarkdown = ref(false);

function toggleCollapse() {
  collapsed.value = !collapsed.value;
}

function renderMd(text: string): string {
  if (!useMarkdown.value || !text) return "";
  try {
    return marked.parse(text) as string;
  } catch {
    return text;
  }
}
</script>

<template>
  <div class="card" :class="{ 'card-error': result.error, 'card-loading': loading }">
    <div class="card-header" @click="toggleCollapse">
      <div class="provider-info">
        <div
          class="provider-icon"
          :class="{
            'icon-error': result.error,
            'icon-loading': loading && !result.error && !result.translated_text,
          }"
        />
        <span class="provider-name">{{ result.provider || (loading ? '加载中...' : '') }}</span>
        <span v-if="result.error" class="error-badge">错误</span>
        <span v-else-if="loading && !result.translated_text" class="loading-badge">翻译中</span>
      </div>
      <div class="header-actions">
        <button
          v-if="!result.error && result.translated_text"
          class="md-toggle"
          :class="{ active: useMarkdown }"
          @click.stop="useMarkdown = !useMarkdown"
          title="Markdown 渲染"
        >M↓</button>
        <button
          v-if="!result.error && result.translated_text"
          class="copy-btn"
          @click.stop="emit('copy', result.translated_text)"
          title="复制"
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="9" y="9" width="13" height="13" rx="2" />
            <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
          </svg>
        </button>
        <button class="collapse-btn" :class="{ rotated: !collapsed }" title="折叠">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="m6 9 6 6 6-6" />
          </svg>
        </button>
      </div>
    </div>

    <div v-if="!collapsed" class="card-body">
      <div v-if="loading && !result.translated_text && !result.error" class="loading-line">
        <span class="loading-dot" />
        <span class="loading-dot" />
        <span class="loading-dot" />
      </div>
      <div v-else-if="result.error" class="card-error-text">{{ result.error }}</div>
      <div
        v-else-if="useMarkdown && result.translated_text"
        class="card-text markdown-body"
        v-html="renderMd(result.translated_text)"
      />
      <p v-else class="card-text">{{ result.translated_text }}</p>
      <div v-if="result.source_lang" class="card-actions">
        <span class="lang-tag">{{ result.source_lang }} → {{ result.target_lang }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.card {
  display: flex;
  flex-direction: column;
  gap: 0;
  background: var(--color-card-bg);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  width: 100%;
  overflow: hidden;
  flex-shrink: 0;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 10px;
  cursor: pointer;
  user-select: none;
  transition: background 0.1s;
}

.card-header:hover {
  background: var(--color-bg-secondary);
}

.provider-info {
  display: flex;
  align-items: center;
  gap: 6px;
  min-width: 0;
}

.provider-icon {
  width: 14px;
  height: 14px;
  border-radius: 4px;
  background: var(--color-accent-light);
  flex-shrink: 0;
}

.icon-error {
  background: #fecaca;
}

.icon-loading {
  background: var(--color-border);
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

.provider-name {
  font-size: 11px;
  font-weight: 600;
  color: var(--color-text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 4px;
  flex-shrink: 0;
}

.md-toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 26px; height: 22px;
  border: 1px solid var(--color-border);
  border-radius: 4px;
  background: var(--color-bg-secondary);
  cursor: pointer;
  color: var(--color-text-placeholder);
  font-size: 10px;
  font-weight: 700;
  transition: all 0.15s;
}

.md-toggle.active {
  background: var(--color-accent-light);
  color: var(--color-accent);
  border-color: var(--color-accent);
}

.copy-btn,
.collapse-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
  border: 1px solid var(--color-border);
  border-radius: 4px;
  background: var(--color-bg-secondary);
  cursor: pointer;
  color: var(--color-text-muted);
  transition: all 0.15s;
}

.copy-btn:hover {
  background: var(--color-accent-light);
  color: var(--color-accent);
}

.collapse-btn {
  transition: transform 0.2s;
}

.collapse-btn.rotated {
  transform: rotate(-90deg);
}

/* 展开内容 */
.card-body {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 0 10px 10px;
}

.card-text {
  font-size: 13px;
  line-height: 1.5;
  color: var(--color-text-primary);
  word-break: break-word;
  white-space: pre-wrap;
  max-height: 120px;
  overflow-y: auto;
  padding-right: 4px;
}

.card-text::-webkit-scrollbar {
  width: 3px;
}

.card-text::-webkit-scrollbar-thumb {
  background: var(--color-border); border-radius: 2px;
}

/* markdown 渲染 */
.markdown-body {
  white-space: normal;
}
.markdown-body :deep(p) { margin: 0 0 4px; }
.markdown-body :deep(p:last-child) { margin-bottom: 0; }
.markdown-body :deep(code) {
  background: var(--color-bg-secondary);
  padding: 1px 4px;
  border-radius: 3px;
  font-size: 11px;
}
.markdown-body :deep(pre) {
  background: var(--color-bg-secondary);
  padding: 6px 8px;
  border-radius: 4px;
  overflow-x: auto;
  font-size: 11px;
}
.markdown-body :deep(pre code) {
  background: none;
  padding: 0;
}
.markdown-body :deep(ul), .markdown-body :deep(ol) {
  padding-left: 16px;
  margin: 2px 0;
}
.markdown-body :deep(strong) { font-weight: 600; }
.markdown-body :deep(em) { font-style: italic; }

.card-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 8px;
}

.loading-badge {
  font-size: 10px;
  font-weight: 500;
  color: var(--color-text-muted);
  background: var(--color-bg-secondary);
  padding: 0 6px;
  border-radius: 4px;
}

.loading-line {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 8px 0;
}

.loading-dot {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: var(--color-border);
  animation: dotPulse 1.4s ease-in-out infinite;
}

.loading-dot:nth-child(2) { animation-delay: 0.2s; }
.loading-dot:nth-child(3) { animation-delay: 0.4s; }

@keyframes dotPulse {
  0%, 80%, 100% { opacity: 0.3; }
  40% { opacity: 0.7; }
}

.error-badge {
  font-size: 10px;
  font-weight: 500;
  color: #dc2626;
  background: #fee2e2;
  padding: 0 6px;
  border-radius: 4px;
}

.card-error {
  border-color: #fecaca;
}

.card-loading {
  border-color: var(--color-border);
  opacity: 0.8;
}

.card-error-text {
  font-size: 12px;
  line-height: 1.5;
  color: #dc2626;
  word-break: break-word;
  white-space: pre-wrap;
  padding: 6px 8px;
  background: #fff5f5;
  border-radius: 6px;
  border: 1px solid #fecaca;
  max-height: 120px;
  overflow-y: auto;
}

.lang-tag {
  font-size: 10px;
  color: var(--color-text-placeholder);
  background: var(--color-bg-secondary);
  padding: 1px 6px;
  border-radius: 4px;
}
</style>
