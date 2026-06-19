<script setup lang="ts">
import { ref } from "vue";
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

function toggleCollapse() {
  collapsed.value = !collapsed.value;
}
</script>

<template>
  <div class="card" :class="{ 'card-error': result.error, 'card-loading': loading }">
    <!-- 头部始终可见 (折叠后只显示这行) -->
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

    <!-- 展开内容 -->
    <div v-if="!collapsed" class="card-body">
      <div v-if="loading && !result.translated_text && !result.error" class="loading-line">
        <span class="loading-dot" />
        <span class="loading-dot" />
        <span class="loading-dot" />
      </div>
      <p v-else-if="result.error" class="card-error-text">{{ result.error }}</p>
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
  background: #ffffff;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  width: 100%;
  overflow: hidden;
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
  background: #f9fafb;
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
  background: #fef3c7;
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.4; }
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

.copy-btn,
.collapse-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
  border: 1px solid var(--color-border);
  border-radius: 4px;
  background: #f9fafb;
  cursor: pointer;
  color: #6b7280;
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
  color: #111827;
  word-break: break-word;
}

.card-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 8px;
}

.loading-badge {
  font-size: 10px;
  font-weight: 500;
  color: #92400e;
  background: #fef3c7;
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
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: #d1d5db;
  animation: dotPulse 1.4s ease-in-out infinite;
}

.loading-dot:nth-child(2) { animation-delay: 0.2s; }
.loading-dot:nth-child(3) { animation-delay: 0.4s; }

@keyframes dotPulse {
  0%, 80%, 100% { opacity: 0.3; transform: scale(0.8); }
  40% { opacity: 1; transform: scale(1); }
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
  border-color: #fde68a;
}

.card-error-text {
  font-size: 12px;
  line-height: 1.5;
  color: #dc2626;
  word-break: break-word;
  padding: 6px 8px;
  background: #fff5f5;
  border-radius: 6px;
  border: 1px solid #fecaca;
}

.lang-tag {
  font-size: 10px;
  color: #9ca3af;
  background: #f3f4f6;
  padding: 1px 6px;
  border-radius: 4px;
}
</style>
