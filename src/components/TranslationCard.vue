<script setup lang="ts">
import type { TranslationResult } from "../types";

defineProps<{
  result: TranslationResult;
  index: number;
}>();

const emit = defineEmits<{
  (e: "copy", text: string): void;
}>();
</script>

<template>
  <div class="card" :class="{ 'card-error': result.error }">
    <div class="card-header">
      <div class="provider-info">
        <div class="provider-icon" :class="{ 'icon-error': result.error }" />
        <span class="provider-name">{{ result.provider }}</span>
        <span v-if="result.error" class="error-badge">错误</span>
      </div>
      <button
        v-if="!result.error"
        class="copy-btn"
        @click="emit('copy', result.translated_text)"
        title="复制"
      >
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="9" y="9" width="13" height="13" rx="2" />
          <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
        </svg>
      </button>
    </div>
    <p v-if="result.error" class="card-error-text">{{ result.error }}</p>
    <p v-else class="card-text">{{ result.translated_text }}</p>
    <div class="card-actions">
      <span class="lang-tag">{{ result.source_lang }} → {{ result.target_lang }}</span>
    </div>
  </div>
</template>

<style scoped>
.card {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 10px;
  background: #ffffff;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  width: 100%;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.provider-info {
  display: flex;
  align-items: center;
  gap: 6px;
}

.provider-icon {
  width: 16px;
  height: 16px;
  border-radius: 4px;
  background: var(--color-accent-light);
  flex-shrink: 0;
}

.provider-name {
  font-size: 11px;
  font-weight: 600;
  color: var(--color-text-secondary);
}

.copy-btn {
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

.card-error {
  border-color: #fecaca;
  background: #fef2f2;
}

.icon-error {
  background: #fecaca;
}

.error-badge {
  font-size: 10px;
  font-weight: 500;
  color: #dc2626;
  background: #fee2e2;
  padding: 0 6px;
  border-radius: 4px;
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
