<script setup lang="ts">
import { RouterView } from "vue-router";
</script>

<template>
  <RouterView />
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  --color-bg: #ffffff;
  --color-bg-secondary: #f9fafb;
  --color-border: #e5e7eb;
  --color-text-primary: #1f2937;
  --color-text-secondary: #374151;
  --color-text-muted: #6b7280;
  --color-text-placeholder: #9ca3af;
  --color-accent: #3b82f6;
  --color-accent-light: #dbeafe;
  --color-danger-light: #fee2e2;
  --font-family: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
}

html, body {
  font-family: var(--font-family);
  font-size: 13px;
  color: var(--color-text-primary);
  background: transparent;
  overflow: hidden;
  user-select: none;
  -webkit-user-select: none;
}
</style>
